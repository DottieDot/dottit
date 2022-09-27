use super::model::thread;

use sea_orm::DbErr;
use service::repos::RepositoryError;

use model::models::Thread;

impl From<thread::Model> for Thread {
  fn from(thread: thread::Model) -> Self {
    Self {
      id:    thread.id.to_string(),
      board: thread.board,
      user:  thread.user,
      title: thread.title,
      text:  thread.text,
      media: thread.media,
      score: thread.score
    }
  }
}

pub fn repo_error_from_db_error(error: DbErr) -> RepositoryError {
  match &error {
    DbErr::RecordNotFound(record) => {
      RepositoryError::Notfound {
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
