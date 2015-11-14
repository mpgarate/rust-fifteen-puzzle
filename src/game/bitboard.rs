use game::position::Position;

#[allow(dead_code)]
const BOARD_STATE_EMPTY: u64 = 0x123456789ABCDEF0;

#[allow(dead_code)]
pub struct BitBoard {
  data: u64,
}

#[allow(dead_code)]
impl BitBoard {
  pub fn new() -> BitBoard {
    BitBoard { data: BOARD_STATE_EMPTY }
  }

  pub fn is_solved(&self) -> bool {
    self.data == BOARD_STATE_EMPTY
  }

  pub fn set(&self, pos: Position, value: u8) -> BitBoard {
    let offset = pos.to_offset() as u64;

    // reset value at position to zero
    let zeroed_data = self.data & !(0xF << offset);

    // update value at position to new value
    let updated_data = zeroed_data | ((value as u64) << (offset));

    BitBoard { data: updated_data }
  }

  pub fn get(&self, pos: Position) -> u8 {
    let offset = pos.to_offset();

    let masked_data = self.data & (0xF << offset);

    ((masked_data >> offset as u64) & 0xF) as u8
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use game::position::Position;

  #[test]
  fn new_bitboard_uses_empty_state() {
    let b: BitBoard = BitBoard::new();
    assert!(b.data == super::BOARD_STATE_EMPTY);
  }

  #[test]
  fn is_solved_true_for_new_board() {
    let b: BitBoard = BitBoard::new();
    assert!(b.is_solved());
  }

  #[test]
  fn set_value_at_valid_position() {
    let b: BitBoard = BitBoard::new()
      .set(
        Position::new(0, 3),
        7
      );

    assert_eq!(
      7,
      b.get(
        Position::new(0, 3)
      )
    );
  }

  #[test]
  fn set_values_at_valid_positions() {
    for value in 0..16 {
      for row in 0..4 {
        for col in 0..4 {

          let b = BitBoard::new()
            .set(
              Position::new(row, col),
              value
            );

          assert_eq!(
            value,
            b.get(Position::new(row, col))
          );
        }
      }
    }
  }
}
