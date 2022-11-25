use serde::{Deserialize, Serialize};

use crate::service_mediator::MediatorQuery;

#[derive(Serialize, Deserialize)]
pub struct CreateApiTokenForUserQuery {
  pub user_id: String
}

#[derive(Serialize, Deserialize)]
pub struct ApiTokenForUser {
  pub user_id: String,
  pub token:   String
}

impl MediatorQuery for CreateApiTokenForUserQuery {
  type Response = ApiTokenForUser;

  fn name() -> &'static str {
    "CreateApiTokenForUserQuery"
  }
}
