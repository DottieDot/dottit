use std::sync::Arc;

use axum::{
  body::{Body, Bytes},
  extract::Path,
  http::StatusCode,
  response::{IntoResponse, Response},
  routing::post,
  Extension, Router, Server
};
use shared_service::{
  events::{MediatorResponse, MEDIATOR_EXCHANGE},
  messaging::{EventBus, EventMetadata, QueueMetadata, QueueOptions}
};
use uuid::Uuid;

use self::{
  mediator_id::MediatorId,
  query_handler::{QueryError, QueryHandler}
};

mod event_bus;
mod mediator_id;
mod query_handler;

async fn query_handler(
  query_handler: Extension<Arc<QueryHandler>>,
  Path(query_name): Path<String>,
  body: Bytes
) -> impl IntoResponse {
  match query_handler.handle_query(query_name, body.to_vec()).await {
    Ok(response_data) => {
      Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(response_data))
    }
    Err(QueryError::TimedOut) => {
      Response::builder()
        .status(StatusCode::GATEWAY_TIMEOUT)
        .body(Body::empty())
    }
    Err(QueryError::InternalError(_)) => {
      Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::empty())
    }
  }
  .unwrap()
}

async fn register_response_listener(
  event_bus: &EventBus,
  mediator_id: &MediatorId,
  query_handler: Arc<QueryHandler>
) {
  event_bus
    .manual_subscribe(
      mediator_id.uuid.to_string(),
      EventMetadata {
        exchange:        MEDIATOR_EXCHANGE,
        queue:           QueueMetadata {
          routing_key:  &mediator_id.uuid.to_string(),
          options:      QueueOptions {
            exclusive: true,
            ..Default::default()
          },
          bind_options: Default::default()
        },
        consume_options: Default::default()
      },
      move |response: MediatorResponse| {
        let cloned = query_handler.clone();
        async move {
          cloned.handle_response(response).await;
          Ok(())
        }
      }
    )
    .await;
}

#[tokio::main]
async fn main() {
  let mediator_id = Arc::new(MediatorId {
    uuid: Uuid::new_v4()
  });
  let event_bus = event_bus::connect().await;
  let query_handler = Arc::new(QueryHandler::new(event_bus.clone(), mediator_id.clone()));

  register_response_listener(&event_bus, &mediator_id, query_handler.clone()).await;

  let router = Router::<axum::body::Body>::new()
    .route("/query/:query_name", post(self::query_handler))
    .layer(Extension(query_handler));

  Server::bind(&"0.0.0.0:8080".parse().expect("invalid server binding"))
    .serve(router.into_make_service())
    .await
    .unwrap();
}
