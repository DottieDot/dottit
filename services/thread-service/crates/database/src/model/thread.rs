use diesel::{Insertable, Queryable};
use model::models::Thread;

use crate::schema::threads;

#[derive(Queryable, Debug)]
pub struct DbThread {
  pub id:    String,
  pub board: String,
  pub user:  String,
  pub title: String,
  pub text:  Option<String>,
  pub media: Option<String>,
  pub score: i32
}

impl Into<Thread> for DbThread {
  fn into(self) -> Thread {
    todo!()
  }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = threads)]
pub struct DbThreadInsertable<'a> {
  pub board: &'a str,
  pub user:  &'a str,
  pub text:  &'a str,
  pub media: &'a str
}
