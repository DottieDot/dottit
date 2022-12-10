use super::model::comment;

use comment_service_service::repos::RepositoryError;
use sea_orm::DbErr;

use comment_service_model::models::Comment;

impl From<comment::Model> for Comment {
  fn from(comment: comment::Model) -> Self {
    Self {
      id:         comment.id,
      thread_id:  comment.thread_id,
      user_id:    comment.user_id,
      text:       comment.text,
      created_at: comment.created_at.into()
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
