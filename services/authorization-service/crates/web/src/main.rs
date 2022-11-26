use std::sync::Arc;

use axum::{body::Body, http::StatusCode, response::IntoResponse, routing::get, Router, Server};
use service::{
  database::AuthTokenDb,
  service::{traits::AuthTokenService as AuthTokenServiceTrait, AuthTokenService}
};
use shared_service::service_mediator::{
  queries::{ApiTokenForUser, CreateApiTokenForUserQuery},
  MediatorConsumer
};

use self::database::Database;

mod database;
mod event_bus;

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
    .route("/health", get(health_probe_handler));

  Server::bind(&"0.0.0.0:8080".parse().expect("invalid server binding"))
    .serve(router.into_make_service())
    .await
    .unwrap();
}
