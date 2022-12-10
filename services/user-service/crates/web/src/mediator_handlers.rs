use std::sync::Arc;

use shared_service::service_mediator::{
  queries::{UserExistsQuery, UserExistsQueryResponse},
  MediatorConsumer
};
use user_service_service::services::traits::{GetUserByIdError, UserService};

pub async fn register_mediator_handlers(
  consumer: Arc<MediatorConsumer>,
  user_service: Arc<dyn UserService>
) -> anyhow::Result<()> {
  consumer
    .subscribe(move |query: UserExistsQuery| {
      let service = user_service.clone();
      async move {
        match service.get_user_by_id(query.user_id).await {
          Ok(_) => Ok(UserExistsQueryResponse { exists: true }),
          Err(GetUserByIdError::NotFound { .. }) => Ok(UserExistsQueryResponse { exists: false }),
          Err(e) => Err(e.into())
        }
      }
    })
    .await;

  Ok(())
}
