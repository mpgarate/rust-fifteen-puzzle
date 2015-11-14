#[allow(dead_code)]
const MAX_ROW_INDEX: u8 = 3;
#[allow(dead_code)]
const MAX_COL_INDEX: u8 = 3;

#[allow(dead_code)]
pub struct Position {
  row: u8,
  col: u8,
}

#[allow(dead_code)]
impl Position {
  pub fn new(row: u8, col: u8) -> Position {
    if row > MAX_ROW_INDEX || col > MAX_COL_INDEX {
      panic!("Invalid argument for position");
    }

    Position { row: row, col: col }
  }

  pub fn to_offset(&self) -> u8 {
    let index = (self.row * 4) + self.col;
    4 * (15 - index)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic]
  fn high_col_position_should_panic() {
      Position::new(0, 4);
  }

  #[test]
  #[should_panic]
  fn high_row_position_should_panic() {
      Position::new(141, 0);
  }
}
