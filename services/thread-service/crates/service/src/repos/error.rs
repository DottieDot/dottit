use std::error::Error as StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
  #[error("entity with key \"{key}\" not found")]
  Notfound {
    key:    String,
    source: Box<dyn StdError>
  },
  #[error("unknown database error: {source:?}")]
  Unknown { source: Box<dyn StdError> }
}
