use async_trait::async_trait;
use board_service_model::models::Board;
use board_service_service::repos::{
  BoardRepository as BoardRepoTrait, RepositoryError, RepositoryResult
};
use sea_orm::{
  prelude::Uuid, ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait,
  PaginatorTrait, QueryFilter, QueryOrder, QuerySelect
};
use shared_service::model::{Page, Pagination};
use std::sync::Arc;
use tokio::join;

use crate::{
  model::{board, board_moderator},
  repo_error_from_db_error
};

pub struct BoardRepository {
  db: Arc<DatabaseConnection>
}

impl BoardRepository {
  pub fn new(db: Arc<DatabaseConnection>) -> Self {
    Self { db }
  }
}

#[async_trait]
impl BoardRepoTrait for BoardRepository {
  async fn get_board_by_name(&self, name: &str) -> RepositoryResult<Option<Board>> {
    let board = board::Entity::find()
      .filter(board::Column::Name.eq(name))
      .one(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    Ok(board.map(Board::from))
  }

  async fn create_board(&self, name: String, creator_user_id: Uuid) -> RepositoryResult<Board> {
    let new_board = board::ActiveModel {
      id:   ActiveValue::NotSet,
      name: ActiveValue::Set(name)
    };

    let board = new_board
      .insert(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    let new_moderator = board_moderator::ActiveModel {
      board_id: ActiveValue::Set(board.id),
      user_id:  ActiveValue::Set(creator_user_id)
    };

    new_moderator
      .insert(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    Ok(board.into())
  }

  async fn get_board_by_id(&self, id: Uuid) -> RepositoryResult<Board> {
    let query_result = board::Entity::find_by_id(id)
      .one(self.db.as_ref())
      .await
      .map_err(repo_error_from_db_error)?;

    query_result.map(|board| board.into()).ok_or_else(|| {
      RepositoryError::NotFound {
        key:    id.to_string(),
        source: None
      }
    })
  }

  async fn get_boards(&self, pagination: Pagination<u64>) -> RepositoryResult<Page<Board, u64>> {
    let count_query = board::Entity::find().count(self.db.as_ref());
    let entity_query = board::Entity::find()
      .order_by_desc(board::Column::Name)
      .offset(pagination.first)
      .limit(pagination.count + 1)
      .all(self.db.as_ref());

    let (query_result, count_result) = join!(entity_query, count_query);

    let count = count_result.map_err(repo_error_from_db_error)?;

    match query_result {
      Ok(threads) => {
        let next = if threads.len() as u64 == pagination.count + 1 {
          Some(pagination.first + pagination.count)
        } else {
          None
        };
        let items = threads
          .into_iter()
          .map(Into::<Board>::into)
          .take(pagination.count.try_into().unwrap())
          .collect::<Vec<_>>();

        Ok(Page {
          items,
          next,
          total: Some(count as u64)
        })
      }
      Err(e) => {
        Err(RepositoryError::DatabaseError {
          source: Box::new(e)
        })
      }
    }
  }
}
