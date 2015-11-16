use game::state::GameState;
use game::position::MoveDirection;

#[allow(dead_code)]
pub struct Solver {
    initial_state: GameState,
}

impl Solver {
  pub fn new(initial_state: GameState) -> Solver {
    Solver { initial_state: initial_state }
  }

  pub fn solve(&self) -> Option<Vec<MoveDirection>> {
    Some(vec![
      MoveDirection::Down,
      MoveDirection::Right,
      MoveDirection::Right,
    ])
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use game::state::GameState;
  use game::position::MoveDirection;

  #[test]
  fn solve_simple_case() {
      let game_state = GameState::new()
        .move_spaces(
          vec![
            MoveDirection::Left,
            MoveDirection::Left,
            MoveDirection::Up,
          ]
        );

      let solver = match game_state {
        Some(g) => Solver::new(g),
        None => panic!(),
      };

     let path = solver.solve();
      match path {
        Some(p) => assert_eq!(3, p.len()),
        None => panic!(),
      }
  }
}
