use async_graphql::SimpleObject;

#[derive(SimpleObject)]
#[graphql(shareable)]
pub struct Unauthenticated {
  message: String
}

impl Default for Unauthenticated {
  fn default() -> Self {
    Self {
      message: "you have to be logged in to access this resource.".into()
    }
  }
}
