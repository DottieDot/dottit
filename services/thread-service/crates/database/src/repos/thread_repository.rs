use diesel::{
  prelude::*,
  result::{Error::NotFound, QueryResult},
  QueryDsl
};
use model::models::{GuidStr, PagedResult, Pagination, Thread};
use service::repos::{self, RepositoryError, RepositoryResult};

use crate::{
  model::{DbThread, DbThreadInsertable},
  schema::threads,
  types::DbPool
};

pub struct ThreadRepository {
  db: DbPool
}

impl repos::ThreadRepository for ThreadRepository {
  fn get_thread_by_id(&self, id: &GuidStr) -> RepositoryResult<Thread> {
    let conn: &mut MysqlConnection = &mut self.db.get().unwrap();

    let query_result: QueryResult<DbThread> = threads::table
      .select(threads::all_columns)
      .filter(threads::id.eq(id))
      .first::<DbThread>(conn);

    match query_result {
      Ok(thread) => Ok(thread.into()),
      Err(error) => {
        match error {
          NotFound => {
            Err(RepositoryError::Notfound {
              key:    id.to_owned(),
              source: Box::new(error)
            })
          }
          _ => {
            Err(RepositoryError::Unknown {
              source: Box::new(error)
            })
          }
        }
      }
    }
  }

  fn get_threads_by_board(
    &self,
    board: &str,
    pagination: Pagination
  ) -> RepositoryResult<PagedResult<Thread>> {
    let conn: &mut MysqlConnection = &mut self.db.get().unwrap();

    let query_result: QueryResult<Vec<DbThread>> = threads::table
      .select(threads::all_columns)
      .filter(threads::board.eq(board))
      .offset(pagination.first.try_into().unwrap())
      .limit((pagination.count + 1).try_into().unwrap())
      .load::<DbThread>(conn);

    match query_result {
      Ok(threads) => {
        let next = pagination.next(threads.len());
        let items = threads
          .into_iter()
          .map(|t| Into::<Thread>::into(t))
          .take(pagination.count.try_into().unwrap())
          .collect::<Vec<_>>();

        Ok(PagedResult { items, next })
      }
      Err(e) => {
        Err(RepositoryError::Unknown {
          source: Box::new(e)
        })
      }
    }
  }

  fn create_thread<'a>(
    &self,
    board: &str,
    user: &str,
    text: &str,
    media: &str
  ) -> RepositoryResult<Thread> {
    let conn: &mut MysqlConnection = &mut self.db.get().unwrap();

    let new_thread = DbThreadInsertable {
      board,
      user,
      text,
      media
    };

    let query_result = diesel::insert_into(threads::table)
      .values(&new_thread)
      .get_result(conn);
  }
}
