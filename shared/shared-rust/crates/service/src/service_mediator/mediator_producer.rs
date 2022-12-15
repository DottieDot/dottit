use thiserror::Error;
use tracing::{debug, info};

use crate::messaging::{FromEventData, FromEventDataError, ToEventData, ToEventDataError};

use super::MediatorQuery;

pub struct MediatorProducer {
  client:   reqwest::Client,
  base_url: String
}

impl MediatorProducer {
  pub fn new(base_url: impl Into<String>) -> Result<Self, reqwest::Error> {
    Ok(Self {
      client:   reqwest::ClientBuilder::new().build()?,
      base_url: base_url.into()
    })
  }

  #[tracing::instrument(skip_all)]
  pub async fn query<Q>(&self, query: Q) -> Result<Q::Response, MediatorProducerError>
  where
    Q: MediatorQuery + ToEventData,
    Q::Response: FromEventData
  {
    info!("Sending mediator query {}", Q::name());

    let url = format!("{}/query/{}", self.base_url, Q::name());
    let bytes = self
      .client
      .post(url)
      .body(query.to_event_data()?)
      .send()
      .await?
      .bytes()
      .await?;

    debug!("Mediator responded with {} bytes", bytes.len());

    Ok(Q::Response::from_event_data(&bytes)?)
  }
}

#[derive(Error, Debug)]
pub enum MediatorProducerError {
  #[error("Reqwest failed to complete request: {source}")]
  ReqwestError { source: reqwest::Error },

  #[error("Failed to parse event data: {source}")]
  FailedToParseResponse { source: FromEventDataError },

  #[error("Failed to convert query to event data: {source}")]
  FailedToCreateEventData { source: ToEventDataError }
}

impl From<reqwest::Error> for MediatorProducerError {
  fn from(error: reqwest::Error) -> Self {
    Self::ReqwestError { source: error }
  }
}

impl From<FromEventDataError> for MediatorProducerError {
  fn from(error: FromEventDataError) -> Self {
    Self::FailedToParseResponse { source: error }
  }
}

impl From<ToEventDataError> for MediatorProducerError {
  fn from(error: ToEventDataError) -> Self {
    Self::FailedToCreateEventData { source: error }
  }
}
