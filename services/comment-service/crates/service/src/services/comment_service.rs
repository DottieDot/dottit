use std::sync::Arc;

use async_trait::async_trait;
use model::models::{PagedResult, Pagination, UuidStr};

use crate::{
  models::dtos::{CommentDto, CreateCommentDto},
  repos::{CommentRepository, RepositoryError}
};

use super::traits::{
  self, CreateCommentError, DeleteCommentError, GetCommentByIdError, GetCommentsByThreadIdError
};

pub struct CommentService {
  comment_repo: Arc<dyn CommentRepository>
}

impl CommentService {
  pub fn new(comment_repo: Arc<dyn CommentRepository>) -> Self {
    Self { comment_repo }
  }
}

#[async_trait]
impl traits::CommentService for CommentService {
  async fn get_comment_by_id(&self, id: &UuidStr) -> Result<CommentDto, GetCommentByIdError> {
    Ok(self.comment_repo.get_comment_by_id(id).await?.into())
  }

  async fn get_comments_by_thread_id(
    &self,
    thread_id: &UuidStr,
    pagination: Pagination
  ) -> Result<PagedResult<CommentDto>, GetCommentsByThreadIdError> {
    Ok(
      self
        .comment_repo
        .get_comments_by_thread_id(thread_id, pagination)
        .await?
        .inner_into()
    )
  }

  async fn create_comment(&self, dto: CreateCommentDto) -> Result<CommentDto, CreateCommentError> {
    Ok(
      self
        .comment_repo
        .create_comment(dto.thread_id, dto.user, dto.text)
        .await?
        .into()
    )
  }

  async fn delete_comment(&self, comment_id: &UuidStr) -> Result<(), DeleteCommentError> {
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
