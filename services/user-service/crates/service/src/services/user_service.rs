use std::sync::Arc;

use async_trait::async_trait;
use lazy_static::lazy_static;
use regex::Regex;
use shared_service::{
  service_mediator::{
    queries::CreateApiTokenForUserQuery, MediatorProducer, MediatorProducerError
  },
  validation::{
    validators::{MatchesRegex, StringLength},
    FieldValidator, ValidationError, ValidationResultBuilder, Validator
  }
};
use tracing::{debug, info};
use uuid::Uuid;

use crate::{
  models::dtos::{AuthenticatedUserDto, CreateUserDto, LoginDto, UserDto},
  repos::{RepositoryError, UserRepository}
};

use super::traits::{
  self, CreateUserError, DeleteUserError, GetUserByIdError, GetUserByUsernameError, LoginError
};

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
  async fn get_user_by_id(&self, id: Uuid) -> Result<UserDto, GetUserByIdError> {
    Ok(self.repo.get_user_by_id(id).await?.into())
  }

  async fn create_user(
    &self,
    dto: CreateUserDto
  ) -> Result<Result<AuthenticatedUserDto, ValidationError>, CreateUserError> {
    if let Err(e) = self.validate(&dto).await {
      info!(
        "Validation failed for creating user with username {}",
        dto.username
      );

      return Ok(Err(e));
    }

    info!("Creating user with username {}", dto.username);

    let password_hash = bcrypt::hash(dto.password, bcrypt::DEFAULT_COST)?;

    let user = self
      .repo
      .create_user(dto.username.clone(), password_hash)
      .await?;
    let token = self
      .mediator
      .query(CreateApiTokenForUserQuery { user_id: user.id })
      .await?
      .token;

    info!("User {} created with username {}", user.id, dto.username);

    Ok(Ok(AuthenticatedUserDto {
      user:      user.into(),
      api_token: token
    }))
  }

  #[tracing::instrument(skip_all)]
  async fn login(&self, dto: LoginDto) -> Result<Option<AuthenticatedUserDto>, LoginError> {
    let Some(user) = self.repo.get_user_by_username(&dto.username).await? else {
      info!("Failed login attempt for nonexistent user: {}", dto.username);
      return Ok(None)
    };

    if !matches!(bcrypt::verify(dto.password, &user.password_hash), Ok(true)) {
      info!("Failed login attempt for user: {}", user.id);
      return Ok(None);
    }

    debug!("Creating token for user: {}", user.id);

    let token = self
      .mediator
      .query(CreateApiTokenForUserQuery { user_id: user.id })
      .await?
      .token;

    info!("User {} logged in", user.id);

    Ok(Some(AuthenticatedUserDto {
      user:      user.into(),
      api_token: token
    }))
  }

  #[tracing::instrument(skip_all)]
  async fn get_user_by_username(
    &self,
    username: &str
  ) -> Result<Option<UserDto>, GetUserByUsernameError> {
    let user = self
      .repo
      .get_user_by_username(username)
      .await?
      .map(UserDto::from);

    match user {
      Some(u) => {
        debug!("User {} retrieved with username: {username}", u.id);
        Ok(Some(u))
      }
      None => {
        debug!("No user with username {username} exists");
        Ok(None)
      }
    }
  }

  async fn delete_user(&self, id: Uuid) -> Result<(), DeleteUserError> {
    Ok(self.repo.delete_user(id).await?)
  }
}

impl From<RepositoryError> for GetUserByUsernameError {
  fn from(error: RepositoryError) -> Self {
    Self::Unknown {
      source: error.into()
    }
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

impl From<RepositoryError> for LoginError {
  fn from(error: RepositoryError) -> Self {
    Self::Unknown {
      source: error.into()
    }
  }
}

impl From<MediatorProducerError> for LoginError {
  fn from(error: MediatorProducerError) -> Self {
    Self::MediatorError { source: error }
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

#[async_trait]
impl Validator<CreateUserDto> for UserService {
  async fn validate(&self, value: &CreateUserDto) -> Result<(), ValidationError> {
    lazy_static! {
      static ref LOWERCASE_RE: Regex = Regex::new("[a-z]").unwrap();
      static ref UPPERCASE_RE: Regex = Regex::new("[A-Z]").unwrap();
      static ref NUMBER_RE: Regex = Regex::new("[0-9]").unwrap();
    }

    ValidationResultBuilder::default()
      .field(FieldValidator::new("username", &value.username).length_range(4..=24))
      .field(
        FieldValidator::new("password", &value.password)
          .min_length(8)
          .matches(&LOWERCASE_RE, "must contain a lower case character.")
          .matches(&UPPERCASE_RE, "must contain an uppercase character.")
          .matches(&NUMBER_RE, "must contain a number.")
      )
      .finish()
  }
}
