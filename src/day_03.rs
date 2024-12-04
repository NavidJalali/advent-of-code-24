use std::{fs, io};

#[derive(Debug, Clone, Copy)]
struct Invocation {
  arg1: u32,
  arg2: u32,
}

impl Invocation {
  fn apply(&self) -> u32 {
    self.arg1 * self.arg2
  }
}

#[derive(Debug)]
enum ParseState {
  Disabled(Vec<Invocation>),
  Running(Vec<Invocation>),
  Invoke {
    backtrack_to: usize,
    accumulated: Vec<Invocation>,
  },
  Arg1 {
    backtrack_to: usize,
    accumulated: Vec<Invocation>,
    arg1: u32,
  },
  Comma {
    backtrack_to: usize,
    accumulated: Vec<Invocation>,
    arg1: u32,
  },
  Arg2 {
    backtrack_to: usize,
    accumulated: Vec<Invocation>,
    arg1: u32,
    arg2: u32,
  },
}

impl ParseState {
  fn accumulated(&self) -> &Vec<Invocation> {
    match self {
      ParseState::Running(accumulated) => accumulated,
      ParseState::Invoke { accumulated, .. } => accumulated,
      ParseState::Arg1 { accumulated, .. } => accumulated,
      ParseState::Comma { accumulated, .. } => accumulated,
      ParseState::Arg2 { accumulated, .. } => accumulated,
      ParseState::Disabled(accumulated) => accumulated,
    }
  }
}

fn starts_with(input: &[char], cursor: usize, value: &str) -> bool {
  let read = input
    .iter()
    .skip(cursor)
    .take(value.len())
    .collect::<String>();

  read == value
}

fn read_number(input: &[char], cursor: usize, max_width: usize) -> Option<(u32, usize)> {
  let mut width = 0;
  let mut value = 0;
  let new_cursor = cursor;

  while width < max_width {
    let c = input.get(new_cursor + width)?;
    if c.is_digit(10) {
      value = value * 10 + c.to_digit(10)?;
      width += 1;
    } else {
      break;
    }
  }

  Some((value, new_cursor + width))
}

pub fn part_1() -> io::Result<u32> {
  let input = fs::read_to_string("input/day_03.txt")?;
  let input = input.chars().collect::<Vec<_>>();
  let input = input.as_slice();
  let length = input.len();
  let mut state = ParseState::Running(vec![]);
  let mut cursor = 0;

  while cursor < length {
    match state {
      ParseState::Running(accumulated) => {
        if starts_with(input, cursor, "mul(") {
          state = ParseState::Invoke {
            backtrack_to: cursor,
            accumulated: accumulated,
          };
          cursor += 4;
        } else {
          state = ParseState::Running(accumulated);
          cursor += 1;
        }
      }
      ParseState::Invoke {
        backtrack_to,
        accumulated,
      } => match read_number(input, cursor, 3) {
        Some((arg, new_cursor)) => {
          state = ParseState::Arg1 {
            backtrack_to,
            accumulated,
            arg1: arg,
          };
          cursor = new_cursor;
        }
        None => {
          state = ParseState::Running(accumulated);
          cursor = backtrack_to + 1;
        }
      },
      ParseState::Arg1 {
        backtrack_to,
        accumulated,
        arg1,
      } => {
        if starts_with(input, cursor, ",") {
          state = ParseState::Comma {
            backtrack_to,
            accumulated,
            arg1,
          };
          cursor += 1;
        } else {
          cursor = backtrack_to + 1;
          state = ParseState::Running(accumulated);
        }
      }
      ParseState::Comma {
        backtrack_to,
        accumulated,
        arg1,
      } => match read_number(input, cursor, 3) {
        Some((arg, new_cursor)) => {
          state = ParseState::Arg2 {
            backtrack_to,
            accumulated,
            arg1,
            arg2: arg,
          };
          cursor = new_cursor;
        }
        None => {
          state = ParseState::Running(accumulated);
          cursor = backtrack_to + 1;
        }
      },
      ParseState::Arg2 {
        backtrack_to,
        accumulated,
        arg1,
        arg2,
      } => {
        if starts_with(input, cursor, ")") {
          let invocation = Invocation { arg1, arg2 };
          let mut acc = accumulated;
          acc.push(invocation);
          state = ParseState::Running(acc);
          cursor += 1;
        } else {
          cursor = backtrack_to + 1;
          state = ParseState::Running(accumulated);
        }
      }
      _ => {
        cursor += 1;
      }
    }
  }

  Ok(
    state
      .accumulated()
      .iter()
      .map(|invocation| invocation.apply())
      .sum(),
  )
}

pub fn part_2() -> io::Result<u32> {
  let input = fs::read_to_string("input/day_03.txt")?;
  let input = input.chars().collect::<Vec<_>>();
  let input = input.as_slice();
  let length = input.len();
  let mut state = ParseState::Running(vec![]);
  let mut cursor = 0;

  while cursor < length {
    if starts_with(input, cursor, "don't()") {
      state = ParseState::Disabled(state.accumulated().clone());
      cursor += 6;
    }
    match state {
      ParseState::Running(accumulated) => {
        if starts_with(input, cursor, "mul(") {
          state = ParseState::Invoke {
            backtrack_to: cursor,
            accumulated: accumulated,
          };
          cursor += 4;
        } else {
          state = ParseState::Running(accumulated);
          cursor += 1;
        }
      }
      ParseState::Invoke {
        backtrack_to,
        accumulated,
      } => match read_number(input, cursor, 3) {
        Some((arg, new_cursor)) => {
          state = ParseState::Arg1 {
            backtrack_to,
            accumulated,
            arg1: arg,
          };
          cursor = new_cursor;
        }
        None => {
          state = ParseState::Running(accumulated);
          cursor = backtrack_to + 1;
        }
      },
      ParseState::Arg1 {
        backtrack_to,
        accumulated,
        arg1,
      } => {
        if starts_with(input, cursor, ",") {
          state = ParseState::Comma {
            backtrack_to,
            accumulated,
            arg1,
          };
          cursor += 1;
        } else {
          cursor = backtrack_to + 1;
          state = ParseState::Running(accumulated);
        }
      }
      ParseState::Comma {
        backtrack_to,
        accumulated,
        arg1,
      } => match read_number(input, cursor, 3) {
        Some((arg, new_cursor)) => {
          state = ParseState::Arg2 {
            backtrack_to,
            accumulated,
            arg1,
            arg2: arg,
          };
          cursor = new_cursor;
        }
        None => {
          state = ParseState::Running(accumulated);
          cursor = backtrack_to + 1;
        }
      },
      ParseState::Arg2 {
        backtrack_to,
        accumulated,
        arg1,
        arg2,
      } => {
        if starts_with(input, cursor, ")") {
          let invocation = Invocation { arg1, arg2 };
          let mut acc = accumulated;
          acc.push(invocation);
          state = ParseState::Running(acc);
          cursor += 1;
        } else {
          cursor = backtrack_to + 1;
          state = ParseState::Running(accumulated);
        }
      }
      ParseState::Disabled(accumulated) => {
        if starts_with(input, cursor, "do()") {
          state = ParseState::Running(accumulated);
          cursor += 4;
        } else {
          state = ParseState::Disabled(accumulated);
          cursor += 1;
        }
      }
    }
  }

  Ok(
    state
      .accumulated()
      .iter()
      .map(|invocation| invocation.apply())
      .sum(),
  )
}
