use std::io;

use crate::fs::read_day;

#[derive(Debug, Clone, Copy)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
  UpLeft,
  UpRight,
  DownLeft,
  DownRight,
}

impl Direction {
  fn move_position(&self, x: usize, y: usize) -> Option<(usize, usize)> {
    match self {
      Direction::Up => {
        if y > 0 {
          Some((x, y - 1))
        } else {
          None
        }
      }
      Direction::Down => Some((x, y + 1)),
      Direction::Left => {
        if x > 0 {
          Some((x - 1, y))
        } else {
          None
        }
      }
      Direction::Right => Some((x + 1, y)),
      Direction::UpLeft => {
        if x > 0 && y > 0 {
          Some((x - 1, y - 1))
        } else {
          None
        }
      }
      Direction::UpRight => {
        if y > 0 {
          Some((x + 1, y - 1))
        } else {
          None
        }
      }
      Direction::DownLeft => {
        if x > 0 {
          Some((x - 1, y + 1))
        } else {
          None
        }
      }
      Direction::DownRight => Some((x + 1, y + 1)),
    }
  }

  fn opposite(&self) -> Direction {
    match self {
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
      Direction::Left => Direction::Right,
      Direction::Right => Direction::Left,
      Direction::UpLeft => Direction::DownRight,
      Direction::UpRight => Direction::DownLeft,
      Direction::DownLeft => Direction::UpRight,
      Direction::DownRight => Direction::UpLeft,
    }
  }

  fn all() -> Vec<Direction> {
    vec![
      Direction::Up,
      Direction::Down,
      Direction::Left,
      Direction::Right,
      Direction::UpLeft,
      Direction::UpRight,
      Direction::DownLeft,
      Direction::DownRight,
    ]
  }
}

struct Grid {
  grid: Vec<Vec<char>>,
}

impl Grid {
  fn unsafe_get(&self, x: usize, y: usize) -> char {
    self.grid[y][x]
  }

  fn get(&self, x: usize, y: usize) -> Option<char> {
    self.grid.get(y).and_then(|row| row.get(x).copied())
  }

  fn in_bounds(&self, x: usize, y: usize) -> bool {
    x < self.width() && y < self.height()
  }

  fn width(&self) -> usize {
    self.grid.get(0).map_or(0, |row| row.len())
  }

  fn height(&self) -> usize {
    self.grid.len()
  }

  fn contains_texts_around(
    &self,
    x: usize,
    y: usize,
    a: &str,
    b: &str,
    direction: &Direction,
  ) -> bool {
    if !self.in_bounds(x, y) {
      return false;
    }

    let aa = format!("{}{}", self.unsafe_get(x, y), a);
    let bb = format!("{}{}", self.unsafe_get(x, y), b);

    (self.contains_text(x, y, &aa, direction)
      && self.contains_text(x, y, &bb, &direction.opposite()))
      || (self.contains_text(x, y, &bb, direction)
        && self.contains_text(x, y, &aa, &direction.opposite()))
  }

  fn contains_text(
    &self,
    x: usize,
    y: usize,
    text: impl Into<String>,
    direction: &Direction,
  ) -> bool {
    let mut position = if self.in_bounds(x, y) {
      Some((x, y))
    } else {
      None
    };

    text.into().chars().all(|c| match position {
      Some((x, y)) => {
        if self.get(x, y) == Some(c) {
          position = direction.move_position(x, y);
          true
        } else {
          false
        }
      }
      None => false,
    })
  }
}

pub fn part_1() -> io::Result<u32> {
  let grid = Grid {
    grid: read_day(4)?
      .map(|line| line.chars().collect::<Vec<_>>())
      .collect::<Vec<_>>(),
  };

  let result = (0..grid.width())
    .flat_map(|x| (0..grid.height()).map(move |y| (x, y)))
    .filter_map(|(x, y)| {
      if grid.unsafe_get(x, y) == 'X' {
        Some(
          Direction::all()
            .iter()
            .map(|direction| {
              if grid.contains_text(x, y, "XMAS", direction) {
                1
              } else {
                0
              }
            })
            .sum::<u32>(),
        )
      } else {
        None
      }
    })
    .sum::<u32>();

  Ok(result)
}

pub fn part_2() -> io::Result<u32> {
  let grid = Grid {
    grid: read_day(4)?
      .map(|line| line.chars().collect::<Vec<_>>())
      .collect::<Vec<_>>(),
  };

  let result = (0..grid.width())
    .flat_map(|x| (0..grid.height()).map(move |y| (x, y)))
    .filter_map(|(x, y)| {
      if grid.unsafe_get(x, y) == 'A' {
        let ul = grid.contains_texts_around(x, y, "M", "S", &Direction::UpLeft);
        let ur = grid.contains_texts_around(x, y, "M", "S", &Direction::UpRight);
        if ul && ur {
          Some(1)
        } else {
          None
        }
      } else {
        None
      }
    })
    .sum::<u32>();

  Ok(result)
}
