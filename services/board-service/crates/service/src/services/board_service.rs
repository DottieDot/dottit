use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use lazy_static::lazy_static;
use regex::Regex;
use shared_service::{
  model::{Page, Pagination},
  validation::{
    validators::{MatchesRegex, StringLength},
    FieldValidator, ValidationError, ValidationResultBuilder, Validator
  }
};
use uuid::Uuid;

use crate::{
  models::dtos::{BoardDto, CreateBoardDto},
  repos::{BoardModeratorRepository, BoardRepository}
};

use super::traits;

pub struct BoardService {
  repo:                   Arc<dyn BoardRepository>,
  mod_repo:               Arc<dyn BoardModeratorRepository>,
  create_board_validator: CreateBoardValidator
}

impl BoardService {
  pub fn new(repo: Arc<dyn BoardRepository>, mod_repo: Arc<dyn BoardModeratorRepository>) -> Self {
    Self {
      repo,
      mod_repo,
      create_board_validator: CreateBoardValidator
    }
  }
}

#[async_trait]
impl traits::BoardService for BoardService {
  async fn get_board_by_name(&self, name: &str) -> anyhow::Result<Option<BoardDto>> {
    Ok(self.repo.get_board_by_name(name).await?.map(BoardDto::from))
  }

  async fn create_board(
    &self,
    user_id: Uuid,
    dto: CreateBoardDto
  ) -> anyhow::Result<Result<BoardDto, ValidationError>> {
    if let Err(e) = self.create_board_validator.validate(&dto).await {
      return Ok(Err(e));
    }

    let board = self
      .repo
      .create_board(dto.name, user_id)
      .await
      .context("error creating board")?;

    Ok(Ok(board.into()))
  }

  async fn get_board_by_id(&self, id: Uuid) -> anyhow::Result<BoardDto> {
    Ok(self.repo.get_board_by_id(id).await?.into())
  }

  async fn get_boards_moderated_by_user(&self, user: Uuid) -> anyhow::Result<Vec<BoardDto>> {
    let boards = self.mod_repo.get_boards_moderated_by_user(user).await?;

    Ok(boards.into_iter().map(BoardDto::from).collect())
  }

  async fn get_board_moderators(&self, board: Uuid) -> anyhow::Result<Vec<Uuid>> {
    Ok(self.mod_repo.get_board_moderators(board).await?)
  }

  async fn get_boards(
    &self,
    pagination: Pagination<u64>
  ) -> anyhow::Result<Page<BoardDto, u64>> {
    Ok(
      self
        .repo
        .get_boards(pagination)
        .await?
        .inner_into::<BoardDto>()
    )
  }
}

struct CreateBoardValidator;

#[async_trait]
impl Validator<CreateBoardDto> for CreateBoardValidator {
  async fn validate(&self, dto: &CreateBoardDto) -> Result<(), ValidationError> {
    lazy_static! {
      static ref ALPHANUMERIC_RE: Regex = Regex::new("^[a-zA-Z0-9]*$").unwrap();
    }

    ValidationResultBuilder::default()
      .field(
        FieldValidator::new("name", &dto.name)
          .length_range(4..=24)
          .matches(&ALPHANUMERIC_RE, "must be alphanumeric.")
      )
      .finish()
  }
}
