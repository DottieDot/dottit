use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::service_mediator::MediatorQuery;

#[derive(Serialize, Deserialize)]
pub struct CreateApiTokenForUserQuery {
  pub user_id: Uuid
}

#[derive(Serialize, Deserialize)]
pub struct ApiTokenForUser {
  pub user_id: Uuid,
  pub token:   String
}

impl MediatorQuery for CreateApiTokenForUserQuery {
  type Response = ApiTokenForUser;

  fn name() -> &'static str {
    "CreateApiTokenForUserQuery"
  }
}
