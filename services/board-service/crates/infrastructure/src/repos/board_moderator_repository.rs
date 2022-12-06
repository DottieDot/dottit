use async_trait::async_trait;
use board_service_model::models::Board;
use board_service_service::repos::{
  BoardModeratorRepository as BoardModeratorRepoTrait, RepositoryResult
};
use sea_orm::{prelude::Uuid, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::sync::Arc;

use crate::{
  model::{board, board_moderator},
  repo_error_from_db_error
};

pub struct BoardModeratorRepository {
  db: Arc<DatabaseConnection>
}

impl BoardModeratorRepository {
  pub fn new(db: Arc<DatabaseConnection>) -> Self {
    Self { db }
  }
}

#[async_trait]
impl BoardModeratorRepoTrait for BoardModeratorRepository {
  async fn get_boards_moderated_by_user(&self, user_id: Uuid) -> RepositoryResult<Vec<Board>> {
    let query_result = board_moderator::Entity::find()
      .filter(board_moderator::Column::UserId.eq(user_id))
      .find_also_related(board::Entity)
      .all(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    let boards = query_result
      .into_iter()
      .filter_map(|(_, board)| board.map(Board::from))
      .collect();
    Ok(boards)
  }

  async fn get_board_moderators(&self, board_id: Uuid) -> RepositoryResult<Vec<Uuid>> {
    let query_result = board_moderator::Entity::find()
      .filter(board_moderator::Column::BoardId.eq(board_id))
      .all(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    let users = query_result
      .into_iter()
      .map(|moderator| moderator.user_id)
      .collect();
    Ok(users)
  }
}
