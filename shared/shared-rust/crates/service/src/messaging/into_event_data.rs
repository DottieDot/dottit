use serde::Serialize;
use thiserror::Error;

pub trait IntoEventData {
  fn into_event_data(&self) -> Result<Vec<u8>, IntoEventDataError>;
}

impl<T> IntoEventData for T
where
  T: Serialize
{
  fn into_event_data(&self) -> Result<Vec<u8>, IntoEventDataError> {
    Ok(serde_json::to_vec(self)?)
  }
}

#[derive(Error, Debug)]
#[error("{msg}")]
pub struct IntoEventDataError {
  pub msg:    String,
  pub source: Option<Box<dyn std::error::Error + Send + Sync>>
}

impl From<serde_json::Error> for IntoEventDataError {
  fn from(error: serde_json::Error) -> Self {
    Self {
      msg:    "failed to convert to json byte string".to_owned(),
      source: Some(error.into())
    }
  }
}
