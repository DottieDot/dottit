use super::model::thread;

use sea_orm::DbErr;
use thread_service_service::repos::RepositoryError;

use thread_service_model::models::Thread;

impl From<thread::Model> for Thread {
  fn from(thread: thread::Model) -> Self {
    Self {
      id:         thread.id,
      board_id:   thread.board_id,
      user_id:    thread.user_id,
      title:      thread.title,
      text:       thread.text,
      created_at: thread.created_at.into()
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
