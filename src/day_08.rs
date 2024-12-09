use std::{
  collections::{HashMap, HashSet},
  io,
};

use crate::fs::read_day;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(usize, usize);

impl Position {
  fn plus(&self, x: i32, y: i32) -> Option<Position> {
    let Position(px, py) = self;
    let nx = *px as i32 + x;
    let ny = *py as i32 + y;
    if nx < 0 || ny < 0 {
      None
    } else {
      Some(Position(nx as usize, ny as usize))
    }
  }
}

type Out = usize;

struct In {
  grid: Grid,
  antennas: HashMap<char, Vec<Position>>,
}

#[derive(Debug, Clone, Copy)]
enum Cell {
  Empty,
  Antenna(char),
}

#[derive(Debug, Clone)]
struct Grid {
  cells: Vec<Vec<Cell>>,
}

impl Grid {
  fn in_bounds(&self, position: &Position) -> bool {
    position.0 < self.width() && position.1 < self.height()
  }

  fn get(&self, position: &Position) -> Cell {
    let Position(x, y) = position;
    self.cells[*y][*x]
  }

  fn set(&mut self, position: &Position, cell: Cell) {
    let Position(x, y) = position;
    self.cells[*y][*x] = cell;
  }

  fn width(&self) -> usize {
    self.cells[0].len()
  }

  fn height(&self) -> usize {
    self.cells.len()
  }

  fn pretty_print(&self) {
    for row in self.cells.iter() {
      for cell in row.iter() {
        match cell {
          Cell::Empty => print!("."),
          Cell::Antenna(c) => print!("{}", c),
        }
      }
      println!();
    }
  }
}

fn input() -> io::Result<In> {
  let mut antennas = HashMap::<char, Vec<Position>>::new();
  let cells = read_day(8)?
    .enumerate()
    .map(|(y, line)| {
      line
        .trim()
        .chars()
        .enumerate()
        .map(|(x, c)| {
          if c.is_alphanumeric() {
            antennas
              .entry(c)
              .or_insert_with(Vec::new)
              .push(Position(x, y));
            Cell::Antenna(c)
          } else if c == '.' {
            Cell::Empty
          } else {
            panic!("unexpected character")
          }
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  Ok(In {
    grid: Grid { cells },
    antennas,
  })
}

fn two_to_one_ratio(grid: &Grid, positions: &Vec<Position>) -> Vec<Position> {
  positions
    .into_iter()
    .flat_map(|a| positions.into_iter().map(|b| (*a, *b)))
    .filter(|(a, b)| *a != *b)
    .flat_map(|(a, b)| {
      let dx = a.0 as i32 - b.0 as i32;
      let dy = a.1 as i32 - b.1 as i32;

      let ux: Option<usize> = (a.0 as i32 + dx).try_into().ok();
      let uy: Option<usize> = (a.1 as i32 + dy).try_into().ok();

      let vx: Option<usize> = (b.0 as i32 - dx).try_into().ok();
      let vy: Option<usize> = (b.1 as i32 - dy).try_into().ok();

      let u = ux.and_then(|x| uy.map(|y| Position(x, y)));
      let v = vx.and_then(|x| vy.map(|y| Position(x, y)));

      [u, v]
        .iter()
        .filter_map(|&p| p)
        .filter(|p| grid.in_bounds(p))
        .collect::<Vec<_>>()
    })
    .collect()
}

fn colinear(grid: &Grid, positions: &Vec<Position>) -> Vec<Position> {
  positions
    .into_iter()
    .flat_map(|a| positions.into_iter().map(|b| (*a, *b)))
    .filter(|(a, b)| *a != *b)
    .flat_map(|(a, b)| {
      let dx = a.0 as i32 - b.0 as i32;
      let dy = a.1 as i32 - b.1 as i32;
      let mut points = Vec::new();

      // from a, keep adding dx, dy until out of bounds, in both directions
      let mut position = Some(a);
      while let Some(p) = position {
        points.push(p);
        position = p.plus(dx, dy).filter(|p| grid.in_bounds(p));
      }

      position = Some(a);
      while let Some(p) = position {
        points.push(p);
        position = p.plus(-dx, -dy).filter(|p| grid.in_bounds(p));
      }

      points
    })
    .collect::<Vec<_>>()
}

pub fn part_1() -> io::Result<Out> {
  let input = input()?;
  let In { grid, antennas } = input;

  let result = antennas
    .iter()
    .flat_map(|(_, positions)| two_to_one_ratio(&grid, positions))
    .collect::<HashSet<_>>();
  Ok(result.len())
}

pub fn part_2() -> io::Result<Out> {
  let input = input()?;
  let In { grid, antennas } = input;

  let result = antennas
    .iter()
    .flat_map(|(_, positions)| colinear(&grid, positions))
    .collect::<HashSet<_>>();

  Ok(result.len())
}
