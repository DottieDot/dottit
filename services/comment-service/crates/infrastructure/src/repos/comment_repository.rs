use crate::{
  model::{comment, comment::Entity as DbComment},
  repo_error_from_db_error
};
use async_trait::async_trait;
use model::models::{Comment, PagedResult, Pagination, UuidStr, UuidString};
use sea_orm::{
  prelude::Uuid, ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait,
  QueryFilter, QueryOrder, QuerySelect
};
use service::repos::{CommentRepository as CommentRepoTrait, RepositoryError, RepositoryResult};
use std::sync::Arc;

pub struct CommentRepository {
  db: Arc<DatabaseConnection>
}

impl CommentRepository {
  pub fn new(db: Arc<DatabaseConnection>) -> Self {
    Self { db }
  }
}

#[async_trait]
impl CommentRepoTrait for CommentRepository {
  async fn get_comment_by_id(&self, id: &UuidStr) -> RepositoryResult<Comment> {
    let uuid = Uuid::parse_str(id).unwrap();

    let query_result = DbComment::find_by_id(uuid)
      .one(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    query_result.map(|comment| comment.into()).ok_or_else(|| {
      RepositoryError::NotFound {
        key:    id.to_owned(),
        source: None
      }
    })
  }

  async fn get_comments_by_thread_id(
    &self,
    thread_id: &UuidStr,
    pagination: Pagination
  ) -> RepositoryResult<PagedResult<Comment>> {
    let thread_uuid = Uuid::parse_str(thread_id).unwrap();

    let query_result = DbComment::find()
      .filter(comment::Column::ThreadId.eq(thread_uuid))
      .order_by_desc(comment::Column::Score)
      .offset(pagination.first)
      .limit(pagination.count + 1)
      .all(self.db.as_ref())
      .await;

    match query_result {
      Ok(comments) => {
        let next = pagination.next(comments.len());
        let items = comments
          .into_iter()
          .map(Into::<Comment>::into)
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

  async fn create_comment(
    &self,
    thread_id: UuidString,
    user: String,
    text: String
  ) -> RepositoryResult<Comment> {
    let thread_uuid = Uuid::parse_str(&thread_id).unwrap();

    let new_comment = comment::ActiveModel {
      id:        ActiveValue::NotSet,
      thread_id: ActiveValue::Set(thread_uuid),
      user:      ActiveValue::Set(user),
      text:      ActiveValue::Set(text),
      score:     ActiveValue::NotSet
    };

    let query_result = new_comment
      .insert(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    Ok(query_result.into())
  }

  async fn delete_comment(&self, comment_id: &UuidStr) -> RepositoryResult<()> {
    let uuid = Uuid::parse_str(comment_id).unwrap();

    DbComment::delete_by_id(uuid)
      .exec(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    Ok(())
  }
}
