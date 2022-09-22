use std::sync::Arc;

use async_graphql::{Context, Object};
use service::services::traits::{GetThreadByIdError, ThreadService};

use super::thread::ThreadQuery;

pub struct Query {
  thread_service: Arc<dyn ThreadService>
}

#[Object]
impl Query {
  async fn get_thread_by_id(
    &self,
    _ctx: &Context<'_>,
    thread_id: String
  ) -> Result<ThreadQuery, GetThreadByIdError> {
    self
      .thread_service
      .get_thread_by_id(&thread_id)
      .await
      .map(ThreadQuery::from)
  }
}
