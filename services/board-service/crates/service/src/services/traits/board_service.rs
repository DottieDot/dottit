use async_trait::async_trait;
use shared_service::{
  model::{Page, Pagination},
  validation::ValidationError
};
use uuid::Uuid;

use crate::models::dtos::{BoardDto, CreateBoardDto};

#[async_trait]
pub trait BoardService: Send + Sync {
  async fn get_board_by_name(&self, name: &str) -> anyhow::Result<Option<BoardDto>>;

  async fn create_board(
    &self,
    user_id: Uuid,
    dto: CreateBoardDto
  ) -> anyhow::Result<Result<BoardDto, ValidationError>>;

  async fn get_board_by_id(&self, id: Uuid) -> anyhow::Result<BoardDto>;

  async fn get_boards_moderated_by_user(&self, user: Uuid) -> anyhow::Result<Vec<BoardDto>>;

  async fn get_board_moderators(&self, board: Uuid) -> anyhow::Result<Vec<Uuid>>;

  async fn get_boards(&self, pagination: Pagination<u64>) -> anyhow::Result<Page<BoardDto, u64>>;
}
