use serde::Serialize;
use thiserror::Error;

pub trait ToEventData {
  fn to_event_data(&self) -> Result<Vec<u8>, ToEventDataError>;
}

impl<T> ToEventData for T
where
  T: Serialize
{
  fn to_event_data(&self) -> Result<Vec<u8>, ToEventDataError> {
    Ok(serde_json::to_vec(self)?)
  }
}

#[derive(Error, Debug)]
#[error("{msg}")]
pub struct ToEventDataError {
  pub msg:    String,
  pub source: Option<Box<dyn std::error::Error + Send + Sync>>
}

impl From<serde_json::Error> for ToEventDataError {
  fn from(error: serde_json::Error) -> Self {
    Self {
      msg:    "failed to convert to json byte string".to_owned(),
      source: Some(error.into())
    }
  }
}
