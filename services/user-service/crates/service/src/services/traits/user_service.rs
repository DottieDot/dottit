use async_trait::async_trait;
use user_service_model::models::{UuidStr, UuidString};
use shared_service::service_mediator::MediatorProducerError;
use std::error::Error as StdError;
use thiserror::Error;

use crate::models::dtos::{AuthenticatedUserDto, CreateUserDto, UserDto};

#[async_trait]
pub trait UserService: Send + Sync {
  async fn get_user_by_id(&self, id: &UuidStr) -> Result<UserDto, GetUserByIdError>;

  async fn create_user(&self, dto: CreateUserDto) -> Result<AuthenticatedUserDto, CreateUserError>;

  async fn delete_user(&self, id: &UuidStr) -> Result<(), DeleteUserError>;
}

#[derive(Error, Debug)]
pub enum GetUserByIdError {
  #[error("no user with id \"{user_id}\" could be found")]
  NotFound {
    user_id: UuidString,
    source:  Box<dyn StdError + Send + Sync>
  },
  #[error("an unknown error ocurred")]
  Unknown {
    source: Box<dyn StdError + Send + Sync>
  }
}

#[derive(Error, Debug)]
pub enum CreateUserError {
  #[error("an unknown error ocurred: {source}")]
  Unknown {
    source: Box<dyn StdError + Send + Sync>
  },
  #[error("mediator request failed: {source}")]
  MediatorError { source: MediatorProducerError },
  #[error("failed to hash user password: {source}")]
  HashError { source: bcrypt::BcryptError }
}

#[derive(Error, Debug)]
pub enum DeleteUserError {
  #[error("no user with id \"{user_id}\" could be found")]
  NotFound {
    user_id: UuidString,
    source:  Box<dyn StdError + Send + Sync>
  },
  #[error("an unknown error ocurred")]
  Unknown {
    source: Box<dyn StdError + Send + Sync>
  }
}
