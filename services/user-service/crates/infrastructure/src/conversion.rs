use super::model::user;

use sea_orm::DbErr;
use user_service_service::repos::RepositoryError;

use user_service_model::models::User;

impl From<user::Model> for User {
  fn from(user: user::Model) -> Self {
    Self {
      id:            user.id.to_string(),
      username:      user.username,
      password_hash: user.password_hash
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
