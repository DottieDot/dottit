use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::service_mediator::MediatorQuery;

use super::{EntityExistsQuery, EntityExistsQueryResponse};

#[derive(Serialize, Deserialize)]
pub struct ThreadExistsQuery {
  pub thread_id: Uuid
}

#[derive(Serialize, Deserialize)]
pub struct ThreadExistsQueryResponse {
  pub exists: bool
}

impl EntityExistsQuery for ThreadExistsQuery {
  fn new_exists_query(uuid: Uuid) -> Self {
    Self { thread_id: uuid }
  }

  fn entity_name() -> &'static str {
    "thread"
  }
}

impl MediatorQuery for ThreadExistsQuery {
  type Response = ThreadExistsQueryResponse;

  fn name() -> &'static str {
    "ThreadExistsQuery"
  }
}

impl EntityExistsQueryResponse for ThreadExistsQueryResponse {
  fn exists(&self) -> bool {
    self.exists
  }
}
