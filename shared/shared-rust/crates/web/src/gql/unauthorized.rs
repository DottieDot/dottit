use async_graphql::SimpleObject;

#[derive(SimpleObject)]
#[graphql(shareable)]
pub struct Unauthorized {
  message: String
}

impl Default for Unauthorized {
  fn default() -> Self {
    Self {
      message: "you do not have access this resource.".into()
    }
  }
}
