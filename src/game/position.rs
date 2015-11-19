use std::fmt;

#[allow(dead_code)]
const MAX_ROW_INDEX: u8 = 3;
#[allow(dead_code)]
const MAX_COL_INDEX: u8 = 3;

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MoveDirection {
  Left,
  Right,
  Up,
  Down
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
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

  pub fn get_neighbor(&self, move_direction: MoveDirection) -> Option<Position> {
    match move_direction {
      MoveDirection::Left if self.col > 0 => {
        Some(Position { row: self.row, col: self.col - 1 })
      }
      MoveDirection::Right if self.col < MAX_COL_INDEX => {
        Some(Position { row: self.row, col: self.col + 1 })
      }
      MoveDirection::Up if self.row > 0 => {
        Some(Position { row: self.row - 1, col: self.col })
      }
      MoveDirection::Down if self.row < MAX_ROW_INDEX => {
        Some(Position { row: self.row + 1, col: self.col })
      }
      _ => None
    }
  }
}

impl PartialEq for Position {
  fn eq(&self, other: &Position) -> bool {
    self.col == other.col && self.row == other.row
  }
}

impl fmt::Debug for Position {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "row: {}, col: {}", self.row, self.col)
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

  #[test]
  fn get_valid_left_move() {
    let p: Position = Position::new(3, 3);
    let n: Option<Position> = p.get_neighbor(MoveDirection::Left);

    assert_eq!(Some(Position::new(3, 2)), n);
  }
}
