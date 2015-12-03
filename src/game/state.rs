use game::bitboard::BitBoard;
use game::position::Position;
use game::position::MoveDirection;

use std::fmt;

#[allow(dead_code)]
const DEFAULT_FREE_SPACE_ROW: u8 = 3;
#[allow(dead_code)]
const DEFAULT_FREE_SPACE_COL: u8 = 3;

#[allow(dead_code)]
#[derive(Eq, Hash, Clone)]
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

  // TODO: consider a functional/recursive style
  pub fn move_spaces(&self, mut move_directions:  Vec<MoveDirection>) -> Option<GameState> {
    let d1 = move_directions.remove(0);
    let mut state = self.move_space(d1);

    for m in move_directions.into_iter() {
      state = match state {
        Some(s) => s.move_space(m),
        None => None,
      }
    }

    state
  }

  pub fn get_score(&self) -> u32 {
    let mut score = 0;

    for row in 0..4 {
      for col in 0..4 {
        let p = Position::new(row, col);

        if p == self.free_space {
          continue;
        }

        let val = self.board.get(p) - 1;

        let target_col = val % 4;
        let target_row = (val - target_col) / 4;
        
        let distance = (target_row as i32 - row as i32).abs() + (target_col as i32 - col as i32).abs();

        score += distance
      }
    }

    score as u32
  }
}

impl PartialEq for GameState {
  fn eq(&self, other: &GameState) -> bool {
    self.board == other.board && self.free_space == other.free_space
  }
}

impl fmt::Display for GameState {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.board)
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

  #[test]
  fn move_spaces() {
    let game_state = GameState::new()
      .move_spaces(
        vec![
          MoveDirection::Left,
          MoveDirection::Left,
          MoveDirection::Up,
        ]
      );

    let expected = match GameState::new().move_space(MoveDirection::Left) {
      Some(g1) => match g1.move_space(MoveDirection::Left) {
        Some(g2) => g2.move_space(MoveDirection::Up),
        None => panic!(),
      },
      None => panic!(),
    };

    assert!(expected == game_state);
  }

}
