use std::sync::Arc;

use async_graphql::{Context, Object, ID};
use service::services::traits::{GetThreadByIdError, ThreadService};

use super::thread_query::ThreadQuery;

pub struct Query;

#[Object]
impl Query {
  async fn get_thread_by_id(
    &self,
    ctx: &Context<'_>,
    thread_id: ID
  ) -> Result<ThreadQuery, GetThreadByIdError> {
    self.find_thread_by_id(ctx, thread_id).await
  }

  #[graphql(entity)]
  async fn find_thread_by_id(
    &self,
    ctx: &Context<'_>,
    #[graphql(key)] id: ID
  ) -> Result<ThreadQuery, GetThreadByIdError> {
    let service = ctx.data::<Arc<dyn ThreadService>>().unwrap();

    service.get_thread_by_id(&id).await.map(ThreadQuery::from)
  }
}
