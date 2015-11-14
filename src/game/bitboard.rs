#[allow(dead_code)]
const BOARD_STATE_EMPTY: u64 = 0x123456789ABCDEF0;

#[allow(dead_code)]
pub struct BitBoard {
  data: u64,
}

#[allow(dead_code)]
impl BitBoard {
  fn new() -> BitBoard {
    BitBoard { data: BOARD_STATE_EMPTY }
  }

  fn is_solved(&self) -> bool {
    self.data == BOARD_STATE_EMPTY
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn constructs_with_empty_state() {
    let b: BitBoard = BitBoard::new();
    assert!(b.data == super::BOARD_STATE_EMPTY);
  }

  #[test]
  fn new_board_is_solved() {
    let b: BitBoard = BitBoard::new();
    assert!(b.is_solved());
  }
}
