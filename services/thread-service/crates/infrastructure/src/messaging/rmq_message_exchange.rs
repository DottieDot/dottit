use std::{
  collections::{hash_map::Entry, HashMap},
  sync::Arc
};

use async_trait::async_trait;
use lapin::{options::ExchangeDeclareOptions, types::FieldTable, Channel, ExchangeKind};
use service::messaging::{MessageExchange, MessageExchangeType, MessageQueue};
use tokio::sync::Mutex;

use super::rmq_message_queue_backend::RmqMessageQueueBackend;

pub struct RmqMessageExchange {
  channel:  Arc<Channel>,
  exchange: String,
  queues:   HashMap<String, Arc<Mutex<MessageQueue>>>
}

impl RmqMessageExchange {
  pub async fn new(
    channel: Arc<Channel>,
    exchange: String,
    exchange_type: MessageExchangeType
  ) -> Self {
    channel
      .exchange_declare(
        &exchange,
        rmq_exchange_king_from_exchange_type(exchange_type),
        ExchangeDeclareOptions::default(),
        FieldTable::default()
      )
      .await
      .unwrap();

    Self {
      channel,
      exchange,
      queues: Default::default()
    }
  }
}

#[async_trait]
impl MessageExchange for RmqMessageExchange {
  async fn queue(&mut self, name: &str) -> &Arc<Mutex<MessageQueue>> {
    let queue = match self.queues.entry(name.to_owned()) {
      Entry::Occupied(o) => o.into_mut(),
      Entry::Vacant(v) => {
        v.insert(Arc::new(Mutex::new(MessageQueue::new(Mutex::new(
          Box::new(
            RmqMessageQueueBackend::new(
              self.channel.clone(),
              self.exchange.clone(),
              name.to_owned()
            )
            .await
          )
        )))))
      }
    };

    queue
  }
}

fn rmq_exchange_king_from_exchange_type(exchange_type: MessageExchangeType) -> ExchangeKind {
  match exchange_type {
    MessageExchangeType::Direct => ExchangeKind::Direct,
    MessageExchangeType::Topic => ExchangeKind::Topic,
    MessageExchangeType::Headers => ExchangeKind::Headers,
    MessageExchangeType::FanOut => ExchangeKind::Fanout
  }
}
