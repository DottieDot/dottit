use std::sync::Arc;

use async_graphql::{Context, Object};
use thread_service_service::services::traits::{GetThreadByIdError, ThreadService};
use uuid::Uuid;

use super::{Board, Thread, User};

pub struct Query;

#[Object]
impl Query {
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

  #[graphql(entity)]
  async fn get_board_by_id(&self, id: Uuid) -> Board {
    Board::new(id)
  }

  #[graphql(entity)]
  async fn get_user_by_id(&self, id: Uuid) -> User {
    User::new(id)
  }
}
