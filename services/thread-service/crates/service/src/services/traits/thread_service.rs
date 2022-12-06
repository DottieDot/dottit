use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dyn_clone::DynClone;
use shared_service::model::{Page, Pagination};
use std::error::Error as StdError;
use thiserror::Error;
use thread_service_model::models::Thread;
use uuid::Uuid;

#[async_trait]
pub trait ThreadService: Send + Sync + DynClone {
  async fn get_thread_by_id(&self, id: Uuid) -> Result<Thread, GetThreadByIdError>;

  async fn get_threads_by_board(
    &self,
    board_id: Uuid,
    pagination: Pagination<DateTime<Utc>>
  ) -> Result<Page<Thread, DateTime<Utc>>, GetThreadsByBoardError>;

  async fn get_threads_by_user(
    &self,
    user_id: Uuid,
    pagination: Pagination<DateTime<Utc>>
  ) -> Result<Page<Thread, DateTime<Utc>>, GetThreadsByBoardError>;

  async fn create_thread(
    &self,
    board: Uuid,
    user_id: Uuid,
    title: String,
    text: String
  ) -> Result<Thread, CreateThreadError>;

  async fn delete_thread(&self, thread_id: Uuid) -> Result<(), DeleteThreadError>;
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
  Unknown {
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
  #[error("no content provided")]
  NoConent,
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
  Unknown {
    source: Box<dyn StdError + Send + Sync>
  }
}
