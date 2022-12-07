use async_trait::async_trait;
use board_service_model::models::Board;
use shared_service::model::{Page, Pagination};
use uuid::Uuid;

use super::RepositoryResult;

#[async_trait]
pub trait BoardRepository: Send + Sync {
  async fn get_board_by_name(&self, name: &str) -> RepositoryResult<Option<Board>>;

  async fn create_board(&self, name: String, creator_user_id: Uuid) -> RepositoryResult<Board>;

  async fn get_board_by_id(&self, id: Uuid) -> RepositoryResult<Board>;

  async fn get_boards(&self, pagination: Pagination<u64>) -> RepositoryResult<Page<Board, u64>>;
}
