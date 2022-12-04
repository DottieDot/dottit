use std::sync::Arc;

use async_graphql::{Context, Object, Union};
use sea_orm::prelude::Uuid;
use shared_web::{gql::Unauthenticated, GqlContextExtensions};
use user_service_service::services::traits::{
  GetUserByIdError, GetUserByUsernameError, UserService
};

use super::user::User;

pub struct Query;

#[Object]
impl Query {
  async fn authenticated_user(
    &self,
    ctx: &Context<'_>
  ) -> Result<AuthenticatedUserResult, GetUserByIdError> {
    match ctx.authenticated_user() {
      Some(user) => {
        let user = self.get_user(ctx, user.user_id).await?;
        Ok(AuthenticatedUserResult::User(user))
      }
      None => {
        Ok(AuthenticatedUserResult::Unauthenticated(
          Unauthenticated::new()
        ))
      }
    }
  }

  async fn get_user(&self, ctx: &Context<'_>, user_id: Uuid) -> Result<User, GetUserByIdError> {
    self.find_user_by_id(ctx, user_id).await
  }

  async fn get_user_by_username(
    &self,
    ctx: &Context<'_>,
    username: String
  ) -> Result<Option<User>, GetUserByUsernameError> {
    let service = ctx.data::<Arc<dyn UserService>>().unwrap();

    let user = service.get_user_by_username(&username).await?;

    Ok(user.map(User::from))
  }

  #[graphql(entity)]
  async fn find_user_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Result<User, GetUserByIdError> {
    let service = ctx.data::<Arc<dyn UserService>>().unwrap();

    service.get_user_by_id(id).await.map(User::from)
  }
}

#[derive(Union)]
pub enum AuthenticatedUserResult {
  User(User),
  Unauthenticated(Unauthenticated)
}
