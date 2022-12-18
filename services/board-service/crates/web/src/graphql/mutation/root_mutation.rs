use std::sync::Arc;

use crate::graphql::query::Board;
use async_graphql::{Context, Object, Union};
use board_service_service::{models::dtos::CreateBoardDto, services::traits::BoardService};
use shared_web::{
  gql::{Unauthenticated, ValidationError},
  GqlContextExtensions
};

pub struct Mutation;

#[Object]
impl Mutation {
  pub async fn create_board(
    &self,
    ctx: &Context<'_>,
    name: String
  ) -> anyhow::Result<CreateBoardResult> {
    let Some(user) = ctx.authenticated_user() else {
      return Ok(CreateBoardResult::Unauthenticated(Unauthenticated::default()))
    };
    let service = ctx.data::<Arc<dyn BoardService>>().unwrap();

    let create_result = service
      .create_board(user.user_id, CreateBoardDto { name })
      .await?;

    match create_result {
      Ok(board) => Ok(CreateBoardResult::Board(board.into())),
      Err(e) => Ok(CreateBoardResult::ValidationError(e.into()))
    }
  }
}

#[derive(Union)]
pub enum CreateBoardResult {
  Unauthenticated(Unauthenticated),
  ValidationError(ValidationError),
  Board(Board)
}
