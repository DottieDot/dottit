use serde::{Deserialize, Serialize};

use crate::messaging::{Event, EventMetadata, QueueMetadata};

use super::THREAD_EXCHANGE;

#[derive(Serialize, Deserialize)]
pub struct ThreadDeletedEvent {
  pub thread_id: String
}

impl Event for ThreadDeletedEvent {
  fn metadata() -> EventMetadata {
    EventMetadata {
      exchange:        THREAD_EXCHANGE,
      queue:           QueueMetadata {
        routing_key:  "threads.deleted",
        options:      Default::default(),
        bind_options: Default::default()
      },
      consume_options: Default::default()
    }
  }
}
