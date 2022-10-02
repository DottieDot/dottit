use std::{future::Future, pin::Pin};

use anyhow::Result;
use async_trait::async_trait;

use super::EventMetadata;

pub type RawEventHandler = Box<
  dyn Fn(Vec<u8>) -> Pin<Box<dyn Future<Output = Result<()>> + Send + Sync + 'static>>
    + Send
    + 'static
>;

#[async_trait]
pub trait EventBusBackend {
  async fn subscribe(
    &mut self,
    queue_name: String,
    metadata: EventMetadata,
    handler: RawEventHandler
  );

  async fn publish(&mut self, metadata: EventMetadata, data: &[u8]);
}
