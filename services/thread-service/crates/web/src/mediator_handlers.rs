use std::sync::Arc;

use shared_service::service_mediator::{
  queries::{ThreadExistsQuery, ThreadExistsQueryResponse},
  MediatorConsumer
};
use thread_service_service::services::traits::{GetThreadByIdError, ThreadService};

pub async fn register_mediator_handlers(
  consumer: Arc<MediatorConsumer>,
  thread_service: Arc<dyn ThreadService>
) -> anyhow::Result<()> {
  consumer
    .subscribe(move |query: ThreadExistsQuery| {
      let service = thread_service.clone();
      async move {
        match service.get_thread_by_id(query.thread_id).await {
          Ok(_) => Ok(ThreadExistsQueryResponse { exists: true }),
          Err(GetThreadByIdError::NotFound { .. }) => {
            Ok(ThreadExistsQueryResponse { exists: false })
          }
          Err(e) => Err(e.into())
        }
      }
    })
    .await;

  Ok(())
}
