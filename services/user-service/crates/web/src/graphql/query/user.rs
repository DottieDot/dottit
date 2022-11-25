use async_graphql::{Object, ID};
use service::models::dtos::UserDto;

#[derive(Debug)]
pub struct User {
  user: UserDto
}

#[Object]
impl User {
  async fn id(&self) -> ID {
    ID::from(&self.user.id)
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
