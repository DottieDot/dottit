use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};
use board_service_service::services::traits::BoardService;

use super::{mutation::Mutation, query::Query};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn build_schema(board_service: Arc<dyn BoardService>) -> AppSchema {
  Schema::build(Query, Mutation, EmptySubscription)
    .data(board_service)
    .enable_federation()
    .finish()
}
