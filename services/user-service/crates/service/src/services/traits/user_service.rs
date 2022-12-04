use async_trait::async_trait;
use shared_service::{service_mediator::MediatorProducerError, validation::ValidationError};
use std::error::Error as StdError;
use thiserror::Error;
use uuid::Uuid;

use crate::models::dtos::{AuthenticatedUserDto, CreateUserDto, LoginDto, UserDto};

#[async_trait]
pub trait UserService: Send + Sync {
  async fn get_user_by_id(&self, id: Uuid) -> Result<UserDto, GetUserByIdError>;

  async fn create_user(
    &self,
    dto: CreateUserDto
  ) -> Result<Result<AuthenticatedUserDto, ValidationError>, CreateUserError>;

  async fn login(&self, dto: LoginDto) -> Result<Option<AuthenticatedUserDto>, LoginError>;

  async fn delete_user(&self, id: Uuid) -> Result<(), DeleteUserError>;

  async fn get_user_by_username(&self, username: &str) -> Result<Option<UserDto>, GetUserByUsernameError>;
}

#[derive(Error, Debug)]
pub enum GetUserByUsernameError {
  #[error("an unknown error ocurred")]
  Unknown {
    source: Box<dyn StdError + Send + Sync>
  }
}

#[derive(Error, Debug)]
pub enum GetUserByIdError {
  #[error("no user with id \"{user_id}\" could be found")]
  NotFound {
    user_id: String,
    source:  Box<dyn StdError + Send + Sync>
  },
  #[error("an unknown error ocurred")]
  Unknown {
    source: Box<dyn StdError + Send + Sync>
  }
}

#[derive(Error, Debug)]
pub enum LoginError {
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
    user_id: String,
    source:  Box<dyn StdError + Send + Sync>
  },
  #[error("an unknown error ocurred")]
  Unknown {
    source: Box<dyn StdError + Send + Sync>
  }
}
