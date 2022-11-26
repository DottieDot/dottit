use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};
use comment_service_service::services::traits::CommentService;

use super::{mutation::Mutation, query::Query};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn build_schema(thread_service: Arc<dyn CommentService>) -> AppSchema {
  Schema::build(Query, Mutation, EmptySubscription)
    .data(thread_service)
    .enable_federation()
    .finish()
}
