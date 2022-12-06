use async_trait::async_trait;
use board_service_model::models::Board;
use uuid::Uuid;

use super::RepositoryResult;

#[async_trait]
pub trait BoardModeratorRepository: Send + Sync {
  async fn get_boards_moderated_by_user(&self, user_id: Uuid) -> RepositoryResult<Vec<Board>>;

  async fn get_board_moderators(&self, board_id: Uuid) -> RepositoryResult<Vec<Uuid>>;
}
