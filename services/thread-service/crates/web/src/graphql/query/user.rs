use std::sync::Arc;

use async_graphql::{Context, Object};
use chrono::{DateTime, Utc};
use shared_service::model::Pagination;
use thread_service_service::services::traits::{GetThreadsByBoardError, ThreadService};
use uuid::Uuid;

use super::{Paged, Thread};

pub struct User {
  id: Uuid
}

impl User {
  pub fn new(id: Uuid) -> Self {
    Self { id }
  }
}

#[Object(extends)]
impl User {
  #[graphql(external)]
  pub async fn id(&self) -> Uuid {
    self.id
  }

  pub async fn threads(
    &self,
    ctx: &Context<'_>,
    first: DateTime<Utc>,
    count: u64
  ) -> Result<Paged<Thread, DateTime<Utc>>, GetThreadsByBoardError> {
    let service = ctx.data::<Arc<dyn ThreadService>>().unwrap();

    Ok(
      service
        .get_threads_by_user(self.id, Pagination { first, count })
        .await?
        .inner_into::<Thread>()
        .into()
    )
  }
}
