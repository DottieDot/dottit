use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};
use user_service_service::services::traits::UserService;

use super::{mutation::Mutation, query::Query};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn build_schema(user_service: Arc<dyn UserService>) -> AppSchema {
  Schema::build(Query, Mutation, EmptySubscription)
    .data(user_service)
    .enable_federation()
    .finish()
}
