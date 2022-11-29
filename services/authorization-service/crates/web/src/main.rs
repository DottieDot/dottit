use std::sync::Arc;

use authorization_service_service::{
  database::AuthTokenDb,
  service::{traits::AuthTokenService as AuthTokenServiceTrait, AuthTokenService}
};
use axum::{
  body::Body,
  extract::Path,
  http::StatusCode,
  response::{IntoResponse, Response},
  routing::get,
  Extension, Json, Router, Server
};
use serde::{Deserialize, Serialize};
use shared_service::service_mediator::{
  queries::{ApiTokenForUser, CreateApiTokenForUserQuery},
  MediatorConsumer
};

use self::database::Database;

mod database;
mod event_bus;

#[derive(Serialize, Deserialize)]
struct ValidateResponse {
  user_id: String
}

async fn validate_handler(
  Path(token): Path<String>,
  token_service: Extension<Arc<AuthTokenService>>
) -> Response {
  match token_service.get_user_id_from_token(&token).await {
    Ok(Some(user_id)) => Json(ValidateResponse { user_id }).into_response(),
    Ok(None) => StatusCode::NOT_FOUND.into_response(),
    Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response()
  }
}

async fn readiness_probe_handler() -> impl IntoResponse {
  StatusCode::OK
}

async fn health_probe_handler() -> impl IntoResponse {
  StatusCode::OK
}

#[tokio::main]
async fn main() {
  let event_bus = event_bus::connect().await;

  let consumer = MediatorConsumer::new(event_bus);

  let db_client = Database::connect();

  let auth_token_db = Arc::new(AuthTokenDb::new(db_client.connection().clone()));
  let auth_token_service = Arc::new(AuthTokenService::new(auth_token_db));

  consumer
    .subscribe(move |query: CreateApiTokenForUserQuery| {
      let service = auth_token_service.clone();
      async move {
        let token = service.create_token_for_user(&query.user_id).await?;

        let result = ApiTokenForUser {
          user_id: query.user_id,
          token
        };

        Ok(result)
      }
    })
    .await;

  // Web
  let router = Router::<Body>::new()
    .route("/readiness", get(readiness_probe_handler))
    .route("/health", get(health_probe_handler))
    .route("/validate/:token", get(validate_handler));

  Server::bind(&"0.0.0.0:8080".parse().expect("invalid server binding"))
    .serve(router.into_make_service())
    .await
    .unwrap();
}
