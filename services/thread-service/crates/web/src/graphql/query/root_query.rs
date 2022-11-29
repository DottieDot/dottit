use std::sync::Arc;

use async_graphql::{Context, Object};
use chrono::{DateTime, Utc};
use shared_service::model::Pagination;
use thread_service_service::services::traits::{
  GetThreadByIdError, GetThreadsByBoardError, ThreadService
};
use uuid::Uuid;

use super::{Paged, Thread};

pub struct Query;

#[Object]
impl Query {
  async fn get_threads_by_board(
    &self,
    ctx: &Context<'_>,
    board_id: Uuid,
    first: DateTime<Utc>,
    count: u64
  ) -> Result<Paged<Thread, DateTime<Utc>>, GetThreadsByBoardError> {
    let service = ctx.data::<Arc<dyn ThreadService>>().unwrap();

    Ok(
      service
        .get_threads_by_board(board_id, Pagination { first, count })
        .await?
        .inner_into::<Thread>()
        .into()
    )
  }

  async fn get_thread_by_id(
    &self,
    ctx: &Context<'_>,
    thread_id: Uuid
  ) -> Result<Thread, GetThreadByIdError> {
    self.find_thread_by_id(ctx, thread_id).await
  }

  #[graphql(entity)]
  async fn find_thread_by_id(
    &self,
    ctx: &Context<'_>,
    id: Uuid
  ) -> Result<Thread, GetThreadByIdError> {
    let service = ctx.data::<Arc<dyn ThreadService>>().unwrap();

    service.get_thread_by_id(id).await.map(Thread::from)
  }
}
