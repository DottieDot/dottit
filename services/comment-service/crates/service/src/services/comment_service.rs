use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use shared_service::{
  model::{Page, Pagination},
  service_mediator::{
    queries::{ThreadExistsQuery, UserExistsQuery},
    MediatorProducer
  },
  validation::{
    validators::{EntityExists, StringLength},
    FieldValidator, ValidationError, ValidationResultBuilder, Validator
  }
};
use uuid::Uuid;

use crate::{
  models::dtos::{CommentDto, CreateCommentDto},
  repos::{CommentRepository, RepositoryError}
};

use super::traits::{
  self, CreateCommentError, DeleteCommentError, GetCommentByIdError, GetCommentsByThreadIdError
};

pub struct CommentService {
  comment_repo:             Arc<dyn CommentRepository>,
  create_comment_validator: CreateCommentValidator
}

impl CommentService {
  pub fn new(
    comment_repo: Arc<dyn CommentRepository>,
    mediator_producer: Arc<MediatorProducer>
  ) -> Self {
    Self {
      comment_repo,
      create_comment_validator: CreateCommentValidator::new(mediator_producer)
    }
  }
}

#[async_trait]
impl traits::CommentService for CommentService {
  async fn get_comment_by_id(&self, id: Uuid) -> Result<CommentDto, GetCommentByIdError> {
    Ok(self.comment_repo.get_comment_by_id(id).await?.into())
  }

  async fn get_comments_by_thread_id(
    &self,
    thread_id: Uuid,
    pagination: Pagination<DateTime<Utc>>
  ) -> Result<Page<CommentDto, DateTime<Utc>>, GetCommentsByThreadIdError> {
    Ok(
      self
        .comment_repo
        .get_comments_by_thread_id(thread_id, pagination)
        .await?
        .inner_into()
    )
  }

  async fn create_comment(
    &self,
    dto: CreateCommentDto
  ) -> Result<Result<CommentDto, ValidationError>, CreateCommentError> {
    if let Err(e) = self.create_comment_validator.validate(&dto).await {
      return Ok(Err(e));
    }

    Ok(Ok(
      self
        .comment_repo
        .create_comment(dto.thread_id, dto.user_id, dto.text)
        .await?
        .into()
    ))
  }

  async fn delete_comment(&self, comment_id: Uuid) -> Result<(), DeleteCommentError> {
    Ok(self.comment_repo.delete_comment(comment_id).await?)
  }
}

impl From<RepositoryError> for GetCommentByIdError {
  fn from(error: RepositoryError) -> Self {
    match &error {
      RepositoryError::NotFound { key, .. } => {
        Self::NotFound {
          comment_id: key.clone(),
          source:     error.into()
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

impl From<RepositoryError> for GetCommentsByThreadIdError {
  fn from(error: RepositoryError) -> Self {
    GetCommentsByThreadIdError::Unknown {
      source: error.into()
    }
  }
}

impl From<RepositoryError> for CreateCommentError {
  fn from(error: RepositoryError) -> Self {
    CreateCommentError::Unknown {
      source: error.into()
    }
  }
}

impl From<RepositoryError> for DeleteCommentError {
  fn from(error: RepositoryError) -> Self {
    match &error {
      RepositoryError::NotFound { key, .. } => {
        Self::NotFound {
          comment_id: key.clone(),
          source:     error.into()
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

struct CreateCommentValidator {
  producer: Arc<MediatorProducer>
}

impl CreateCommentValidator {
  pub fn new(producer: Arc<MediatorProducer>) -> Self {
    Self { producer }
  }
}

#[async_trait]
impl Validator<CreateCommentDto> for CreateCommentValidator {
  async fn validate(&self, dto: &CreateCommentDto) -> Result<(), ValidationError> {
    ValidationResultBuilder::default()
      .field(FieldValidator::new("text", &dto.text).length_range(1..=2048))
      .field(
        FieldValidator::new("thread_id", &dto.thread_id)
          .entity_exists::<ThreadExistsQuery>(&self.producer)
          .await
      )
      .field(
        FieldValidator::new("user_id", &dto.user_id)
          .entity_exists::<UserExistsQuery>(&self.producer)
          .await
      )
      .finish()
  }
}
