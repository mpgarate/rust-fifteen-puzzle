use game::bitboard::BitBoard;
use game::position::Position;
use game::position::MoveDirection;

#[allow(dead_code)]
const DEFAULT_FREE_SPACE_ROW: u8 = 3;
#[allow(dead_code)]
const DEFAULT_FREE_SPACE_COL: u8 = 3;

#[allow(dead_code)]
pub struct GameState {
  board: BitBoard,
  free_space: Position,
}

#[allow(dead_code)]
impl GameState {
  pub fn new() -> GameState {
    GameState {
      board: BitBoard::new(),
      free_space: Position::new(DEFAULT_FREE_SPACE_ROW, DEFAULT_FREE_SPACE_COL),
    }
  }

  pub fn is_solved(&self) -> bool {
    self.board.is_solved()
  }

  pub fn move_space(&self, move_direction: MoveDirection) -> Option<GameState> {
    match self.free_space.get_neighbor(move_direction) {
      Some(position) => {
        Some(GameState {
          board: self.board.swap(self.free_space, position),
          free_space: position,
        })
      }
      _ => None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use game::position::Position;
  use game::position::MoveDirection;

  #[test]
  fn new_state_is_solved() {
    let s: GameState = GameState::new();

    assert!(s.is_solved());
  }

  #[test]
  fn move_space_valid_direction() {
    let s: Option<GameState> = GameState::new()
      .move_space(MoveDirection::Left);

    match s {
      Some(state) => {
        if state.free_space != Position::new(3, 2) {
          panic!()
        }
      }
      _ => panic!(),
    }
  }

  #[test]
  fn move_space_illegal_direction() {
    let s: Option<GameState> = GameState::new()
      .move_space(MoveDirection::Down);

    match s {
      Some(_) => panic!(),
      None => (),
    }
  }
}
