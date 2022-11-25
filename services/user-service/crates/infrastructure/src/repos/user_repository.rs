use crate::{
  model::{user, user::Entity as DbUser},
  repo_error_from_db_error
};
use async_trait::async_trait;
use model::models::{User, UuidStr};
use sea_orm::{prelude::Uuid, ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};
use service::repos::{RepositoryError, RepositoryResult, UserRepository as UserRepoTrait};
use std::sync::Arc;

pub struct UserRepository {
  db: Arc<DatabaseConnection>
}

impl UserRepository {
  pub fn new(db: Arc<DatabaseConnection>) -> Self {
    Self { db }
  }
}

#[async_trait]
impl UserRepoTrait for UserRepository {
  async fn get_user_by_id(&self, id: &UuidStr) -> RepositoryResult<User> {
    let uuid = Uuid::parse_str(id).unwrap();

    let query_result = DbUser::find_by_id(uuid)
      .one(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    query_result.map(|user| user.into()).ok_or_else(|| {
      RepositoryError::NotFound {
        key:    id.to_owned(),
        source: None
      }
    })
  }

  async fn create_user(&self, username: String, password_hash: String) -> RepositoryResult<User> {
    let new_user = user::ActiveModel {
      id:            ActiveValue::NotSet,
      username:      ActiveValue::Set(username),
      password_hash: ActiveValue::Set(password_hash)
    };

    let query_result = new_user
      .insert(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    Ok(query_result.into())
  }

  async fn delete_user(&self, id: &UuidStr) -> RepositoryResult<()> {
    let uuid = Uuid::parse_str(id).unwrap();

    DbUser::delete_by_id(uuid)
      .exec(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    Ok(())
  }
}
