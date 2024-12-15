use std::io::Result;

use crate::fs::read_day;

#[derive(Debug)]
struct Problem {
  a: (u64, u64),
  b: (u64, u64),
  p: (u64, u64),
}

impl Problem {
  fn new(ax: u64, ay: u64, bx: u64, by: u64, px: u64, py: u64) -> Self {
    Self {
      a: (ax, ay),
      b: (bx, by),
      p: (px, py),
    }
  }

  fn solve(&self) -> Option<Solution> {
    let (ax, ay) = self.a;
    let (bx, by) = self.b;
    let (px, py) = self.p;

    let (ax, bx) = (ax as f64, bx as f64);
    let (ay, by) = (ay as f64, by as f64);
    let (px, py) = (px as f64, py as f64);

    let n = (ay * px - ax * py) / (ay * bx - ax * by);
    let m = (px - n * bx) / ax;

    let solution_positives = n >= 0.0 && m >= 0.0;
    let solution_integers = n.fract() == 0.0 && m.fract() == 0.0;

    if solution_positives && solution_integers {
      Some(Solution::new(m as u64, n as u64))
    } else {
      None
    }
  }
}

#[derive(Debug)]
struct Solution {
  a_count: u64,
  b_count: u64,
}

impl Solution {
  fn new(a_count: u64, b_count: u64) -> Self {
    Self { a_count, b_count }
  }

  fn cost(&self) -> u64 {
    self.a_count * 3 + self.b_count
  }
}

fn parse_one(lines: &[String]) -> Problem {
  let a_line = &lines[0];
  let b_line = &lines[1];
  let p_line = &lines[2];

  let a_line = a_line
    .strip_prefix("Button A: X+")
    .expect("Expected line to begin with 'Button A: X+'")
    .split(", Y+")
    .map(|num| num.parse::<u64>().expect("Expected number"))
    .collect::<Vec<_>>();

  let b_line = b_line
    .strip_prefix("Button B: X+")
    .expect("Expected line to begin with 'Button B: X+'")
    .split(", Y+")
    .map(|num| num.parse::<u64>().expect("Expected number"))
    .collect::<Vec<_>>();

  let p_line = p_line
    .strip_prefix("Prize: X=")
    .expect("Expected line to begin with 'Button P: X+'")
    .split(", Y=")
    .map(|num| num.parse::<u64>().expect("Expected number"))
    .collect::<Vec<_>>();

  Problem::new(
    a_line[0], a_line[1], b_line[0], b_line[1], p_line[0], p_line[1],
  )
}

fn input() -> Result<Vec<Problem>> {
  let lines = read_day(13)?
    .collect::<Vec<_>>()
    .split(|line| line.is_empty())
    .collect::<Vec<_>>()
    .into_iter()
    .map(|x| parse_one(x))
    .collect::<Vec<_>>();

  Ok(lines)
}

pub fn part_1() -> Result<u64> {
  let problems = input()?;

  let result = problems
    .iter()
    .filter_map(|problem| problem.solve())
    .map(|solution| solution.cost())
    .sum::<u64>();

  Ok(result)
}

pub fn part_2() -> Result<u64> {
  let problems = input()?;

  let result = problems
    .iter()
    .map(|problem| Problem {
      a: problem.a,
      b: problem.b,
      p: (problem.p.0 + 10000000000000, problem.p.1 + 10000000000000),
    })
    .filter_map(|problem| problem.solve())
    .map(|solution| solution.cost())
    .sum::<u64>();

  Ok(result)
}
