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
use shared_service::events::ThreadDeletedEvent;
use shared_web::{middleware::auth, AuthenticatedUser};
use std::sync::Arc;

use thread_service_infrastructure::repos::ThreadRepository;
use thread_service_service::services::ThreadService;

use self::{
  database::Database,
  graphql::{build_schema, AppSchema}
};

mod database;
mod event_bus;
pub mod graphql;

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

  event_bus
    .subscribe(
      "thread_service.thread_deleted".to_owned(),
      |event: ThreadDeletedEvent| {
        async move {
          println!("Thread {} deleted", event.thread_id);
          Ok(())
        }
      }
    )
    .await;

  // services
  let thread_repo = Arc::new(ThreadRepository::new(db.connection().clone()));
  let thread_service = Arc::new(ThreadService::new(thread_repo, event_bus.clone()));

  let schema = build_schema(thread_service).await;

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
