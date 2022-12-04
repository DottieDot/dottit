use std::{
  collections::HashMap,
  future::Future,
  pin::Pin,
  str::FromStr,
  sync::Arc,
  task::{
    Context,
    Poll::{Pending, Ready},
    Waker
  },
  time::Duration
};

use shared_service::{
  events::{MediatorQuery, MediatorResponse, MEDIATOR_EXCHANGE},
  messaging::{EventBus, EventMetadata, QueueMetadata, ToEventDataError}
};
use thiserror::Error;
use tokio::time::timeout;
use uuid::Uuid;

use crate::mediator_id::MediatorId;

pub struct QueryHandler {
  futures:     tokio::sync::Mutex<HashMap<Uuid, Arc<std::sync::Mutex<QueryFutureSharedState>>>>,
  event_bus:   Arc<EventBus>,
  mediator_id: Arc<MediatorId>
}

impl QueryHandler {
  pub fn new(event_bus: Arc<EventBus>, mediator_id: Arc<MediatorId>) -> Self {
    Self {
      futures: Default::default(),
      event_bus,
      mediator_id
    }
  }

  pub async fn handle_query(&self, query: String, body: Vec<u8>) -> Result<Vec<u8>, QueryError> {
    let query_id = Uuid::new_v4();

    println!("Querying {query} with {query_id}");

    self
      .event_bus
      .manual_publish(
        EventMetadata {
          exchange:        MEDIATOR_EXCHANGE,
          queue:           QueueMetadata {
            routing_key:  &query,
            options:      Default::default(),
            bind_options: Default::default()
          },
          consume_options: Default::default()
        },
        MediatorQuery {
          return_address: self.mediator_id.uuid.to_string(),
          query_id:       query_id.to_string(),
          data:           body
        }
      )
      .await?;

    let state = Arc::new(std::sync::Mutex::new(QueryFutureSharedState::new()));
    {
      self.futures.lock().await.insert(query_id, state.clone());
    }
    let future = QueryFuture::new(state.clone());

    match timeout(Duration::from_secs(1), future).await {
      Ok(data) => Ok(data),
      Err(_) => Err(QueryError::TimedOut)
    }
  }

  pub async fn handle_response(&self, response: MediatorResponse) {
    let state = {
      self
        .futures
        .lock()
        .await
        .remove(&Uuid::from_str(&response.query_id).unwrap())
        .expect("response received for non-existent query")
    };

    println!("Response received for {}", response.query_id);

    state.lock().unwrap().success(response.data);
  }
}

pub struct QueryFutureSharedState {
  result: Option<Vec<u8>>,
  waker:  Option<Waker>
}

impl QueryFutureSharedState {
  pub fn new() -> Self {
    Self {
      result: None,
      waker:  None
    }
  }

  pub fn success(&mut self, data: Vec<u8>) {
    self.result = Some(data);
    self
      .waker
      .take()
      .expect("success called on QueryFutureSharedState without a waker")
      .wake();
  }
}

pub struct QueryFuture {
  shared_state: Arc<std::sync::Mutex<QueryFutureSharedState>>
}

impl QueryFuture {
  pub fn new(shared_state: Arc<std::sync::Mutex<QueryFutureSharedState>>) -> Self {
    Self { shared_state }
  }
}

impl Future for QueryFuture {
  type Output = Vec<u8>;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> std::task::Poll<Self::Output> {
    let shared_state = &mut self.shared_state.lock().unwrap();
    match shared_state.result.take() {
      Some(data) => Ready(data),
      None => {
        shared_state.waker = Some(cx.waker().clone());
        Pending
      }
    }
  }
}

#[derive(Error, Debug)]
pub enum QueryError {
  #[error("the query timed out")]
  TimedOut,
  #[error("something failed within the mediator")]
  InternalError(#[from] Option<Box<dyn std::error::Error>>)
}

impl From<ToEventDataError> for QueryError {
  fn from(error: ToEventDataError) -> Self {
    Self::InternalError(Some(error.into()))
  }
}
