use uuid::Uuid;

use crate::service_mediator::MediatorQuery;

pub trait EntityExistsQuery: MediatorQuery {
  fn new_exists_query(uuid: Uuid) -> Self;

  fn entity_name() -> &'static str;
}

pub trait EntityExistsQueryResponse {
  fn exists(&self) -> bool;
}
