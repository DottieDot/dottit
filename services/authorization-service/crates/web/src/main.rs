use axum::{body::Body, http::StatusCode, response::IntoResponse, routing::get, Router, Server};
use shared_service::service_mediator::{
  queries::{ApiTokenForUser, CreateApiTokenForUserQuery},
  MediatorConsumer
};

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

  consumer
    .subscribe(|query: CreateApiTokenForUserQuery| {
      async move {
        let result = ApiTokenForUser {
          user_id: query.user_id,
          token:   "Test Token!".to_owned()
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
