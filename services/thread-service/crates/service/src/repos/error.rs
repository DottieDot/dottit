use std::error::Error as StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
  #[error("entity with key \"{key}\" not found")]
  NotFound {
    key:    String,
    source: Option<Box<dyn StdError + Send + Sync>>
  },
  #[error("database error: {source:?}")]
  DatabaseError {
    source: Box<dyn StdError + Send + Sync>
  }
}
