use std::sync::Arc;

use async_graphql::{Context, Object};
use board_service_service::{models::dtos::BoardDto, services::traits::BoardService};
use uuid::Uuid;

use super::User;

pub struct Board {
  board: BoardDto
}

#[Object]
impl Board {
  pub async fn id(&self) -> Uuid {
    self.board.id
  }

  pub async fn name(&self) -> &str {
    &self.board.name
  }

  pub async fn moderators(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<User>> {
    let service = ctx.data::<Arc<dyn BoardService>>().unwrap();

    let mods = service
      .get_board_moderators(self.board.id)
      .await?
      .into_iter()
      .map(|id| User { id })
      .collect();

    Ok(mods)
  }
}

impl From<BoardDto> for Board {
  fn from(dto: BoardDto) -> Self {
    Self { board: dto }
  }
}
