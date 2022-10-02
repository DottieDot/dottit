use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::Extension, routing::post, Router};
use std::sync::Arc;

use infrastructure::repos::ThreadRepository;
use service::services::ThreadService;

use self::{
  database::Database,
  graphql::{build_schema, AppSchema}
};

mod database;
mod event_bus;
pub mod graphql;

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
  schema.execute(req.into_inner()).await.into()
}

#[tokio::main]
async fn main() {
  // database
  let db = Database::connect().await;

  db.migrate().await;

  // event bus
  let event_bus = event_bus::connect().await;

  // services
  let thread_repo = Arc::new(ThreadRepository::new(db.connection().clone()));
  let thread_service = Arc::new(ThreadService::new(thread_repo, event_bus));

  let schema = build_schema(thread_service).await;

  // Web
  let router = Router::<axum::body::Body>::new()
    .route("/graphql", post(graphql_handler))
    .layer(Extension(schema));

  axum::Server::bind(&"0.0.0.0:8080".parse().expect("invalid server binding"))
    .serve(router.into_make_service())
    .await
    .unwrap();
}
