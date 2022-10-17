use serde::de::DeserializeOwned;
use thiserror::Error;

pub trait FromEventData: Sized {
  fn from_event_data(data: &[u8]) -> Result<Self, FromEventDataError>;
}

impl<T> FromEventData for T
where
  T: DeserializeOwned
{
  fn from_event_data(data: &[u8]) -> Result<Self, FromEventDataError> {
    Ok(serde_json::from_slice(data)?)
  }
}

#[derive(Error, Debug)]
#[error("{msg}")]
pub struct FromEventDataError {
  pub msg:    String,
  pub source: Option<Box<dyn std::error::Error + Sync + Send>>
}

impl From<serde_json::Error> for FromEventDataError {
  fn from(error: serde_json::Error) -> Self {
    Self {
      msg:    "failed to deserialize event data".to_owned(),
      source: Some(error.into())
    }
  }
}
