use crate::fs::read_day;
use std::{fmt::Debug, io, num::ParseIntError, str::FromStr};

struct Report {
  levels: Vec<Level>,
}

impl Debug for Report {
  // Report { 1, 2 ,3 }
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Report {{ ")?;
    for (i, level) in self.levels.iter().enumerate() {
      write!(f, "{}", level.0)?;
      if i < self.levels.len() - 1 {
        write!(f, ", ")?;
      }
    }
    write!(f, " }}")
  }
}

impl Report {
  fn safely_increasing(&self, min_by: u32, max_by: u32) -> bool {
    self
      .levels
      .iter()
      .zip(self.levels.iter().skip(1))
      .all(|(level, next_level)| {
        let increasing = level.0 < next_level.0;
        let diff = next_level.0.saturating_sub(level.0);
        increasing && diff >= min_by && diff <= max_by
      })
  }

  fn safely_decreasing(&self, min_by: u32, max_by: u32) -> bool {
    self
      .levels
      .iter()
      .zip(self.levels.iter().skip(1))
      .all(|(level, next_level)| {
        let decreasing = level.0 > next_level.0;
        let diff = level.0.saturating_sub(next_level.0);
        decreasing && diff >= min_by && diff <= max_by
      })
  }

  fn dampened(&self) -> bool {
    let mut found_any_ok = false;

    let mut consider_removing = |xs: &Vec<Level>, index: usize| {
      if !found_any_ok {
        let mut levels = xs.clone();
        levels.remove(index);
        let report = Report { levels };
        if report.is_safe() {
          found_any_ok = true;
        }
      }
    };

    consider_removing(&self.levels, 0);

    for index in 0..self.levels.len() - 1 {
      let diff = self.levels[index].0 as i32 - self.levels[index + 1].0 as i32;
      let abs_diff = diff.abs();

      // If the diff is not okay then either the left or right has to be removed
      if abs_diff < 1 || abs_diff > 3 {
        consider_removing(&self.levels, index);
        consider_removing(&self.levels, index + 1);
        break;
      }

      // Hill or valley case. Like 3, 10, 5 or 10, 3, 8. One has to be removed
      if index + 2 < self.levels.len() {
        let next_diff = self.levels[index + 1].0 as i32 - self.levels[index + 2].0 as i32;
        // to detect hill or valley, we check if the diffs have different signs
        if (diff > 0) != (next_diff > 0) {
          consider_removing(&self.levels, index);
          consider_removing(&self.levels, index + 1);
          consider_removing(&self.levels, index + 2);
          break;
        }
      }
    }

    found_any_ok
  }

  fn is_safe(&self) -> bool {
    self.safely_increasing(1, 3) || self.safely_decreasing(1, 3)
  }
}

#[derive(Debug, Clone, Copy)]
struct Level(u32);

impl FromStr for Level {
  type Err = ParseIntError;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    s.parse::<u32>().map(Level)
  }
}

fn read_input() -> io::Result<impl Iterator<Item = Report>> {
  let result = read_day(2)?
    .map(|line| {
      line
        .split(" ")
        .filter_map(|str| str.trim().parse::<Level>().ok())
        .collect::<Vec<Level>>()
    })
    .map(|levels| Report { levels });

  Ok(result)
}

pub fn part_1() -> io::Result<usize> {
  Ok(read_input()?.filter(|report| report.is_safe()).count())
}

pub fn part_2() -> io::Result<usize> {
  Ok(read_input()?.filter(|report| report.dampened()).count())
}
