use std::sync::Arc;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use authorization_service_service::{
  database::AuthTokenDb,
  service::{traits::AuthTokenService as AuthTokenServiceTrait, AuthTokenService}
};
use axum::{
  body::Body,
  extract::Path,
  http::StatusCode,
  middleware,
  response::{Html, IntoResponse, Response},
  routing::{get, post},
  Extension, Json, Router, Server
};
use serde::{Deserialize, Serialize};
use shared_service::service_mediator::{
  queries::{ApiTokenForUser, CreateApiTokenForUserQuery},
  MediatorConsumer
};
use shared_web::{middleware::auth, AuthenticatedUser};
use uuid::Uuid;

use self::{
  database::Database,
  graphql::{build_schema, AppSchema}
};

mod database;
mod event_bus;
mod graphql;

#[derive(Serialize, Deserialize)]
struct ValidateResponse {
  user_id: Uuid
}

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

  let tmp = auth_token_service.clone();
  consumer
    .subscribe(move |query: CreateApiTokenForUserQuery| {
      let service = tmp.clone();
      async move {
        let token = service.create_token_for_user(query.user_id).await?;

        let result = ApiTokenForUser {
          user_id: query.user_id,
          token
        };

        Ok(result)
      }
    })
    .await;

  let schema = build_schema(auth_token_service.clone()).await;

  // Web
  let router = Router::<Body>::new()
    .route("/readiness", get(readiness_probe_handler))
    .route("/health", get(health_probe_handler))
    .route("/validate/:token", get(validate_handler))
    .route("/", get(playground_handler))
    .route("/graphql", post(graphql_handler))
    .route_layer(middleware::from_fn(auth))
    .layer(Extension(schema))
    .layer(Extension(auth_token_service.clone()));

  Server::bind(&"0.0.0.0:8080".parse().expect("invalid server binding"))
    .serve(router.into_make_service())
    .await
    .unwrap();
}
