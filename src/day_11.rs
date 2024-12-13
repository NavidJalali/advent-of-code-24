use std::{
  collections::HashMap,
  io::{Error, ErrorKind, Result},
};

use crate::fs::read_day;

fn input() -> Result<Vec<usize>> {
  match read_day(11)?
    .map(|line| {
      line
        .trim()
        .split(" ")
        .map(|s| s.parse::<usize>().expect("expected number"))
        .collect()
    })
    .next()
  {
    Some(vec) => Ok(vec),
    None => Err(Error::new(ErrorKind::Other, "no input")),
  }
}

fn width(n: usize) -> usize {
  n.checked_ilog10().map_or_else(|| 1, |x| x + 1) as usize
}

fn brute_force_step(vec: &Vec<usize>) -> Vec<usize> {
  vec
    .iter()
    .flat_map(|&stone| {
      if stone == 0 {
        vec![1]
      } else {
        let width = width(stone);
        if width % 2 == 0 {
          let half = width / 2;
          let left = stone / 10usize.pow(half as u32);
          let right = stone % 10usize.pow(half as u32);
          vec![left, right]
        } else {
          let new_stone = stone * 2024;
          vec![new_stone]
        }
      }
    })
    .collect::<Vec<_>>()
}

pub fn part_1() -> Result<usize> {
  let input = input()?;
  let iterations = 25;
  let result = (0..iterations).fold(input, |vec, _| brute_force_step(&vec));
  Ok(result.len())
}

type Stone = usize;
type Steps = usize;

fn count_steps(stone: Stone, steps: Steps, memo: &mut HashMap<(Stone, Steps), usize>) -> usize {
  if let Some(&result) = memo.get(&(stone, steps)) {
    return result;
  } else {
    let result = if steps == 0 {
      1
    } else if stone == 0 {
      count_steps(1, steps - 1, memo)
    } else {
      let width = width(stone);
      if width % 2 == 0 {
        let half = width / 2;
        let left = stone / 10usize.pow(half as u32);
        let right = stone % 10usize.pow(half as u32);
        count_steps(left, steps - 1, memo) + count_steps(right, steps - 1, memo)
      } else {
        count_steps(stone * 2024, steps - 1, memo)
      }
    };
    memo.insert((stone, steps), result);
    result
  }
}

pub fn part_2() -> Result<usize> {
  let input = input()?;
  let iterations = 75;
  let mut memo = HashMap::new();
  let result = input
    .iter()
    .map(|&stone| count_steps(stone, iterations, &mut memo))
    .sum();
  Ok(result)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_width() {
    assert_eq!(width(0), 1);
    assert_eq!(width(1), 1);
    assert_eq!(width(9), 1);
    assert_eq!(width(10), 2);
    assert_eq!(width(99), 2);
    assert_eq!(width(100), 3);
    assert_eq!(width(999), 3);
    assert_eq!(width(1000), 4);
    assert_eq!(width(9999), 4);
  }
}
