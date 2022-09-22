use async_trait::async_trait;
use model::models::{GuidStr, PagedResult, Pagination, Thread};

use crate::repos::{RepositoryError, ThreadRepository};

use super::traits::{
  self, CreateThreadError, DeleteThreadError, GetThreadByIdError, GetThreadsByBoardError
};

#[derive(Clone)]
pub struct ThreadService {
  thread_repo: Box<dyn ThreadRepository>
}

#[async_trait]
impl traits::ThreadService for ThreadService {
  async fn get_thread_by_id(&self, id: &GuidStr) -> Result<Thread, GetThreadByIdError> {
    Ok(self.thread_repo.get_thread_by_id(id).await?)
  }

  async fn get_threads_by_board(
    &self,
    board: &str,
    pagination: Pagination
  ) -> Result<PagedResult<Thread>, traits::GetThreadsByBoardError> {
    Ok(
      self
        .thread_repo
        .get_threads_by_board(board, pagination)
        .await?
    )
  }

  async fn create_thread(
    &self,
    board: &str,
    user: &str,
    title: &str,
    text: Option<&str>,
    media: Option<&str>
  ) -> Result<Thread, traits::CreateThreadError> {
    Ok(
      self
        .thread_repo
        .create_thread(board, user, title, text, media)
        .await?
    )
  }

  async fn delete_thread(&self, thread_id: &GuidStr) -> Result<(), DeleteThreadError> {
    Ok(self.thread_repo.delete_thread(thread_id).await?)
  }
}

impl From<RepositoryError> for GetThreadByIdError {
  fn from(error: RepositoryError) -> Self {
    match &error {
      RepositoryError::Notfound { key, .. } => {
        Self::NotFound {
          thread_id: key.clone(),
          source:    Box::new(error)
        }
      }
      _ => {
        Self::Unknwon {
          source: Box::new(error)
        }
      }
    }
  }
}

impl From<RepositoryError> for DeleteThreadError {
  fn from(error: RepositoryError) -> Self {
    match &error {
      RepositoryError::Notfound { key, .. } => {
        Self::NotFound {
          thread_id: key.clone(),
          source:    Box::new(error)
        }
      }
      _ => {
        Self::Unknwon {
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
