use async_trait::async_trait;

use service::messaging::{MessageBus, MessageExchange, MessageExchangeType};
use std::{
  collections::{hash_map::Entry, HashMap},
  sync::Arc
};
use tokio::sync::Mutex;

use super::rmq_message_exchange::RmqMessageExchange;

pub struct RabbitMqMessageBusBackend {
  channel:   Arc<lapin::Channel>,
  exchanges: HashMap<String, Arc<Mutex<Box<dyn MessageExchange + Send>>>>
}

#[async_trait]
impl MessageBus for RabbitMqMessageBusBackend {
  async fn exchange(
    &mut self,
    name: &str,
    exchange_type: MessageExchangeType
  ) -> &Arc<Mutex<Box<dyn MessageExchange + Send + 'static>>> {
    let exchange = match self.exchanges.entry(name.to_owned()) {
      Entry::Occupied(o) => o.into_mut(),
      Entry::Vacant(v) => {
        v.insert(Arc::new(Mutex::new(Box::new(
          RmqMessageExchange::new(self.channel.clone(), name.to_owned(), exchange_type).await
        ))))
      }
    };

    exchange
  }
}
