use game::state::GameState;
use game::position::MoveDirection;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct SolverState {
  game: GameState,
  steps: Vec<MoveDirection>,
}

impl SolverState {
  pub fn new(game: GameState, steps: Vec<MoveDirection>) -> SolverState {
    SolverState { game: game, steps: steps }
  }

  pub fn is_solved(&self) -> bool {
    self.game.is_solved()
  }

  pub fn steps_with(&self, direction: MoveDirection) -> Vec<MoveDirection> {
    let mut vm = self.steps.to_vec();
    vm.push(direction);
    let v = vm;
    v
  }
}

impl PartialEq for SolverState {
  fn eq(&self, other: &SolverState) -> bool {
    self.game == other.game && self.game == other.game
  }
}

impl Eq for SolverState {
}

impl PartialOrd<SolverState> for SolverState {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }

  fn lt(&self, other: &Self) -> bool { self.game.get_score() > other.game.get_score() }
  fn le(&self, other: &Self) -> bool { self.game.get_score() >= other.game.get_score() }
  fn gt(&self, other: &Self) -> bool { self.game.get_score() < other.game.get_score() }
  fn ge(&self, other: &Self) -> bool { self.game.get_score() <= other.game.get_score() }
}

impl Ord for SolverState {
  fn cmp(&self, other: &Self) -> Ordering {
    let s1 = self.game.get_score();
    let s2 = other.game.get_score();

    if s1 == s2 {
      Ordering::Equal
    } else if s1 > s2 {
      Ordering::Less
    } else {
      Ordering::Greater
    }
  }
}

#[allow(dead_code)]
pub fn solve(state: GameState) -> Option<Vec<MoveDirection>> {

  if state.is_solved() {
    return Some(vec![]);
  }

  let move_directions = [
    MoveDirection::Up,
    MoveDirection::Down,
    MoveDirection::Left,
    MoveDirection::Right,
  ];

  let mut board_state_heap = BinaryHeap::new();

  board_state_heap.push(
    SolverState::new(state, vec![])
  );

  let mut iterations = 0;

  loop {
    iterations += 1;

    if iterations % 100 == 0 {
      println!("{} iterations", iterations);
    }

    let current_state = match board_state_heap.pop() {
      Some(s) => s,
      None => break
    };

    if current_state.is_solved() {
      //println!("Solved!");

      for s in current_state.steps.iter() {
        //println!("step: {:?}", s);
      }

      return Some(current_state.steps);
    }

    for direction in &move_directions {
      match (direction, current_state.steps.last()) {
        (&MoveDirection::Left, Some(&MoveDirection::Right)) => continue,
        (&MoveDirection::Right, Some(&MoveDirection::Left)) => continue,
        (&MoveDirection::Up, Some(&MoveDirection::Down)) => continue,
        (&MoveDirection::Down, Some(&MoveDirection::Up)) => continue,
        _ => ()
      }

      match current_state.game.move_space(direction.clone()) {
        Some(g) => board_state_heap.push(SolverState::new(g,
                                                          current_state.steps_with(direction.clone()))),
        None => ()
      }
    }
  }

  None
}

#[cfg(test)]
mod tests {
  use super::*;
  use game::state::GameState;
  use game::position::MoveDirection;
  
  use std::cmp::Ordering;

  #[test]
  fn order_cmp() {
    let go1 = GameState::new().move_spaces(
      vec![
        MoveDirection::Left,
        MoveDirection::Left,
        MoveDirection::Up,
      ]
    );

    let go2 = GameState::new().move_spaces(
      vec![
        MoveDirection::Up,
      ]
    );

    let (g1, g2) = match (go1, go2) {
      (Some(g1), Some(g2)) => (g1, g2),
      _ => panic!(),
    };

    let s1 = SolverState {game: g1, steps: vec![]};
    let s2 = SolverState {game: g2, steps: vec![]};

    assert_eq!(s1.cmp(&s2), Ordering::Less);
  }

  #[test]
  fn solve_already_solved() {
    match solve(GameState::new()) {
      Some(path) => assert_eq!(0, path.len()),
      None => panic!()
    }
  }

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

    match game_state {
      Some(g) => match solve(g) {
        Some(path) => assert_eq!(3, path.len()),
        None => panic!(),
      },
      None => panic!(),
    };
  }
}
