use std::sync::Arc;

use async_graphql::{Context, Object};
use service::services::traits::{GetThreadByIdError, ThreadService};

use super::thread_query::ThreadQuery;

pub struct RootQuery;

#[Object]
impl RootQuery {
  async fn get_thread_by_id(
    &self,
    ctx: &Context<'_>,
    thread_id: String
  ) -> Result<ThreadQuery, GetThreadByIdError> {
    let service = ctx.data::<Arc<dyn ThreadService>>().unwrap();

    service
      .get_thread_by_id(&thread_id)
      .await
      .map(ThreadQuery::from)
  }
}
