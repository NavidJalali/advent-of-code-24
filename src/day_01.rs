use std::{collections::HashMap, io, result};

use crate::fs::read_day;

fn parse_number_pairs(lines: impl Iterator<Item = String>) -> (Vec<u32>, Vec<u32>) {
  lines
    .map(|line| {
      line
        .split(" ")
        .map(|str| str.trim())
        .filter_map(|str| {
          if str.is_empty() {
            None
          } else {
            str.parse::<u32>().ok()
          }
        })
        .collect::<Vec<_>>()
    })
    .filter_map(|vec| match vec.as_slice() {
      [a, b] => Some((*a, *b)),
      _ => None,
    })
    .unzip()
}

pub fn part_1() -> io::Result<u32> {
  let lines = read_day(1)?;

  let (mut first, mut second) = parse_number_pairs(lines);

  first.sort();
  second.sort();

  let result = first
    .iter()
    .zip(second.iter())
    .map(|(a, b)| if a > b { a - b } else { b - a })
    .sum::<u32>();

  Ok(result)
}

pub fn part_2() -> io::Result<u32> {
  let lines = read_day(1)?;

  let (first, second) = parse_number_pairs(lines);

  let second_frequency = second
    .iter()
    .fold(HashMap::<u32, u32>::new(), |mut acc, &num| {
      *acc.entry(num).or_default() += 1;
      acc
    });

  let result = first
    .iter()
    .map(|&num| {
      let frequency = second_frequency.get(&num).unwrap_or(&0);
      num * frequency
    })
    .sum();

  Ok(result)
}
