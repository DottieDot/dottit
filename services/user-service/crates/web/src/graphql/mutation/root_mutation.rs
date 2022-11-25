use std::sync::Arc;

use async_graphql::{Context, Object, ID};
use service::{
  models::dtos::CreateUserDto,
  services::traits::{CreateUserError, DeleteUserError, UserService}
};

use crate::graphql::query::AuthenticatedUser;

pub struct Mutation;

#[Object]
impl Mutation {
  pub async fn create_user(
    &self,
    ctx: &Context<'_>,
    username: String,
    password: String
  ) -> Result<AuthenticatedUser, CreateUserError> {
    let service = ctx.data::<Arc<dyn UserService>>().unwrap();

    service
      .create_user(CreateUserDto { username, password })
      .await
      .map(AuthenticatedUser::from)
  }

  pub async fn delete_user(&self, ctx: &Context<'_>, user_id: ID) -> Result<bool, DeleteUserError> {
    let service = ctx.data::<Arc<dyn UserService>>().unwrap();

    service.delete_user(&user_id).await.map(|_| true)
  }
}
