use std::{
  collections::{HashMap, HashSet},
  fmt::Debug,
  io, vec,
};

use crate::fs::read_day;

struct Grid {
  cells: Vec<Vec<u32>>,
  trailheads: Vec<Position>,
}

impl Debug for Grid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for row in &self.cells {
      for cell in row {
        let repr = if cell == &10 {
          '.'
        } else {
          std::char::from_digit(*cell, 10).unwrap()
        };
        write!(f, "{}", repr)?;
      }
      writeln!(f)?;
    }

    f.write_fmt(format_args!("trailheads: {:?}", self.trailheads))?;

    Ok(())
  }
}

impl Grid {
  fn in_bounds(&self, &Position(x, y): &Position) -> bool {
    y < self.cells.len() && x < self.cells[y].len()
  }

  fn get(&self, &Position(x, y): &Position) -> Option<u32> {
    self.cells.get(y).and_then(|row| row.get(x)).copied()
  }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Position(usize, usize);

impl Debug for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("[{},{}]", self.0, self.1))
  }
}

impl Position {
  fn up(&self, grid: &Grid) -> Option<Position> {
    let Position(x, y) = self;
    if *y > 0 {
      Some(Position(*x, *y - 1)).filter(|position| grid.in_bounds(position))
    } else {
      None
    }
  }

  fn down(&self, grid: &Grid) -> Option<Position> {
    let Position(x, y) = self;
    let new_position = Position(*x, *y + 1);
    Some(new_position).filter(|position| grid.in_bounds(position))
  }

  fn left(&self, grid: &Grid) -> Option<Position> {
    let Position(x, y) = self;
    if *x > 0 {
      Some(Position(*x - 1, *y)).filter(|position| grid.in_bounds(position))
    } else {
      None
    }
  }

  fn right(&self, grid: &Grid) -> Option<Position> {
    let Position(x, y) = self;
    let new_position = Position(*x + 1, *y);
    Some(new_position).filter(|position| grid.in_bounds(position))
  }

  fn dirs(&self, grid: &Grid) -> Vec<Position> {
    [
      self.up(grid),
      self.down(grid),
      self.left(grid),
      self.right(grid),
    ]
    .iter()
    .filter_map(|&position| position)
    .collect()
  }
}

fn input() -> io::Result<Grid> {
  let mut trailheads = Vec::new();
  let result = read_day(10)?
    .enumerate()
    .map(|(y, line)| {
      line
        .trim()
        .chars()
        .enumerate()
        .map(|(x, c)| {
          let c = c
            .to_digit(10)
            .or(if c == '.' { Some(10) } else { None })
            .expect("expected digit");
          if c == 0 {
            trailheads.push(Position(x, y));
          }
          c
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  Ok(Grid {
    cells: result,
    trailheads,
  })
}

struct Entry {
  position: Position,
  via: Vec<Position>,
}

fn dfs<'a>(
  grid: &Grid,
  position: &Position,
  unique_9s: &'a mut HashMap<Position, HashSet<Position>>,
) {
  let mut visited = HashSet::new();
  let mut stack: Vec<Entry> = vec![Entry {
    position: *position,
    via: Vec::new(),
  }];

  while let Some(Entry { position, via }) = stack.pop() {
    if visited.contains(&position) {
      continue;
    }

    visited.insert(position);

    let cell = grid.get(&position).expect("expected cell");

    if cell == 9 {
      // we have reached the end. cache the path
      for point in via.iter() {
        unique_9s.entry(*point).or_default().insert(position);
      }
    } else {
      let dirs = position.dirs(grid);
      let dirs = dirs
        .iter()
        .filter(|&point| grid.get(point) == Some(cell + 1))
        .collect::<Vec<_>>();

      dirs.iter().for_each(|&point| {
        let mut new_via = via.clone();
        new_via.push(position);
        stack.push(Entry {
          position: *point,
          via: new_via,
        });
      });
    }
  }
}

pub fn part_1() -> io::Result<usize> {
  let grid = input()?;
  let Grid { cells, trailheads } = grid;
  let mut unique_9s = HashMap::<Position, HashSet<Position>>::new();

  for trailhead in trailheads.iter() {
    dfs(
      &Grid {
        cells: cells.clone(),
        trailheads: Vec::new(),
      },
      &trailhead,
      &mut unique_9s,
    );
  }

  let result = trailheads
    .iter()
    .flat_map(|trailhead| unique_9s.get(trailhead).map(|s| s.len()))
    .sum();

  Ok(result)
}

fn unique_subtrails_from(
  grid: &Grid,
  position: &Position,
  cache: &mut HashMap<Position, HashSet<Vec<Position>>>,
) -> HashSet<Vec<Position>> {
  if let Some(subtrails) = cache.get(position) {
    subtrails.to_owned()
  } else {
    let cell = grid.get(position).expect("expected cell");

    if cell == 9 {
      let trailend = HashSet::from([vec![*position]]);
      cache.insert(*position, trailend.clone());
      trailend
    } else {
      let dirs = position.dirs(grid);
      let dirs = dirs
        .iter()
        .filter(|&point| grid.get(point) == Some(cell + 1))
        .collect::<Vec<_>>();

      let mut subtrails = HashSet::new();
      for dir in dirs {
        unique_subtrails_from(grid, dir, cache)
          .into_iter()
          .map(|mut subtrail| {
            subtrail.push(*position);
            subtrail
          })
          .for_each(|subtrail| {
            subtrails.insert(subtrail);
          });
      }

      cache.insert(*position, subtrails.clone());
      subtrails
    }
  }
}

pub fn part_2() -> io::Result<usize> {
  let grid = input()?;

  let mut cache = HashMap::<Position, HashSet<Vec<Position>>>::new();

  let result = grid
    .trailheads
    .iter()
    .map(|trailhead| unique_subtrails_from(&grid, trailhead, &mut cache).len())
    .sum();

  Ok(result)
}
