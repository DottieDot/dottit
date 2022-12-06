use std::sync::Arc;

use crate::{
  model::thread::{self, Entity as DbThread},
  repo_error_from_db_error
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
  prelude::Uuid, ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait,
  QueryFilter, QueryOrder, QuerySelect
};
use shared_service::model::{Page, Pagination};
use thread_service_model::models::Thread;
use thread_service_service::repos::{self, RepositoryError, RepositoryResult};

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
  async fn get_thread_by_id(&self, id: Uuid) -> RepositoryResult<Thread> {
    let query_result = DbThread::find_by_id(id)
      .one(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    query_result.map(|thread| thread.into()).ok_or_else(|| {
      RepositoryError::NotFound {
        key:    id.to_string(),
        source: None
      }
    })
  }

  async fn get_threads_by_user(
    &self,
    user: Uuid,
    pagination: Pagination<DateTime<Utc>>
  ) -> RepositoryResult<Page<Thread, DateTime<Utc>>> {
    let query_result = DbThread::find()
      .filter(thread::Column::UserId.eq(user))
      .filter(thread::Column::CreatedAt.gte(pagination.first))
      .order_by_desc(thread::Column::CreatedAt)
      .limit(pagination.count + 1)
      .all(self.db.as_ref())
      .await;

    match query_result {
      Ok(threads) => {
        let next = if threads.len() as u64 == pagination.count + 1 {
          threads.last().map(|t| t.created_at)
        } else {
          None
        };
        let items = threads
          .into_iter()
          .map(Into::<Thread>::into)
          .take(pagination.count.try_into().unwrap())
          .collect::<Vec<_>>();

        Ok(Page {
          items,
          next: next.map(|dt| dt.into())
        })
      }
      Err(e) => {
        Err(RepositoryError::DatabaseError {
          source: Box::new(e)
        })
      }
    }
  }

  async fn get_threads_by_board(
    &self,
    board_id: Uuid,
    pagination: Pagination<DateTime<Utc>>
  ) -> RepositoryResult<Page<Thread, DateTime<Utc>>> {
    let query_result = DbThread::find()
      .filter(thread::Column::BoardId.eq(board_id))
      .filter(thread::Column::CreatedAt.gte(pagination.first))
      .order_by_desc(thread::Column::CreatedAt)
      .limit(pagination.count + 1)
      .all(self.db.as_ref())
      .await;

    match query_result {
      Ok(threads) => {
        let next = if threads.len() as u64 == pagination.count + 1 {
          threads.last().map(|t| t.created_at)
        } else {
          None
        };
        let items = threads
          .into_iter()
          .map(Into::<Thread>::into)
          .take(pagination.count.try_into().unwrap())
          .collect::<Vec<_>>();

        Ok(Page {
          items,
          next: next.map(|dt| dt.into())
        })
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
    board_id: Uuid,
    user_id: Uuid,
    title: String,
    text: String
  ) -> RepositoryResult<Thread> {
    let new_thread = thread::ActiveModel {
      id:         ActiveValue::NotSet,
      board_id:   ActiveValue::Set(board_id),
      user_id:    ActiveValue::Set(user_id),
      title:      ActiveValue::Set(title),
      text:       ActiveValue::Set(text),
      created_at: ActiveValue::NotSet
    };

    let query_result = new_thread
      .insert(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    Ok(query_result.into())
  }

  async fn delete_thread(&self, thread_id: Uuid) -> RepositoryResult<()> {
    DbThread::delete_by_id(thread_id)
      .exec(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    Ok(())
  }
}
