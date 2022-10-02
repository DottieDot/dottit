use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use super::message_exchange::{MessageExchange, MessageExchangeType};

#[async_trait]
pub trait MessageBus: Send + Sync {
  async fn exchange(
    &mut self,
    name: &str,
    exchange_type: MessageExchangeType
  ) -> &Arc<Mutex<Box<dyn MessageExchange + Send + 'static>>>;
}
