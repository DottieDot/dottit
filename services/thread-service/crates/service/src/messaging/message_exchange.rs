use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use super::message_queue::MessageQueue;

#[async_trait]
pub trait MessageExchange {
  async fn queue(&mut self, name: &str) -> &Arc<Mutex<MessageQueue>>;
}

#[derive(Clone, Copy)]
pub enum MessageExchangeType {
  Direct,
  Topic,
  Headers,
  FanOut
}
