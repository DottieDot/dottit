use std::sync::Arc;

use async_graphql::{Context, Object};
use board_service_service::services::traits::BoardService;
use shared_service::model::Pagination;
use uuid::Uuid;

use super::{user::User, Board, Paged};

pub struct Query;

#[Object]
impl Query {
  async fn get_board_by_name(
    &self,
    ctx: &Context<'_>,
    name: String
  ) -> anyhow::Result<Option<Board>> {
    let service = ctx.data::<Arc<dyn BoardService>>().unwrap();

    Ok(service.get_board_by_name(&name).await?.map(Board::from))
  }

  async fn boards(
    &self,
    ctx: &Context<'_>,
    first: u64,
    count: u64
  ) -> anyhow::Result<Paged<Board, u64>> {
    let service = ctx.data::<Arc<dyn BoardService>>().unwrap();

    Ok(
      service
        .get_boards(Pagination { first, count })
        .await?
        .inner_into::<Board>()
        .into()
    )
  }

  #[graphql(entity)]
  async fn find_board_by_id(&self, ctx: &Context<'_>, id: Uuid) -> anyhow::Result<Board> {
    let service = ctx.data::<Arc<dyn BoardService>>().unwrap();

    service.get_board_by_id(id).await.map(Board::from)
  }

  #[graphql(entity)]
  async fn get_user_by_id(&self, id: Uuid) -> User {
    User { id }
  }
}
