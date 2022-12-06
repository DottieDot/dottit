use super::model::board;

use board_service_service::repos::RepositoryError;
use sea_orm::DbErr;

use board_service_model::models::Board;

impl From<board::Model> for Board {
  fn from(board: board::Model) -> Self {
    Self {
      id:   board.id,
      name: board.name
    }
  }
}

pub fn repo_error_from_db_error(error: DbErr) -> RepositoryError {
  match &error {
    DbErr::RecordNotFound(record) => {
      RepositoryError::NotFound {
        key:    record.clone(),
        source: Some(Box::new(error))
      }
    }
    _ => {
      RepositoryError::DatabaseError {
        source: Box::new(error)
      }
    }
  }
}
