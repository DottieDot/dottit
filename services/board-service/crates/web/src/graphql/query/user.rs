use std::sync::Arc;

use async_graphql::{Context, Object};
use board_service_service::services::traits::BoardService;
use uuid::Uuid;

use super::Board;

pub struct User {
  pub id: Uuid
}

#[Object(extends)]
impl User {
  #[graphql(external)]
  async fn id(&self) -> Uuid {
    self.id
  }

  async fn moderates(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<Board>> {
    let service = ctx.data::<Arc<dyn BoardService>>().unwrap();

    let boards = service
      .get_boards_moderated_by_user(self.id)
      .await?
      .into_iter()
      .map(Board::from)
      .collect();

    Ok(boards)
  }
}
