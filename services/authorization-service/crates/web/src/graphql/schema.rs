use std::sync::Arc;

use async_graphql::{EmptySubscription, Object, Schema};
use authorization_service_service::service::traits::AuthTokenService;

use super::mutation::Mutation;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
  pub async fn unused_authorization_query(&self) -> &str {
    "unused"
  }
}

pub async fn build_schema(auth_service: Arc<dyn AuthTokenService>) -> AppSchema {
  Schema::build(Query, Mutation, EmptySubscription)
    .data(auth_service)
    .enable_federation()
    .finish()
}
