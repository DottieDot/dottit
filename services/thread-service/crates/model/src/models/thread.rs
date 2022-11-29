use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Thread {
  pub id:         Uuid,
  pub board_id:   Uuid,
  pub user_id:    Uuid,
  pub title:      String,
  pub text:       String,
  pub created_at: DateTime<Utc>
}
