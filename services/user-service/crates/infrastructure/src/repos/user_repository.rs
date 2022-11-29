use crate::{
  model::{user, user::Entity as DbUser},
  repo_error_from_db_error
};
use async_trait::async_trait;
use sea_orm::{
  prelude::Uuid, ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait,
  QueryFilter
};
use std::sync::Arc;
use user_service_model::models::User;
use user_service_service::repos::{
  RepositoryError, RepositoryResult, UserRepository as UserRepoTrait
};

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
  async fn get_user_by_id(&self, id: Uuid) -> RepositoryResult<User> {
    let query_result = DbUser::find_by_id(id)
      .one(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    query_result.map(|user| user.into()).ok_or_else(|| {
      RepositoryError::NotFound {
        key:    id.to_string(),
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

  async fn get_user_by_username(&self, username: &str) -> RepositoryResult<Option<User>> {
    let user = DbUser::find()
      .filter(user::Column::Username.eq(username))
      .one(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    Ok(user.map(|u| u.into()))
  }

  async fn delete_user(&self, id: Uuid) -> RepositoryResult<()> {
    DbUser::delete_by_id(id)
      .exec(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    Ok(())
  }
}
