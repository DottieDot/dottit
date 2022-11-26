use std::sync::Arc;

use async_trait::async_trait;
use user_service_model::models::UuidStr;
use shared_service::service_mediator::{
  queries::CreateApiTokenForUserQuery, MediatorProducer, MediatorProducerError
};

use crate::{
  models::dtos::{AuthenticatedUserDto, CreateUserDto, UserDto},
  repos::{RepositoryError, UserRepository}
};

use super::traits::{self, CreateUserError, DeleteUserError, GetUserByIdError};

pub struct UserService {
  repo:     Arc<dyn UserRepository>,
  mediator: Arc<MediatorProducer>
}

impl UserService {
  pub fn new(repo: Arc<dyn UserRepository>, mediator: Arc<MediatorProducer>) -> Self {
    Self { repo, mediator }
  }
}

#[async_trait]
impl traits::UserService for UserService {
  async fn get_user_by_id(&self, id: &UuidStr) -> Result<UserDto, GetUserByIdError> {
    Ok(self.repo.get_user_by_id(id).await?.into())
  }

  async fn create_user(&self, dto: CreateUserDto) -> Result<AuthenticatedUserDto, CreateUserError> {
    let password_hash = bcrypt::hash(dto.password, bcrypt::DEFAULT_COST)?;

    let user = self.repo.create_user(dto.username, password_hash).await?;
    let token = self
      .mediator
      .query(CreateApiTokenForUserQuery {
        user_id: user.id.clone()
      })
      .await?
      .token;

    Ok(AuthenticatedUserDto {
      user:      user.into(),
      api_token: token
    })
  }

  async fn delete_user(&self, id: &UuidStr) -> Result<(), DeleteUserError> {
    Ok(self.repo.delete_user(id).await?)
  }
}

impl From<RepositoryError> for GetUserByIdError {
  fn from(error: RepositoryError) -> Self {
    match &error {
      RepositoryError::NotFound { key, .. } => {
        Self::NotFound {
          user_id: key.clone(),
          source:  error.into()
        }
      }
      _ => {
        Self::Unknown {
          source: error.into()
        }
      }
    }
  }
}

impl From<RepositoryError> for CreateUserError {
  fn from(error: RepositoryError) -> Self {
    CreateUserError::Unknown {
      source: error.into()
    }
  }
}

impl From<MediatorProducerError> for CreateUserError {
  fn from(error: MediatorProducerError) -> Self {
    Self::MediatorError { source: error }
  }
}

impl From<bcrypt::BcryptError> for CreateUserError {
  fn from(error: bcrypt::BcryptError) -> Self {
    Self::HashError { source: error }
  }
}

impl From<RepositoryError> for DeleteUserError {
  fn from(error: RepositoryError) -> Self {
    match &error {
      RepositoryError::NotFound { key, .. } => {
        Self::NotFound {
          user_id: key.clone(),
          source:  error.into()
        }
      }
      _ => {
        Self::Unknown {
          source: error.into()
        }
      }
    }
  }
}
