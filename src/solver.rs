use game::state::GameState;
use game::position::MoveDirection;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

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

  pub fn should_skip(&self, direction: MoveDirection) -> bool {
      match (direction, self.steps.last()) {
        (MoveDirection::Left, Some(&MoveDirection::Right)) => true,
        (MoveDirection::Right, Some(&MoveDirection::Left)) => true,
        (MoveDirection::Up, Some(&MoveDirection::Down)) => true,
        (MoveDirection::Down, Some(&MoveDirection::Up)) => true,
        _ => false
      }
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

pub fn depth_limited_dfs(state: SolverState, mut depth: u64) -> Option<Vec<MoveDirection>> {
  let move_directions = [
    MoveDirection::Left,
    MoveDirection::Right,
    MoveDirection::Up,
    MoveDirection::Down,
  ];

  if depth == 0 && state.game.is_solved() {
    println!("steps!");
    Some(state.steps)
  } else if depth > 0 {
    for m in &move_directions {

      match state.should_skip(m.clone()) {
        true => continue,
        false => ()
      }

      match state.game.move_space(m.clone()) {
        Some(gs) => {
          let result = depth_limited_dfs(
            SolverState::new(
              gs,
              state.steps_with(m.clone())
            ),
            depth - 1
          );

          match result {
            Some(steps) => { return Some(steps) },
            None => ()
          }
        }
        None => ()
      }
    }
    None
  } else {
    None
  }
}

pub fn solve_idfs(state: GameState) -> Option<Vec<MoveDirection>> {
  let mut depth = 0;

  loop {
    if depth % 10 == 0 {
      ()
    }

    println!("depth {}", depth);

    if (depth > 100000) {
      return None;
    }
    match depth_limited_dfs(
      SolverState::new(
        state.clone(),
        vec![],
      ),
      depth
    ) {
      Some(steps) => {
        println!("got steps!");
        return Some(steps);
      }
      None => ()
    }

    depth += 1
  }
}

#[allow(dead_code)]
pub fn solve_guided(state: GameState) -> Option<Vec<MoveDirection>> {

  if state.is_solved() {
    return Some(vec![]);
  }

  let move_directions = [
    MoveDirection::Up,
    MoveDirection::Down,
    MoveDirection::Left,
    MoveDirection::Right,
  ];

  let mut heap = BinaryHeap::new();

  heap.push(
    SolverState::new(state.clone(), vec![])
  );

  let mut visited = HashSet::new();
  visited.insert(state.clone());

  loop {

    let current_state = match heap.pop() {
      Some(s) => s,
      None => break
    };

    if current_state.is_solved() {
      println!("Solved!");

      for s in current_state.steps.iter() {
        println!("step: {:?}", s);
      }

      return Some(current_state.steps);
    } else {
      visited.insert(current_state.game.clone());
    }

    for direction in &move_directions {
      match current_state.should_skip(direction.clone()) {
        true => continue,
        false => ()
      }

      match current_state.game.move_space(direction.clone()) {
        Some(g) => {
          if !visited.contains(&g) {
            let s = SolverState::new(g, current_state.steps_with(direction.clone()));
            heap.push(s);
          }
        },
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
    match solve_guided(GameState::new()) {
      Some(path) => assert_eq!(0, path.len()),
      None => panic!()
    }
  }

  #[test]
  fn solve_simple_case() {
    println!("simple case...");
    let game_state = GameState::new()
      .move_spaces(
        vec![
          MoveDirection::Up,
          MoveDirection::Left,
          MoveDirection::Up,
        ]
      );

    match game_state {
      Some(g) => match solve_idfs(g) {
        Some(path) => assert_eq!(3, path.len()),
        None => panic!(),
      },
      None => panic!(),
    };
  }

  #[test]
  fn solve_medium_case() {
    let game_state = GameState::new()
      .move_spaces(
        vec![
          MoveDirection::Left,
          MoveDirection::Left,
          MoveDirection::Up,
          MoveDirection::Up,
          MoveDirection::Up,
          MoveDirection::Left,
          MoveDirection::Down,
          MoveDirection::Down,
          MoveDirection::Right,
          MoveDirection::Right,
          MoveDirection::Up,
          MoveDirection::Right,
          MoveDirection::Up,
        ]
      );

    match game_state {
      Some(g) => match solve_idfs(g) {
        Some(path) => assert_eq!(13, path.len()),
        None => panic!(),
      },
      None => panic!(),
    };
  }

  #[test]
  fn solve_hard_case() {
    let game_state = GameState::new()
      .move_spaces(
        vec![
          MoveDirection::Left,
          MoveDirection::Left,
          MoveDirection::Up,
          MoveDirection::Up,
          MoveDirection::Up,
          MoveDirection::Left,
          MoveDirection::Down,
          MoveDirection::Down,
          MoveDirection::Right,
          MoveDirection::Right,
          MoveDirection::Up,
          MoveDirection::Right,
          MoveDirection::Up,
          MoveDirection::Left,
          MoveDirection::Left,
          MoveDirection::Left,
          MoveDirection::Down,
          MoveDirection::Right,
          MoveDirection::Down,
          MoveDirection::Right,
        ]
      );

    match game_state {
      Some(g) => match solve_idfs(g) {
        Some(path) => assert_eq!(20, path.len()),
        None => panic!(),
      },
      None => panic!(),
    };
  }
}
