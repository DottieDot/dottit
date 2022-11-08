use std::sync::Arc;

use crate::{
  model::{thread, Thread as DbThread},
  repo_error_from_db_error
};
use async_trait::async_trait;
use model::models::{PagedResult, Pagination, Thread, UuidStr};
use sea_orm::{
  prelude::Uuid, ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait,
  QueryFilter, QueryOrder, QuerySelect
};
use service::repos::{self, RepositoryError, RepositoryResult};

#[derive(Clone)]
pub struct ThreadRepository {
  db: Arc<DatabaseConnection>
}

impl ThreadRepository {
  pub fn new(db: Arc<DatabaseConnection>) -> Self {
    Self { db }
  }
}

#[async_trait]
impl repos::ThreadRepository for ThreadRepository {
  async fn get_thread_by_id(&self, id: &UuidStr) -> RepositoryResult<Thread> {
    let uuid = Uuid::parse_str(id).unwrap();

    let query_result = DbThread::find_by_id(uuid)
      .one(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    query_result.map(|thread| thread.into()).ok_or_else(|| {
      RepositoryError::NotFound {
        key:    id.to_owned(),
        source: None
      }
    })
  }

  async fn get_threads_by_board(
    &self,
    board: &str,
    pagination: Pagination
  ) -> RepositoryResult<PagedResult<Thread>> {
    let query_result = DbThread::find()
      .filter(thread::Column::Board.eq(board))
      .order_by_desc(thread::Column::Score)
      .offset(pagination.first)
      .limit(pagination.count + 1)
      .all(self.db.as_ref())
      .await;

    match query_result {
      Ok(threads) => {
        let next = pagination.next(threads.len());
        let items = threads
          .into_iter()
          .map(Into::<Thread>::into)
          .take(pagination.count.try_into().unwrap())
          .collect::<Vec<_>>();

        Ok(PagedResult { items, next })
      }
      Err(e) => {
        Err(RepositoryError::DatabaseError {
          source: Box::new(e)
        })
      }
    }
  }

  async fn create_thread(
    &self,
    board: &str,
    user: &str,
    title: &str,
    text: Option<&str>,
    media: Option<&str>
  ) -> RepositoryResult<Thread> {
    let new_thread = thread::ActiveModel {
      id:    ActiveValue::NotSet,
      board: ActiveValue::Set(board.to_owned()),
      user:  ActiveValue::Set(user.to_owned()),
      title: ActiveValue::Set(title.to_owned()),
      text:  ActiveValue::Set(text.map(|s| s.to_owned())),
      media: ActiveValue::Set(media.map(|s| s.to_owned())),
      score: ActiveValue::NotSet
    };

    let query_result = new_thread
      .insert(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    Ok(query_result.into())
  }

  async fn delete_thread(&self, thread_id: &UuidStr) -> RepositoryResult<()> {
    let uuid = Uuid::parse_str(thread_id).unwrap();

    DbThread::delete_by_id(uuid)
      .exec(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    Ok(())
  }
}
