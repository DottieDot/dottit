use std::{future::Future, pin::Pin, sync::Arc};

use async_trait::async_trait;
use lapin::{options::BasicAckOptions, Channel};
use service::messaging::MessageQueueBackend;
use tokio::{stream::StreamExt, sync::Mutex};

pub struct RmqMessageQueueBackend {
  channel:   Arc<Channel>,
  exchange:  String,
  queue:     String,
  consumers: Mutex<Vec<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>>
}

impl RmqMessageQueueBackend {
  pub async fn new(channel: Arc<Channel>, exchange: String, queue: String) -> Self {
    channel
      .queue_bind(
        &queue,
        &exchange,
        &queue,
        Default::default(),
        Default::default()
      )
      .await
      .unwrap();

    Self {
      channel,
      exchange,
      queue,
      consumers: Mutex::new(Default::default())
    }
  }
}

#[async_trait]
impl MessageQueueBackend for RmqMessageQueueBackend {
  async fn publish(&self, data: &[u8]) {
    self
      .channel
      .basic_publish(
        &self.exchange,
        &self.queue,
        Default::default(),
        data,
        Default::default()
      )
      .await
      .unwrap();
  }

  async fn subscribe(
    &mut self,
    consumer_tag: &str,
    handler: Box<
      dyn Fn(Vec<u8>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync + 'static
    >
  ) {
    let mut consumer = self
      .channel
      .basic_consume(
        &self.queue,
        consumer_tag,
        Default::default(),
        Default::default()
      )
      .await
      .unwrap();

    let executer = async move {
      while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
          handler(delivery.data.clone()).await;
          if let Err(_) = delivery.ack(BasicAckOptions::default()).await {
            // TODO! Error
          }
        } else {
          // TODO! Error
        }
      }
    };

    self.consumers.lock().await.push(Box::pin(executer));
  }
}
