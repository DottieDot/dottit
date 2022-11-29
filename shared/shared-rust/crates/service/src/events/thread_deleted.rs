use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::messaging::{Event, EventMetadata, QueueMetadata};

use super::THREAD_EXCHANGE;

#[derive(Serialize, Deserialize)]
pub struct ThreadDeletedEvent {
  pub thread_id: Uuid
}

impl Event for ThreadDeletedEvent {
  fn metadata() -> EventMetadata<'static> {
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
