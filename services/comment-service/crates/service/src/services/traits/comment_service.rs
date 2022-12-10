use async_trait::async_trait;
use chrono::{DateTime, Utc};
use shared_service::{
  model::{Page, Pagination},
  validation::ValidationError
};
use std::error::Error as StdError;
use thiserror::Error;
use uuid::Uuid;

use crate::models::dtos::{CommentDto, CreateCommentDto};

#[async_trait]
pub trait CommentService: Send + Sync {
  async fn get_comment_by_id(&self, id: Uuid) -> Result<CommentDto, GetCommentByIdError>;

  async fn get_comments_by_thread_id(
    &self,
    thread_id: Uuid,
    pagination: Pagination<DateTime<Utc>>
  ) -> Result<Page<CommentDto, DateTime<Utc>>, GetCommentsByThreadIdError>;

  async fn create_comment(
    &self,
    dto: CreateCommentDto
  ) -> Result<Result<CommentDto, ValidationError>, CreateCommentError>;

  async fn delete_comment(&self, comment_id: Uuid) -> Result<(), DeleteCommentError>;
}

#[derive(Error, Debug)]
pub enum GetCommentByIdError {
  #[error("no comment with id \"{comment_id}\" could be found")]
  NotFound {
    comment_id: String,
    source:     Box<dyn StdError + Send + Sync>
  },
  #[error("an unknown error ocurred")]
  Unknown {
    source: Box<dyn StdError + Send + Sync>
  }
}

#[derive(Error, Debug)]
pub enum GetCommentsByThreadIdError {
  #[error("an unknown error ocurred")]
  Unknown {
    source: Box<dyn StdError + Send + Sync>
  }
}

#[derive(Error, Debug)]
pub enum CreateCommentError {
  #[error("an unknown error ocurred")]
  Unknown {
    source: Box<dyn StdError + Send + Sync>
  }
}

#[derive(Error, Debug)]
pub enum DeleteCommentError {
  #[error("no comment with id \"{comment_id}\" could be found")]
  NotFound {
    comment_id: String,
    source:     Box<dyn StdError + Send + Sync>
  },
  #[error("an unknown error ocurred")]
  Unknown {
    source: Box<dyn StdError + Send + Sync>
  }
}
