use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};
use service::services::traits::ThreadService;

use super::{mutation::RootMutation, query::RootQuery};

pub type AppSchema = Schema<RootQuery, RootMutation, EmptySubscription>;

pub async fn build_schema(thread_service: Arc<dyn ThreadService>) -> AppSchema {
  Schema::build(RootQuery, RootMutation, EmptySubscription)
    .data(thread_service)
    .finish()
}
