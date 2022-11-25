use std::collections::HashSet;

use async_trait::async_trait;
use lapin::{
  options::{BasicConsumeOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions},
  Channel, Connection, Consumer, ExchangeKind
};
use log::error;
use shared_service::messaging::{
  ConsumeOptions, EventBusBackend, EventExchangeType, EventMetadata, ExchangeMetadata,
  ExchangeOptions, QueueOptions, RawEventHandler
};
use tokio::runtime::Handle;
use tokio_stream::StreamExt;
use uuid::Uuid;

pub struct AmqpEventBusBackend {
  _connection: Connection,
  channel:     Channel,
  exchanges:   HashSet<String> //runtime:     Runtime
}

impl AmqpEventBusBackend {
  pub async fn connect(uri: &str) -> Self {
    let connection = Connection::connect(uri, Default::default()).await.unwrap();
    let channel = connection.create_channel().await.unwrap();
    Self {
      _connection: connection,
      channel,
      exchanges: Default::default() // runtime: Runtime::new().unwrap()
    }
  }
}

impl AmqpEventBusBackend {
  async fn declare_exchange<'a>(&mut self, metadata: ExchangeMetadata<'a>) {
    if self.exchanges.insert(metadata.name.to_owned()) {
      let ExchangeOptions {
        passive,
        durable,
        auto_delete,
        internal,
        no_wait
      } = metadata.options;

      self
        .channel
        .exchange_declare(
          metadata.name,
          rmq_exchange_kind_from_exchange_type(metadata.exchange_type),
          ExchangeDeclareOptions {
            passive,
            durable,
            auto_delete,
            internal,
            nowait: no_wait
          },
          Default::default()
        )
        .await
        .unwrap();
    }
  }

  async fn declare_queue<'a>(&self, name: &str, metadata: EventMetadata<'a>) {
    let QueueOptions {
      passive,
      durable,
      exclusive,
      auto_delete,
      no_wait
    } = metadata.queue.options;

    self
      .channel
      .queue_declare(
        name,
        QueueDeclareOptions {
          passive,
          durable,
          exclusive,
          auto_delete,
          nowait: no_wait
        },
        Default::default()
      )
      .await
      .unwrap();
  }

  async fn bind_queue<'a>(&self, name: &str, metadata: EventMetadata<'a>) {
    self
      .channel
      .queue_bind(
        name,
        metadata.exchange.name,
        metadata.queue.routing_key,
        QueueBindOptions {
          nowait: metadata.queue.bind_options.no_wait
        },
        Default::default()
      )
      .await
      .unwrap();
  }

  async fn consume(
    &self,
    queue_name: &str,
    ConsumeOptions {
      no_local,
      no_ack,
      exclusive,
      no_wait
    }: ConsumeOptions
  ) -> Consumer {
    self
      .channel
      .basic_consume(
        queue_name,
        Uuid::new_v4().to_string().as_str(),
        BasicConsumeOptions {
          no_local,
          no_ack,
          exclusive,
          nowait: no_wait
        },
        Default::default()
      )
      .await
      .unwrap()
  }
}

#[async_trait]
impl EventBusBackend for AmqpEventBusBackend {
  async fn subscribe<'a>(
    &mut self,
    queue_name: String,
    metadata: EventMetadata<'a>,
    handler: RawEventHandler
  ) {
    let new_queue_name = &format!("{}.{queue_name}", metadata.exchange.name);

    self.declare_exchange(metadata.exchange).await;
    self.declare_queue(new_queue_name, metadata).await;
    self.bind_queue(new_queue_name, metadata).await;

    let mut consumer = self.consume(new_queue_name, metadata.consume_options).await;

    Handle::current().spawn(async move {
      while let Some(delivery) = consumer.next().await {
        let delivery = match delivery {
          Ok(d) => d,
          Err(e) => {
            error!("Faulty amqp delivery: {e}");
            continue;
          }
        };
        let acker = delivery.acker;
        let data = delivery.data;

        let fut = handler(data);
        match fut.await {
          Ok(()) => {
            if let Err(e) = acker.ack(Default::default()).await {
              error!("Acknowledgement failed: {e}")
            }
          }
          Err(e) => {
            error!("Error while handling event: {e}")
          }
        }
      }
    });
  }

  async fn publish<'a>(&mut self, metadata: EventMetadata<'a>, data: &[u8]) {
    self.declare_exchange(metadata.exchange).await;

    self
      .channel
      .basic_publish(
        metadata.exchange.name,
        metadata.queue.routing_key,
        Default::default(),
        data,
        Default::default()
      )
      .await
      .unwrap();
  }
}

fn rmq_exchange_kind_from_exchange_type(exchange_type: EventExchangeType) -> ExchangeKind {
  match exchange_type {
    EventExchangeType::Direct => ExchangeKind::Direct,
    EventExchangeType::Topic => ExchangeKind::Topic,
    EventExchangeType::Headers => ExchangeKind::Headers,
    EventExchangeType::FanOut => ExchangeKind::Fanout
  }
}
