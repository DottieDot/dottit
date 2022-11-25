use async_graphql::Object;
use service::models::dtos::AuthenticatedUserDto;

use super::User;

pub struct AuthenticatedUser {
  dto: AuthenticatedUserDto
}

#[Object]
impl AuthenticatedUser {
  pub async fn api_token(&self) -> &str {
    &self.dto.api_token
  }

  pub async fn user(&self) -> User {
    self.dto.user.clone().into()
  }
}

impl From<AuthenticatedUserDto> for AuthenticatedUser {
  fn from(dto: AuthenticatedUserDto) -> Self {
    Self { dto }
  }
}
