use std::sync::Arc;

use async_graphql::{Context, Object, ID};
use thread_service_model::models::Pagination;
use thread_service_service::services::traits::{GetThreadByIdError, GetThreadsByBoardError, ThreadService};

use super::{Paged, Thread};

pub struct Query;

#[Object]
impl Query {
  async fn get_threads_by_board(
    &self,
    ctx: &Context<'_>,
    board: String,
    first: u64,
    count: u64
  ) -> Result<Paged<Thread>, GetThreadsByBoardError> {
    let service = ctx.data::<Arc<dyn ThreadService>>().unwrap();

    Ok(
      service
        .get_threads_by_board(&board, Pagination { first, count })
        .await?
        .inner_into::<Thread>()
        .into()
    )
  }

  async fn get_thread_by_id(
    &self,
    ctx: &Context<'_>,
    thread_id: ID
  ) -> Result<Thread, GetThreadByIdError> {
    self.find_thread_by_id(ctx, thread_id).await
  }

  #[graphql(entity)]
  async fn find_thread_by_id(
    &self,
    ctx: &Context<'_>,
    id: ID
  ) -> Result<Thread, GetThreadByIdError> {
    let service = ctx.data::<Arc<dyn ThreadService>>().unwrap();

    service.get_thread_by_id(&id).await.map(Thread::from)
  }
}
