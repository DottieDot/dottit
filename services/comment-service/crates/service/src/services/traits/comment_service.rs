use async_trait::async_trait;
use model::models::{PagedResult, Pagination, UuidStr, UuidString};
use std::error::Error as StdError;
use thiserror::Error;

use crate::models::dtos::{CommentDto, CreateCommentDto};

#[async_trait]
pub trait CommentService: Send + Sync {
  async fn get_comment_by_id(&self, id: &UuidStr) -> Result<CommentDto, GetCommentByIdError>;

  async fn get_comments_by_thread_id(
    &self,
    thread_id: &UuidStr,
    pagination: Pagination
  ) -> Result<PagedResult<CommentDto>, GetCommentsByThreadIdError>;

  async fn create_comment(&self, dto: CreateCommentDto) -> Result<CommentDto, CreateCommentError>;

  async fn delete_comment(&self, comment_id: &UuidStr) -> Result<(), DeleteCommentError>;
}

#[derive(Error, Debug)]
pub enum GetCommentByIdError {
  #[error("no comment with id \"{comment_id}\" could be found")]
  NotFound {
    comment_id: UuidString,
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
    comment_id: UuidString,
    source:     Box<dyn StdError + Send + Sync>
  },
  #[error("an unknown error ocurred")]
  Unknown {
    source: Box<dyn StdError + Send + Sync>
  }
}
