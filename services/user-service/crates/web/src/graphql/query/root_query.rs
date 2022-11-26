use std::sync::Arc;

use async_graphql::{Context, Object, ID};
use user_service_service::services::traits::{GetUserByIdError, UserService};

use super::user::User;

pub struct Query;

#[Object]
impl Query {
  async fn get_user(&self, ctx: &Context<'_>, user_id: ID) -> Result<User, GetUserByIdError> {
    self.find_user_by_id(ctx, user_id).await
  }

  #[graphql(entity)]
  async fn find_user_by_id(&self, ctx: &Context<'_>, id: ID) -> Result<User, GetUserByIdError> {
    let service = ctx.data::<Arc<dyn UserService>>().unwrap();

    service.get_user_by_id(&id).await.map(User::from)
  }
}
