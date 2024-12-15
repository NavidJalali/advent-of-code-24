use std::{io, num::ParseIntError};

use crate::fs::read_day;

fn input() -> io::Result<Vec<(u64, Vec<u64>)>> {
  Ok(
    read_day(7)?
      .map(|line| {
        let parts = line.split(":").collect::<Vec<_>>();
        let parts = parts.as_slice();
        match parts {
          [before, after] => {
            let left = before.trim().parse::<u64>().expect("expected number");
            let right = after
              .trim()
              .split(" ")
              .map(|s| s.parse::<u64>())
              .collect::<Result<Vec<_>, ParseIntError>>()
              .expect("expected numbers");
            (left, right)
          }
          _ => panic!("unexpected line"),
        }
      })
      .collect::<Vec<_>>(),
  )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BinOp {
  Plus,
  Mult,
  Concat,
}

impl BinOp {
  fn permutations_part_1(size: usize) -> Vec<Vec<BinOp>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    BinOp::permutations_rec_part_1(size, &mut result, &mut current);
    result
  }

  fn permutations_rec_part_1(size: usize, result: &mut Vec<Vec<BinOp>>, current: &mut Vec<BinOp>) {
    if current.len() == size {
      result.push(current.clone());
      return;
    }

    for op in [BinOp::Plus, BinOp::Mult].iter() {
      current.push(*op);
      BinOp::permutations_rec_part_1(size, result, current);
      current.pop();
    }
  }

  fn permutations_part_2(size: usize) -> Vec<Vec<BinOp>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    BinOp::permutations_rec_part_2(size, &mut result, &mut current);
    result
  }

  fn permutations_rec_part_2(size: usize, result: &mut Vec<Vec<BinOp>>, current: &mut Vec<BinOp>) {
    if current.len() == size {
      result.push(current.clone());
      return;
    }

    for op in [BinOp::Plus, BinOp::Mult, BinOp::Concat].iter() {
      current.push(*op);
      BinOp::permutations_rec_part_2(size, result, current);
      current.pop();
    }
  }
}

fn evals_to(components: &[u64], ops: &[BinOp], mut target: u64) -> bool {
  let mut cursor = components.len() - 1;
  let mut current = components[cursor];
  let mut op = ops[cursor - 1];

  loop {
    match op {
      BinOp::Plus => {
        if target > current {
          target -= current;
          if cursor == 1 {
            return target == components[0];
          } else {
            cursor -= 1;
            current = components[cursor];
            op = ops[cursor - 1];
          }
        } else {
          return false;
        }
      }
      BinOp::Mult => {
        if target % current == 0 {
          target /= current;
          if cursor == 1 {
            return target == components[0];
          } else {
            cursor -= 1;
            current = components[cursor];
            op = ops[cursor - 1];
          }
        } else {
          return false;
        }
      }
      BinOp::Concat => {
        let width: u32 = current.ilog10() + 1;
        let mult = 10_u64.pow(width);
        let target_end = target % mult;
        if target_end == current {
          target /= mult;
          if cursor == 1 {
            return target == components[0];
          } else {
            cursor -= 1;
            current = components[cursor];
            op = ops[cursor - 1];
          }
        } else {
          return false;
        }
      }
    }
  }
}

pub fn part_1() -> io::Result<u64> {
  let input = input()?;
  let result = input
    .iter()
    .flat_map(|(total, components)| {
      BinOp::permutations_part_1(components.len() - 1)
        .iter()
        .find(|ops| evals_to(components, ops, *total))
        .map(|_| *total)
    })
    .sum();

  Ok(result)
}

pub fn part_2() -> io::Result<u64> {
  let input = input()?;
  let result = input
    .iter()
    .flat_map(|(total, components)| {
      BinOp::permutations_part_2(components.len() - 1)
        .iter()
        .find(|ops| evals_to(components, ops, *total))
        .map(|_| *total)
    })
    .sum();

  Ok(result)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_permutations_part_1() {
    let expected = vec![
      vec![BinOp::Plus, BinOp::Plus],
      vec![BinOp::Plus, BinOp::Mult],
      vec![BinOp::Mult, BinOp::Plus],
      vec![BinOp::Mult, BinOp::Mult],
    ];
    assert_eq!(BinOp::permutations_part_1(2), expected);
  }

  #[test]
  fn test_permutations_part_2() {
    let expected = vec![
      vec![BinOp::Plus, BinOp::Plus],
      vec![BinOp::Plus, BinOp::Mult],
      vec![BinOp::Plus, BinOp::Concat],
      vec![BinOp::Mult, BinOp::Plus],
      vec![BinOp::Mult, BinOp::Mult],
      vec![BinOp::Mult, BinOp::Concat],
      vec![BinOp::Concat, BinOp::Plus],
      vec![BinOp::Concat, BinOp::Mult],
      vec![BinOp::Concat, BinOp::Concat],
    ];
    assert_eq!(BinOp::permutations_part_2(2), expected);
  }

  #[test]
  fn test_part_1() {
    assert_eq!(part_1().unwrap(), 10741443549536);
  }

  fn test_part_2() {
    assert_eq!(part_2().unwrap(), 500335179214836);
  }

  #[test]
  fn test_evals() {
    let components = vec![2, 1, 209, 29, 84];
    let perm = vec![BinOp::Concat, BinOp::Concat, BinOp::Mult, BinOp::Plus];
    let target = 615145;
    assert!(evals_to(&components, &perm, target));
  }
}
