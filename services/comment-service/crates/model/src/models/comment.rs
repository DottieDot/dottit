use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Comment {
  pub id:         Uuid,
  pub thread_id:  Uuid,
  pub user_id:    Uuid,
  pub text:       String,
  pub created_at: DateTime<Utc>
}
