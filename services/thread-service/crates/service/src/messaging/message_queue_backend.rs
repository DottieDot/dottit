use std::{future::Future, pin::Pin};

use async_trait::async_trait;

#[async_trait]
pub trait MessageQueueBackend {
  async fn publish(&self, data: &[u8]);

  async fn subscribe(
    &mut self,
    consumer_tag: &str,
    handler: Box<
      dyn Fn(Vec<u8>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync + 'static
    >
  );
}
