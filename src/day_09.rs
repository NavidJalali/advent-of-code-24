use std::{
  array,
  collections::{BTreeSet, HashMap},
  fmt::Debug,
  io, u8,
};

use crate::fs::read_day;

#[derive(Clone, Copy)]
enum Block {
  File { id: usize, head: usize, size: u8 },
  FreeSpace { head: usize, size: u8 },
}

impl Debug for Block {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Block::File { id, head, size } => {
        write!(f, "F{}[{},{}]", id, head, head + (*size as usize) - 1)
      }
      Block::FreeSpace { head, size } => {
        write!(f, "S[{},{}]", head, head + (*size as usize) - 1)
      }
    }
  }
}

impl Block {
  fn pretty_print(blocks: &Vec<Block>) -> String {
    blocks
      .iter()
      .map(|block| match block {
        Block::File { id, size, .. } => (0..*size)
          .map(move |_| format!("{}", id))
          .collect::<Vec<_>>()
          .concat(),
        Block::FreeSpace { size, .. } => (0..*size).map(move |_| ".").collect::<Vec<_>>().concat(),
      })
      .collect::<Vec<_>>()
      .concat()
  }
}

fn blocks_from_string(line: String) -> Vec<Block> {
  let mut id = 0;
  let mut head = 0;
  line
    .trim()
    .chars()
    .enumerate()
    .map(|(index, c)| {
      let size = c.to_digit(10).expect("expected digit") as u8;
      if index % 2 == 0 {
        let file = Block::File { id, head, size };
        head += size as usize;
        id += 1;
        file
      } else {
        let free_space = Block::FreeSpace { head, size };
        head += size as usize;
        free_space
      }
    })
    .collect::<Vec<_>>()
}

fn input() -> io::Result<String> {
  let line = read_day(9)?.map(|line| line.trim().to_string()).next();

  match line {
    Some(line) => Ok(line),
    None => io::Result::Err(io::Error::new(io::ErrorKind::Other, "no input")),
  }
}

fn block_compact(blocks: &mut Vec<Option<usize>>) {
  let mut left_index = 0;
  let mut right_index = blocks.len() - 1;

  while left_index < right_index {
    let left = blocks[left_index];
    let right = blocks[right_index];

    match left {
      Some(_) => {
        left_index += 1;
      }
      None => match right {
        Some(_) => {
          blocks[left_index] = blocks[right_index];
          blocks[right_index] = None;
          left_index += 1;
          right_index -= 1;
        }
        None => {
          right_index -= 1;
        }
      },
    }
  }
}

fn checksum(ids: &Vec<Option<usize>>) -> usize {
  ids
    .iter()
    .enumerate()
    .filter_map(|(index, &id)| id.map(|id| id * index))
    .sum()
}

pub fn part_1() -> io::Result<usize> {
  let input = blocks_from_string(input()?);
  let mut ids = input
    .iter()
    .flat_map(|block| match block {
      Block::File { id, size, .. } => vec![Some(*id); *size as usize],
      Block::FreeSpace { size, .. } => vec![None; *size as usize],
    })
    .collect::<Vec<_>>();
  block_compact(&mut ids);
  let result = checksum(&ids);
  Ok(result)
}

#[derive(Debug)]
struct State {
  blocks: Vec<Block>,
  // smallest index of an empty block that can fit a file of size i (the hole has size i or greater)
  holes: [BTreeSet<usize>; 10],
  files: HashMap<usize, usize>,
}

impl State {
  fn new(blocks: Vec<Block>) -> Self {
    // drop all the empties at the end, and remove 0 size holes
    let blocks = blocks
      .into_iter()
      .filter(|block| match block {
        Block::File { .. } => true,
        Block::FreeSpace { size, .. } => *size != 0,
      })
      .collect::<Vec<_>>();

    let mut holes = array::from_fn(|_| BTreeSet::new());

    let mut files = HashMap::new();

    for (index, block) in blocks.iter().enumerate() {
      match block {
        Block::File { id, .. } => {
          files.insert(*id, index);
        }
        Block::FreeSpace { size, .. } => {
          if *size != 0 {
            // this hole can fit a file of size i or smaller
            for i in 1..=*size as usize {
              holes[i].insert(index);
            }
          }
        }
      }
    }
    Self {
      blocks,
      holes,
      files,
    }
  }

  // attempt to move file to leftmost empty space that fits if possible
  fn attempt_move(&mut self, file_index: usize) -> usize {
    let file = &self.blocks[file_index];
    let (file_id, file_head, file_size) = match file {
      Block::File { id, head, size } => (*id, *head, *size),
      Block::FreeSpace { .. } => {
        return 0;
      }
    };

    if let Some(&hole_index) = self.holes[file_size as usize].first() {
      if hole_index > file_index {
        // the hole is to the right of the file. skip.
        return 0;
      }
      let hole = &self.blocks[hole_index];
      let (hole_head, hole_size) = match hole {
        Block::File { id, .. } => panic!("expected hole at {} got file {}", hole_index, id),
        Block::FreeSpace { head, size } => (*head, *size),
      };

      if hole_size == file_size {
        // the hole is the same size as the file. move the file to the hole.
        self.blocks[file_index] = Block::FreeSpace {
          head: file_head,
          size: file_size,
        };

        self.blocks[hole_index] = Block::File {
          id: file_id,
          head: hole_head,
          size: file_size,
        };

        for i in 1..10 as usize {
          self.holes[i].remove(&hole_index);
        }

        for i in 1..=hole_size as usize {
          self.holes[i].insert(file_index);
        }

        self.files.insert(file_id, hole_index);

        0
      } else if hole_size > file_size {
        // the hole is larger. Now we need to split the hole.
        self.blocks[file_index] = Block::FreeSpace {
          head: file_head,
          size: file_size,
        };

        self.blocks[hole_index] = Block::File {
          id: file_id,
          head: hole_head,
          size: file_size,
        };

        self.blocks.insert(
          hole_index + 1,
          Block::FreeSpace {
            head: hole_head + file_size as usize,
            size: hole_size - file_size,
          },
        );

        // This insert breaks the index of all the holes after the hole we just split.

        for i in 1..10 as usize {
          self.holes[i].remove(&hole_index);
        }

        for i in 1..=file_size as usize {
          self.holes[i].insert(file_index);
        }

        for i in 1..10 {
          self.holes[i] = self.holes[i]
            .iter()
            .map(|&index| if index > hole_index { index + 1 } else { index })
            .collect();
        }

        let new_hole_size: u8 = hole_size - file_size;

        for i in 1..=new_hole_size as usize {
          self.holes[i].insert(hole_index + 1);
        }

        for (_, index) in self.files.iter_mut() {
          if *index > hole_index {
            *index += 1;
          }
        }

        self.files.insert(file_id, hole_index);

        1
      } else {
        panic!("invariant violated")
      }
    } else {
      0
    }
  }
}

pub fn part_2() -> io::Result<usize> {
  let input = blocks_from_string(input()?);
  let mut state = State::new(input);
  let max_file_id = state
    .files
    .keys()
    .max()
    .expect("expected at least one file");

  for file_id in (0..=*max_file_id).rev() {
    let file_index = state.files[&file_id];
    state.attempt_move(file_index);
  }

  let ids = &state
    .blocks
    .iter()
    .flat_map(|block| match block {
      Block::File { id, size, .. } => vec![Some(*id); *size as usize],
      Block::FreeSpace { size, .. } => vec![None; *size as usize],
    })
    .collect::<Vec<_>>();

  let result = checksum(ids);

  Ok(result)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_1() {
    let input = blocks_from_string("2333133121414131402".to_string());
    let mut ids = input
      .iter()
      .flat_map(|block| match block {
        Block::File { id, size, .. } => vec![Some(*id); *size as usize],
        Block::FreeSpace { size, .. } => vec![None; *size as usize],
      })
      .collect::<Vec<_>>();
    block_compact(&mut ids);
    let result = checksum(&ids);
    assert_eq!(result, 1928);
  }
}
