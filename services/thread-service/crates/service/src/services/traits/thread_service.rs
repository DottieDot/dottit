use async_trait::async_trait;
use dyn_clone::DynClone;
use model::models::{GuidStr, PagedResult, Pagination, Thread};
use std::error::Error as StdError;
use thiserror::Error;

#[async_trait]
pub trait ThreadService: Send + Sync + DynClone {
  async fn get_thread_by_id(&self, id: &GuidStr) -> Result<Thread, GetThreadByIdError>;

  async fn get_threads_by_board(
    &self,
    board: &str,
    pagination: Pagination
  ) -> Result<PagedResult<Thread>, GetThreadsByBoardError>;

  async fn create_thread(
    &self,
    board: &str,
    user: &str,
    title: &str,
    text: Option<&str>,
    media: Option<&str>
  ) -> Result<Thread, CreateThreadError>;

  async fn delete_thread(&self, thread_id: &GuidStr) -> Result<(), DeleteThreadError>;
}
dyn_clone::clone_trait_object!(ThreadService);

#[derive(Error, Debug)]
pub enum GetThreadByIdError {
  #[error("no thread with id \"{thread_id}\" could be found")]
  NotFound {
    thread_id: String,
    source:    Box<dyn StdError + Send + Sync>
  },
  #[error("an unknown error occured")]
  Unknwon {
    source: Box<dyn StdError + Send + Sync>
  }
}

#[derive(Error, Debug)]
pub enum GetThreadsByBoardError {
  #[error("an unknown error occured")]
  Unknwon {
    source: Box<dyn StdError + Send + Sync>
  }
}

#[derive(Error, Debug)]
pub enum CreateThreadError {
  #[error("an unknown error occured")]
  Unknwon {
    source: Box<dyn StdError + Send + Sync>
  }
}

#[derive(Error, Debug)]
pub enum DeleteThreadError {
  #[error("no thread with id \"{thread_id}\" could be found")]
  NotFound {
    thread_id: String,
    source:    Box<dyn StdError + Send + Sync>
  },
  #[error("an unknown error occured")]
  Unknwon {
    source: Box<dyn StdError + Send + Sync>
  }
}
