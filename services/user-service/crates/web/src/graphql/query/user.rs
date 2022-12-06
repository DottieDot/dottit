use async_graphql::Object;
use sea_orm::prelude::Uuid;
use user_service_service::models::dtos::UserDto;

#[derive(Debug)]
pub struct User {
  user: UserDto
}

#[Object]
impl User {
  async fn id(&self) -> Uuid {
    self.user.id
  }

  async fn username(&self) -> &str {
    &self.user.username
  }
}

impl From<UserDto> for User {
  fn from(user: UserDto) -> Self {
    Self { user }
  }
}
