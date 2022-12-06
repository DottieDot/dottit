use board_service_model::models::Board;
use uuid::Uuid;

pub struct BoardDto {
  pub id:   Uuid,
  pub name: String
}

impl From<Board> for BoardDto {
  fn from(board: Board) -> Self {
    Self {
      id:   board.id,
      name: board.name
    }
  }
}
