use std::sync::Arc;

use async_graphql::{Context, Object, Union};
use authorization_service_service::{database::traits::DbError, service::traits::AuthTokenService};
use shared_web::{gql::Unauthenticated, GqlContextExtensions};

pub struct Mutation;

#[Object]
impl Mutation {
  pub async fn logout_user(&self, ctx: &Context<'_>) -> Result<LogoutResult, DbError> {
    let service = ctx.data::<Arc<dyn AuthTokenService>>().unwrap();

    match ctx.authenticated_user() {
      Some(user) => {
        service.delete_token(&user.token).await?;
        Ok(LogoutResult::LoggedOut(LoggedOut))
      }
      None => Ok(LogoutResult::NotAuthenticated(Unauthenticated::default()))
    }
  }
}

#[derive(Union)]
pub enum LogoutResult {
  NotAuthenticated(Unauthenticated),
  LoggedOut(LoggedOut)
}

pub struct LoggedOut;

#[Object]
impl LoggedOut {
  pub async fn message(&self) -> &str {
    "logged out."
  }
}
