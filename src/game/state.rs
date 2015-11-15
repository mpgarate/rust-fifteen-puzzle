use game::bitboard::BitBoard;

pub struct GameState {
  board: BitBoard
}

impl GameState {
  pub fn new() -> GameState {
    GameState { board: BitBoard::new() }
  }

  pub fn is_solved(&self) -> bool {
    self.board.is_solved()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_state_is_solved() {
    let s: GameState = GameState::new();

    assert!(s.is_solved());
  }
}
