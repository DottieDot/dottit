use std::{future::Future, pin::Pin};

use anyhow::Result;
use async_trait::async_trait;

use super::EventMetadata;

pub type RawEventHandler =
  Box<dyn Fn(Vec<u8>) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send>;

#[async_trait]
pub trait EventBusBackend {
  async fn subscribe<'a>(
    &mut self,
    queue_name: String,
    metadata: EventMetadata<'a>,
    handler: RawEventHandler
  );

  async fn publish<'a>(&mut self, metadata: EventMetadata<'a>, data: &[u8]);
}
