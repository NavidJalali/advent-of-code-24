use std::{
  collections::{HashMap, HashSet},
  io,
};

use crate::fs::read_day;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn turn_right(&self) -> Direction {
    match self {
      Direction::Up => Direction::Right,
      Direction::Down => Direction::Left,
      Direction::Left => Direction::Up,
      Direction::Right => Direction::Down,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(usize, usize);

impl Position {
  fn step(&self, direction: &Direction) -> Option<Position> {
    let Position(x, y) = self;
    match direction {
      Direction::Up => {
        if *y > 0 {
          Some(Position(*x, *y - 1))
        } else {
          None
        }
      }
      Direction::Down => Some(Position(*x, *y + 1)),
      Direction::Left => {
        if *x > 0 {
          Some(Position(*x - 1, *y))
        } else {
          None
        }
      }
      Direction::Right => Some(Position(*x + 1, *y)),
    }
  }
}

#[derive(Debug, Clone, Copy)]
enum Cell {
  Empty,
  Obstacle,
}

#[derive(Debug, Clone)]
struct Grid {
  cells: Vec<Vec<Cell>>,
}

impl Grid {
  fn get(&self, position: &Position) -> Option<&Cell> {
    let Position(x, y) = position;
    self.cells.get(*y).and_then(|row| row.get(*x))
  }

  fn set(&mut self, position: &Position, cell: Cell) {
    self.cells[position.1][position.0] = cell;
  }

  fn empty(width: usize, height: usize) -> Grid {
    let cells = vec![vec![Cell::Empty; width]; height];
    Grid { cells }
  }

  fn obstruct(&self, position: &Position) -> Grid {
    let mut new_grid = self.clone();
    new_grid.set(position, Cell::Obstacle);
    new_grid
  }
}

#[derive(Debug, Clone, Copy)]
struct Transform {
  position: Position,
  direction: Direction,
}

struct Input {
  grid: Grid,
  guard: Transform,
}

#[derive(Debug)]
enum Outcome {
  Loops {
    visited: HashMap<Position, HashSet<Direction>>,
  },
  Escapes {
    visited: HashMap<Position, HashSet<Direction>>,
  },
}

fn sim_guard(input: &Input) -> Outcome {
  let Input { grid, mut guard } = input;
  let mut guard_positions: HashMap<Position, HashSet<Direction>> = HashMap::new();
  let set = guard_positions.entry(guard.position).or_default();
  set.insert(guard.direction);

  loop {
    let next_position = guard.position.step(&guard.direction);
    let next_cell = next_position.and_then(|position| grid.get(&position));

    match (next_position, next_cell) {
      (Some(next_position), Some(Cell::Empty)) => {
        // check if we have been here before
        let set = guard_positions.entry(next_position).or_default();
        if set.contains(&guard.direction) {
          return Outcome::Loops {
            visited: guard_positions,
          };
        } else {
          set.insert(guard.direction);
        }

        guard.position = next_position;
      }
      (Some(_), Some(Cell::Obstacle)) => {
        let set = guard_positions.entry(guard.position).or_default();
        let new_direction = guard.direction.turn_right();
        if set.contains(&new_direction) {
          return Outcome::Loops {
            visited: guard_positions,
          };
        } else {
          set.insert(new_direction);
          guard.direction = new_direction;
        }
      }
      _ => {
        return Outcome::Escapes {
          visited: guard_positions,
        };
      }
    }
  }
}

fn input() -> io::Result<Input> {
  let lines = read_day(6)?
    .map(|line| line.trim().chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();
  let height = lines.len();
  let width = lines[0].len();
  let mut grid = Grid::empty(width, height);
  let mut guard = None;

  for (y, row) in lines.iter().enumerate() {
    for (x, cell) in row.iter().enumerate() {
      let position = Position(x, y);
      match cell {
        '.' => {} // its already empty
        '^' => {
          grid.set(&position, Cell::Empty);
          guard = Some(Transform {
            position,
            direction: Direction::Up,
          });
        }
        'v' => {
          grid.set(&position, Cell::Empty);
          guard = Some(Transform {
            position,
            direction: Direction::Down,
          });
        }
        '<' => {
          grid.set(&position, Cell::Empty);
          guard = Some(Transform {
            position,
            direction: Direction::Left,
          });
        }
        '>' => {
          grid.set(&position, Cell::Empty);
          guard = Some(Transform {
            position,
            direction: Direction::Right,
          });
        }
        '#' => grid.set(&position, Cell::Obstacle),
        _ => panic!("unexpected cell"),
      }
    }
  }

  Ok(Input {
    grid,
    guard: guard.unwrap(),
  })
}

pub fn part_1() -> io::Result<usize> {
  let input = input()?;

  match sim_guard(&input) {
    Outcome::Loops { .. } => {
      io::Result::Err(io::Error::new(io::ErrorKind::Other, "guard is in a loop"))
    }
    Outcome::Escapes { visited } => Ok(visited.len()),
  }
}

pub fn part_2() -> io::Result<usize> {
  let input = input()?;

  let mut reachable_by_guard = match sim_guard(&input) {
    Outcome::Loops { visited } => visited.keys().cloned().collect::<HashSet<_>>(),
    Outcome::Escapes { visited } => visited.keys().cloned().collect::<HashSet<_>>(),
  };

  reachable_by_guard.remove(&input.guard.position);

  let result = reachable_by_guard
    .into_iter()
    .map(|position| {
      sim_guard(&Input {
        grid: input.grid.obstruct(&position),
        guard: input.guard.clone(),
      })
    })
    .fold(0, |acc, outcome| match outcome {
      Outcome::Loops { .. } => acc + 1,
      Outcome::Escapes { .. } => acc,
    });

  Ok(result)
}
