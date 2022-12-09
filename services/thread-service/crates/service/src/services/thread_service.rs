use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use shared_service::{
  events::ThreadDeletedEvent,
  messaging::EventBus,
  model::{Page, Pagination},
  validation::{
    validators::StringLength, FieldValidator, ValidationError, ValidationResultBuilder, Validator
  }
};
use thread_service_model::models::Thread;
use uuid::Uuid;

use crate::{
  model::dtos::CreateThreadDto,
  repos::{RepositoryError, ThreadRepository}
};

use super::traits::{
  self, CreateThreadError, DeleteThreadError, GetThreadByIdError, GetThreadsByBoardError
};

pub struct ThreadService {
  thread_repo:             Arc<dyn ThreadRepository>,
  event_bus:               Arc<EventBus>,
  create_thread_validator: CreateThreadValidator
}

impl ThreadService {
  pub fn new(thread_repo: Arc<dyn ThreadRepository>, event_bus: Arc<EventBus>) -> Self {
    Self {
      thread_repo,
      event_bus,
      create_thread_validator: CreateThreadValidator
    }
  }
}

#[async_trait]
impl traits::ThreadService for ThreadService {
  async fn get_thread_by_id(&self, id: Uuid) -> Result<Thread, GetThreadByIdError> {
    Ok(self.thread_repo.get_thread_by_id(id).await?)
  }

  async fn get_threads_by_user(
    &self,
    user_id: Uuid,
    pagination: Pagination<DateTime<Utc>>
  ) -> Result<Page<Thread, DateTime<Utc>>, GetThreadsByBoardError> {
    Ok(
      self
        .thread_repo
        .get_threads_by_user(user_id, pagination)
        .await?
    )
  }

  async fn get_threads_by_board(
    &self,
    board_id: Uuid,
    pagination: Pagination<DateTime<Utc>>
  ) -> Result<Page<Thread, DateTime<Utc>>, traits::GetThreadsByBoardError> {
    Ok(
      self
        .thread_repo
        .get_threads_by_board(board_id, pagination)
        .await?
    )
  }

  async fn create_thread(
    &self,
    dto: CreateThreadDto
  ) -> Result<Result<Thread, ValidationError>, traits::CreateThreadError> {
    if let Err(e) = self.create_thread_validator.validate(&dto).await {
      return Ok(Err(e));
    }

    Ok(Ok(
      self
        .thread_repo
        .create_thread(dto.board_id, dto.user_id, dto.title, dto.text)
        .await?
    ))
  }

  async fn delete_thread(&self, thread_id: Uuid) -> Result<(), DeleteThreadError> {
    self.thread_repo.delete_thread(thread_id).await?;

    self
      .event_bus
      .publish(ThreadDeletedEvent { thread_id })
      .await
      .unwrap();

    Ok(())
  }
}

impl From<RepositoryError> for GetThreadByIdError {
  fn from(error: RepositoryError) -> Self {
    match &error {
      RepositoryError::NotFound { key, .. } => {
        Self::NotFound {
          thread_id: key.clone(),
          source:    Box::new(error)
        }
      }
      _ => {
        Self::Unknown {
          source: Box::new(error)
        }
      }
    }
  }
}

impl From<RepositoryError> for DeleteThreadError {
  fn from(error: RepositoryError) -> Self {
    match &error {
      RepositoryError::NotFound { key, .. } => {
        Self::NotFound {
          thread_id: key.clone(),
          source:    Box::new(error)
        }
      }
      _ => {
        Self::Unknown {
          source: Box::new(error)
        }
      }
    }
  }
}

impl From<RepositoryError> for GetThreadsByBoardError {
  fn from(error: RepositoryError) -> Self {
    Self::Unknwon {
      source: Box::new(error)
    }
  }
}

impl From<RepositoryError> for CreateThreadError {
  fn from(error: RepositoryError) -> Self {
    Self::Unknwon {
      source: Box::new(error)
    }
  }
}

struct CreateThreadValidator;

#[async_trait]
impl Validator<CreateThreadDto> for CreateThreadValidator {
  async fn validate(&self, dto: &CreateThreadDto) -> Result<(), ValidationError> {
    ValidationResultBuilder::default()
      .field(FieldValidator::new("title", &dto.title).length_range(8..=128))
      .field(FieldValidator::new("text", &dto.text).length_range(8..=2046))
      .finish()
  }
}
