pub trait MediatorQuery {
  type Response;

  fn name() -> &'static str;
}
