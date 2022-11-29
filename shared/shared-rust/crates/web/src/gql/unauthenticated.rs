use async_graphql::Object;

pub struct Unauthenticated;

#[Object]
impl Unauthenticated {
  pub async fn message(&self) -> &'static str {
    "you have to be logged in to access this resource."
  }
}
