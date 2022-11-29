use axum::{
  http::{Request, StatusCode},
  middleware::Next,
  response::Response
};

use crate::AuthenticatedUser;

pub async fn auth<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
  let user = AuthenticatedUser::from_http_request(&req);

  req.extensions_mut().insert(user);

  Ok(next.run(req).await)
}
