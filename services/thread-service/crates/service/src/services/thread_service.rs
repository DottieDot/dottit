use std::sync::Arc;

use async_trait::async_trait;
use model::models::{PagedResult, Pagination, Thread, UuidStr};
use shared_service::{events::ThreadDeletedEvent, messaging::EventBus};

use crate::repos::{RepositoryError, ThreadRepository};

use super::traits::{
  self, CreateThreadError, DeleteThreadError, GetThreadByIdError, GetThreadsByBoardError
};

#[derive(Clone)]
pub struct ThreadService {
  thread_repo: Arc<dyn ThreadRepository>,
  event_bus:   Arc<EventBus>
}

impl ThreadService {
  pub fn new(thread_repo: Arc<dyn ThreadRepository>, event_bus: Arc<EventBus>) -> Self {
    Self {
      thread_repo,
      event_bus
    }
  }
}

#[async_trait]
impl traits::ThreadService for ThreadService {
  async fn get_thread_by_id(&self, id: &UuidStr) -> Result<Thread, GetThreadByIdError> {
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
    if text.is_none() && media.is_none() {
      return Err(CreateThreadError::NoConent);
    }

    Ok(
      self
        .thread_repo
        .create_thread(board, user, title, text, media)
        .await?
    )
  }

  async fn delete_thread(&self, thread_id: &UuidStr) -> Result<(), DeleteThreadError> {
    self.thread_repo.delete_thread(thread_id).await?;

    self
      .event_bus
      .publish(ThreadDeletedEvent {
        thread_id: thread_id.to_owned()
      })
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
