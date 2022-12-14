use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
  extract::Extension,
  http::StatusCode,
  middleware,
  response::{Html, IntoResponse},
  routing::{get, post},
  Router
};
use shared_service::service_mediator::MediatorConsumer;
use shared_web::{middleware::auth, AuthenticatedUser};
use std::sync::Arc;

use thread_service_infrastructure::repos::ThreadRepository;
use thread_service_service::services::ThreadService;

use self::{
  database::Database,
  graphql::{build_schema, AppSchema},
  mediator_handlers::register_mediator_handlers
};

mod database;
mod event_bus;
pub mod graphql;
mod mediator_handlers;

async fn graphql_handler(
  Extension(user): Extension<Option<AuthenticatedUser>>,
  schema: Extension<AppSchema>,
  req: GraphQLRequest
) -> GraphQLResponse {
  let query = req.into_inner();

  let query = match user {
    Some(user) => query.data(user),
    None => query
  };

  schema.execute(query).await.into()
}

async fn playground_handler() -> impl IntoResponse {
  Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

async fn readiness_probe_handler() -> impl IntoResponse {
  StatusCode::OK
}

async fn health_probe_handler() -> impl IntoResponse {
  StatusCode::OK
}

#[tokio::main]
async fn main() {
  // database
  let db = Database::connect().await;

  db.migrate().await;

  // event bus
  let event_bus = event_bus::connect().await;

  // mediator
  let mediator_consumer = Arc::new(MediatorConsumer::new(event_bus.clone()));

  // services
  let thread_repo = Arc::new(ThreadRepository::new(db.connection().clone()));
  let thread_service = Arc::new(ThreadService::new(thread_repo, event_bus.clone()));

  // GraphQL
  let schema = build_schema(thread_service.clone()).await;

  // mediator handler
  register_mediator_handlers(mediator_consumer.clone(), thread_service.clone())
    .await
    .unwrap();

  // Web
  let router = Router::<axum::body::Body>::new()
    .route("/", get(playground_handler))
    .route("/graphql", post(graphql_handler))
    .route_layer(middleware::from_fn(auth))
    .route("/readiness", get(readiness_probe_handler))
    .route("/health", get(health_probe_handler))
    .layer(Extension(schema));

  axum::Server::bind(&"0.0.0.0:8080".parse().expect("invalid server binding"))
    .serve(router.into_make_service())
    .await
    .unwrap();
}
