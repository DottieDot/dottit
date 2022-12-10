use async_graphql::Object;
use uuid::Uuid;

pub struct User {
  pub id: Uuid
}

#[Object(extends)]
impl User {
  #[graphql(external)]
  async fn id(&self) -> Uuid {
    self.id
  }
}
