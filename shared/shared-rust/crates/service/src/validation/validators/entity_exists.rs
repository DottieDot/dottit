use async_trait::async_trait;
use uuid::Uuid;

use crate::{
  messaging::{FromEventData, ToEventData},
  service_mediator::{
    queries::{EntityExistsQuery, EntityExistsQueryResponse},
    MediatorProducer, MediatorQuery
  },
  validation::FieldValidator
};

#[async_trait]
pub trait EntityExists: Sized {
  async fn entity_exists<T>(self, producer: &MediatorProducer) -> Self
  where
    T: MediatorQuery + ToEventData + EntityExistsQuery + Send + Sync,
    T::Response: FromEventData + EntityExistsQueryResponse + EntityExistsQueryResponse + Send;
}

#[async_trait]
impl<'a> EntityExists for FieldValidator<'a, Uuid> {
  async fn entity_exists<T>(mut self, producer: &MediatorProducer) -> Self
  where
    T: MediatorQuery + ToEventData + EntityExistsQuery + Send + Sync,
    T::Response: FromEventData + EntityExistsQueryResponse + EntityExistsQueryResponse + Send
  {
    let result = producer.query(T::new_exists_query(*self.value)).await;

    match result {
      Ok(response) if !response.exists() => {
        self.add_error(format!("{} does not exist.", T::entity_name()))
      }
      Err(_) => self.add_error(format!("unable to validate {} exists.", T::entity_name())),
      _ => {}
    }

    self
  }
}
