use std::sync::Arc;

use async_graphql::{Context, Object, Union};
use shared_web::{
  gql::{AlreadyLoggedIn, ValidationError},
  GqlContextExtensions
};
use user_service_service::{
  models::dtos::{CreateUserDto, LoginDto},
  services::traits::{CreateUserError, LoginError, UserService}
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
  ) -> Result<CreateUserResult, CreateUserError> {
    let service = ctx.data::<Arc<dyn UserService>>().unwrap();
    if ctx.authenticated_user().is_some() {
      return Ok(CreateUserResult::AlreadyLoggedIn(AlreadyLoggedIn::default()));
    }

    let authenticated_user = service
      .create_user(CreateUserDto { username, password })
      .await?;

    match authenticated_user {
      Ok(user) => Ok(CreateUserResult::AuthenticatedUser(user.into())),
      Err(err) => Ok(CreateUserResult::ValidationError(err.into()))
    }
  }

  pub async fn login_user(
    &self,
    ctx: &Context<'_>,
    username: String,
    password: String
  ) -> Result<LoginUserResult, LoginError> {
    let service = ctx.data::<Arc<dyn UserService>>().unwrap();
    if ctx.authenticated_user().is_some() {
      return Ok(LoginUserResult::AlreadyLoggedIn(AlreadyLoggedIn::default()));
    }

    match service.login(LoginDto { username, password }).await? {
      Some(user) => Ok(LoginUserResult::AuthenticatedUser(user.into())),
      None => Ok(LoginUserResult::LoginFailed(LoginFailed))
    }
  }
}

#[derive(Union)]
pub enum CreateUserResult {
  AuthenticatedUser(AuthenticatedUser),
  ValidationError(ValidationError),
  AlreadyLoggedIn(AlreadyLoggedIn)
}

pub struct LoginFailed;

#[Object]
impl LoginFailed {
  pub async fn message(&self) -> &'static str {
    "failed to login."
  }
}

#[derive(Union)]
pub enum LoginUserResult {
  AuthenticatedUser(AuthenticatedUser),
  AlreadyLoggedIn(AlreadyLoggedIn),
  LoginFailed(LoginFailed)
}
