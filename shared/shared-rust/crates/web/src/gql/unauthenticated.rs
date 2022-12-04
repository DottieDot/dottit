use async_graphql::SimpleObject;

#[derive(SimpleObject)]
#[graphql(shareable)]
pub struct Unauthenticated {
  message: String
}

impl Unauthenticated {
  pub fn new() -> Self {
    Self {
      message: "you have to be logged in to access this resource.".into()
    }
  }
}
