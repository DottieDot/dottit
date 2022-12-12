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
use shared_service::service_mediator::MediatorProducer;
use shared_web::{middleware::auth, AuthenticatedUser};
use std::{env, sync::Arc};

use comment_service_infrastructure::repos::CommentRepository;
use comment_service_service::services::CommentService;

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
  // let _ = event_bus::connect().await;

  // mediator
  let mediator_producer =
    Arc::new(MediatorProducer::new(env::var("MEDIATOR_URL").unwrap()).unwrap());

  // services
  let comment_repo = Arc::new(CommentRepository::new(db.connection().clone()));
  let comment_service = Arc::new(CommentService::new(comment_repo, mediator_producer));

  let schema = build_schema(comment_service).await;

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
