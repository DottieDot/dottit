use super::model::comment;

use sea_orm::DbErr;
use comment_service_service::repos::RepositoryError;

use comment_service_model::models::Comment;

impl From<comment::Model> for Comment {
  fn from(comment: comment::Model) -> Self {
    Self {
      id:        comment.id.to_string(),
      thread_id: comment.thread_id.to_string(),
      user:      comment.user,
      text:      comment.text,
      score:     comment.score
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
