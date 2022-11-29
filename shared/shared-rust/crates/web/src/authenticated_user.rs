use std::str::FromStr;

use axum::http::Request;
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthenticatedUser {
  pub user_id: Uuid
}

impl AuthenticatedUser {
  pub fn from_http_request<B>(request: &Request<B>) -> Option<Self> {
    let headers = request.headers();
    headers
      .get("dottit-user-id")
      .and_then(|value| value.to_str().ok())
      .and_then(|value| Uuid::from_str(value).ok())
      .map(|value| {
        Self {
          user_id: value.into()
        }
      })
  }
}
