use std::{collections::HashSet, hash::Hash, io::Result, iter};

use crate::fs::read_day;

fn input() -> Result<Vec<Vec<char>>> {
  Ok(
    read_day(12)?
      .map(|line| line.trim().chars().collect())
      .collect(),
  )
}

type Position = (usize, usize);

trait PositionExt {
  fn up(&self) -> Option<Position>;
  fn down(&self) -> Option<Position>;
  fn left(&self) -> Option<Position>;
  fn right(&self) -> Option<Position>;
}

impl PositionExt for Position {
  fn up(&self) -> Option<Position> {
    let (x, y) = self;
    if *y > 0 {
      Some((*x, *y - 1))
    } else {
      None
    }
  }

  fn down(&self) -> Option<Position> {
    let (x, y) = self;
    Some((*x, *y + 1))
  }

  fn left(&self) -> Option<Position> {
    let (x, y) = self;
    if *x > 0 {
      Some((*x - 1, *y))
    } else {
      None
    }
  }

  fn right(&self) -> Option<Position> {
    let (x, y) = self;
    Some((*x + 1, *y))
  }
}

trait GridExt {
  fn in_bounds(&self, position: Position) -> bool;
}

impl GridExt for Vec<Vec<char>> {
  fn in_bounds(&self, position: Position) -> bool {
    let (x, y) = position;
    y < self.len() && x < self[y].len()
  }
}

type Region = HashSet<Position>;

fn flood_fill(grid: &Vec<Vec<char>>, position: Position, region: &mut Region) {
  let (x_start, y_start) = position;
  let cell = grid[y_start][x_start];
  vec![
    position.up(),
    position.down(),
    position.left(),
    position.right(),
  ]
  .iter()
  .filter_map(|&position| position)
  .filter(|&(x, y)| grid.in_bounds((x, y)))
  .filter(|&(x, y)| grid[y][x] == cell)
  .for_each(|position| {
    if region.insert(position) {
      flood_fill(grid, position, region);
    }
  });
}

fn regions(grid: &Vec<Vec<char>>) -> Vec<Region> {
  let mut seen = HashSet::<Position>::new();
  let width = grid[0].len();
  let height = grid.len();

  let mut regions = Vec::<Region>::new();

  for y in 0..height {
    for x in 0..width {
      let position = (x, y);
      if seen.contains(&position) {
        continue;
      }

      let mut region = HashSet::<Position>::new();
      region.insert(position);
      flood_fill(grid, position, &mut region);
      seen.extend(region.clone());
      regions.push(region);
    }
  }

  regions
}

fn perimeter(region: &Region) -> usize {
  region.iter().fold(0, |acc, position| {
    let neighbor_in_region = [
      position.up(),
      position.down(),
      position.left(),
      position.right(),
    ]
    .iter()
    .filter_map(|&position| position)
    .filter(|&(x, y)| region.contains(&(x, y)))
    .count();
    acc + 4 - neighbor_in_region
  })
}

fn area(region: &Region) -> usize {
  region.len()
}

pub fn part_1() -> Result<usize> {
  let grid = input()?;
  let regions = regions(&grid);

  let result = regions
    .iter()
    .map(|region| area(region) * perimeter(region))
    .sum::<usize>();

  Ok(result)
}

fn dir(position: Position, origin: Position) -> (i32, i32) {
  let (x, y) = position;
  let (ox, oy) = origin;
  ((x as i32 - ox as i32), (y as i32 - oy as i32))
}

fn sum_wrt(pos_1: Position, pos_2: Position, origin: Position) -> Position {
  let dir1 = dir(pos_1, origin);
  let dir2 = dir(pos_2, origin);
  (
    (dir1.0 + dir2.0 + origin.0 as i32) as usize,
    (dir1.1 + dir2.1 + origin.1 as i32) as usize,
  )
}

fn corners(region: &HashSet<(usize, usize)>, grid: &Vec<Vec<char>>) -> usize {
  region
    .iter()
    .map(|position| {
      let clockwise_pairs = [
        (position.up(), position.right()),
        (position.right(), position.down()),
        (position.down(), position.left()),
        (position.left(), position.up()),
      ];

      // there are 2 types of corners
      // Y
      // X Y
      // Type 0 is where two consecutive neighbors are not in the region
      //
      // X Y
      // X X
      // Type 1 is when two consecutive neighbors are in the region but their "sum" is not
      let x = clockwise_pairs
        .into_iter()
        .map(|(u, v)| {
          (
            u.filter(|p| grid.in_bounds(*p)),
            v.filter(|p| grid.in_bounds(*p)),
          )
        })
        .fold(0usize, |acc, (u, v)| {
          let u_in_region = u.map(|p| region.contains(&p)).unwrap_or(false);
          let v_in_region = v.map(|p| region.contains(&p)).unwrap_or(false);

          if !u_in_region && !v_in_region {
            acc + 1
          } else if u_in_region
            && v_in_region
            && !region.contains(&sum_wrt(u.unwrap(), v.unwrap(), *position))
          {
            acc + 1
          } else {
            acc
          }
        });

      x
    })
    .sum::<usize>()
}

fn region_crop(region: &Region, grid: &Vec<Vec<char>>) -> char {
  let (px, py) = region.iter().next().unwrap();
  grid[*py][*px]
}

pub fn part_2() -> Result<usize> {
  let grid = input()?;
  let regions = regions(&grid);

  // count the corners
  let result = regions
    .iter()
    .map(|region| {
      let area = area(region);
      let corners = corners(region, &grid);
      area * corners
    })
    .sum();

  Ok(result)
}
