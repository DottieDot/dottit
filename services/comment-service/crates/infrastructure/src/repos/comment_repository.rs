use crate::{
  model::{comment, comment::Entity as DbComment},
  repo_error_from_db_error
};
use async_trait::async_trait;
use comment_service_model::models::Comment;
use comment_service_service::repos::{
  CommentRepository as CommentRepoTrait, RepositoryError, RepositoryResult
};
use sea_orm::{
  prelude::{DateTimeUtc, Uuid},
  ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
  QueryOrder, QuerySelect
};
use shared_service::model::{Page, Pagination};
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
  async fn get_comment_by_id(&self, id: Uuid) -> RepositoryResult<Comment> {
    let query_result = DbComment::find_by_id(id)
      .one(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    query_result.map(|comment| comment.into()).ok_or_else(|| {
      RepositoryError::NotFound {
        key:    id.to_string(),
        source: None
      }
    })
  }

  async fn get_comments_by_thread_id(
    &self,
    thread_id: Uuid,
    pagination: Pagination<DateTimeUtc>
  ) -> RepositoryResult<Page<Comment, DateTimeUtc>> {
    let query_result = DbComment::find()
      .filter(comment::Column::ThreadId.eq(thread_id))
      .filter(comment::Column::CreatedAt.lte(pagination.first))
      .order_by_desc(comment::Column::CreatedAt)
      .limit(pagination.count + 1)
      .all(self.db.as_ref())
      .await;

    match query_result {
      Ok(comments) => {
        let next = if comments.len() as u64 == pagination.count + 1 {
          comments.last().map(|c| c.created_at)
        } else {
          None
        };
        let items = comments
          .into_iter()
          .map(Into::<Comment>::into)
          .take(pagination.count.try_into().unwrap())
          .collect::<Vec<_>>();

        Ok(Page {
          items,
          next: next.map(|dt| dt.into()),
          total: None
        })
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
    thread_id: Uuid,
    user_id: Uuid,
    text: String
  ) -> RepositoryResult<Comment> {
    let new_comment = comment::ActiveModel {
      id:         ActiveValue::NotSet,
      thread_id:  ActiveValue::Set(thread_id),
      user_id:    ActiveValue::Set(user_id),
      text:       ActiveValue::Set(text),
      created_at: ActiveValue::NotSet
    };

    let query_result = new_comment
      .insert(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    Ok(query_result.into())
  }

  async fn delete_comment(&self, comment_id: Uuid) -> RepositoryResult<()> {
    DbComment::delete_by_id(comment_id)
      .exec(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    Ok(())
  }
}
