use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::service_mediator::MediatorQuery;

use super::{EntityExistsQuery, EntityExistsQueryResponse};

#[derive(Serialize, Deserialize)]
pub struct UserExistsQuery {
  pub user_id: Uuid
}

#[derive(Serialize, Deserialize)]
pub struct UserExistsQueryResponse {
  pub exists: bool
}

impl EntityExistsQuery for UserExistsQuery {
  fn new_exists_query(uuid: Uuid) -> Self {
    Self { user_id: uuid }
  }

  fn entity_name() -> &'static str {
    "user"
  }
}

impl MediatorQuery for UserExistsQuery {
  type Response = UserExistsQueryResponse;

  fn name() -> &'static str {
    "UserExistsQuery"
  }
}

impl EntityExistsQueryResponse for UserExistsQueryResponse {
  fn exists(&self) -> bool {
    self.exists
  }
}
