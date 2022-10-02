use std::collections::HashSet;

use async_trait::async_trait;
use lapin::{
  options::{BasicConsumeOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions},
  Channel, Connection, Consumer, ExchangeKind
};
use shared_service::messaging::{
  ConsumeOptions, EventBusBackend, EventExchangeType, EventMetadata, ExchangeMetadata,
  ExchangeOptions, QueueOptions, RawEventHandler
};
use tokio::stream::StreamExt;
use uuid::Uuid;

pub struct RmqEventBusBackend {
  _connection: Connection,
  channel:     Channel,
  exchanges:   HashSet<String>
}

impl RmqEventBusBackend {
  pub async fn connect(uri: &str) -> Self {
    let connection = Connection::connect(uri, Default::default()).await.unwrap();
    let channel = connection.create_channel().await.unwrap();
    Self {
      _connection: connection,
      channel,
      exchanges: Default::default()
    }
  }
}

impl RmqEventBusBackend {
  async fn declare_exchange(&mut self, metadata: ExchangeMetadata) {
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

  async fn declare_queue(&self, name: &str, metadata: EventMetadata) {
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

  async fn bind_queue(&self, name: &str, metadata: EventMetadata) {
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
        &queue_name,
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
impl EventBusBackend for RmqEventBusBackend {
  async fn subscribe(
    &mut self,
    queue_name: String,
    metadata: EventMetadata,
    handler: RawEventHandler
  ) {
    let new_queue_name = &format!("{}.{queue_name}", metadata.exchange.name);

    self.declare_exchange(metadata.exchange).await;
    self.declare_queue(&new_queue_name, metadata).await;
    self.bind_queue(&new_queue_name, metadata).await;

    let mut consumer = self
      .consume(&new_queue_name, metadata.consume_options)
      .await;

    tokio::spawn(async move {
      while let Some(delivery) = consumer.next().await {
        let delivery = delivery.unwrap();
        let acker = delivery.acker;
        let data = delivery.data;

        let fut = handler(data);
        match fut.await {
          Ok(()) => acker.ack(Default::default()).await.unwrap(),
          Err(_) => {}
        }
      }
    });
  }

  async fn publish(&mut self, metadata: EventMetadata, data: &[u8]) {
    self.declare_exchange(metadata.exchange).await;

    self
      .channel
      .basic_publish(
        metadata.exchange.name,
        metadata.queue.routing_key,
        Default::default(),
        &data,
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
