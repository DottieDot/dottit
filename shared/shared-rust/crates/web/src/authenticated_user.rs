use std::str::FromStr;

use axum::http::Request;
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthenticatedUser {
  pub token:   String,
  pub user_id: Uuid
}

impl AuthenticatedUser {
  pub fn from_http_request<B>(request: &Request<B>) -> Option<Self> {
    let headers = request.headers();
    let user_id = headers
      .get("dottit-user-id")
      .and_then(|value| value.to_str().ok())
      .and_then(|value| Uuid::from_str(value).ok())?;
    let token = headers
      .get("dottit-api-token")
      .and_then(|value| value.to_str().ok())
      .map(|value| value.to_owned())?;

    Some(Self { user_id, token })
  }
}
