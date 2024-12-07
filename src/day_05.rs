use std::{
  collections::{HashMap, HashSet},
  io,
  str::FromStr,
};

use crate::fs::read_day;

#[derive(Debug, Clone, Copy)]
struct Rule {
  // `before` must be done before `after`
  before: u32,
  after: u32,
}

#[derive(Debug)]
struct Update {
  sequence: Vec<u32>,
}

#[derive(Debug)]
struct Input {
  rules: Vec<Rule>,
  updates: Vec<Update>,
}

fn input() -> io::Result<Input> {
  let lines = read_day(5)?
    .map(|line| line.trim().to_string())
    .collect::<Vec<_>>();

  let divider_index = lines.iter().position(|line| line.is_empty()).unwrap();

  let (rules, updates) = lines.split_at(divider_index);
  let updates = &updates[1..];

  let rules = rules
    .iter()
    .map(|line| {
      let line = line
        .split("|")
        .filter_map(|s| s.trim().parse::<u32>().ok())
        .collect::<Vec<_>>();

      let slice = line.as_slice();

      match slice {
        [before, after] => Some(Rule {
          before: *before,
          after: *after,
        }),
        _ => None,
      }
    })
    .collect::<Vec<_>>();

  let updates = updates
    .iter()
    .map(|line| {
      let sequence = line
        .split(",")
        .filter_map(|s| s.trim().parse::<u32>().ok())
        .collect::<Vec<_>>();

      Update { sequence }
    })
    .collect::<Vec<_>>();

  Ok(Input {
    rules: rules.into_iter().flatten().collect(),
    updates: updates,
  })
}

type DirectedGraph = HashMap<u32, HashSet<u32>>;

fn make_topsort_graph(rules: Vec<Rule>) -> DirectedGraph {
  let mut graph = DirectedGraph::new();

  for rule in rules {
    let set = graph.entry(rule.after).or_default();
    set.insert(rule.before);
  }

  graph
}

fn get_middle_element(v: &Vec<u32>) -> Option<u32> {
  if v.len() % 2 == 0 {
    None
  } else {
    Some(v[v.len() / 2])
  }
}

fn match_rules(graph: &DirectedGraph, update: &Update) -> bool {
  let mut not_allowed: HashSet<u32> = HashSet::new();

  for num in update.sequence.iter() {
    if not_allowed.contains(num) {
      return false;
    }

    match graph.get(num) {
      Some(set) => not_allowed.extend(set.iter()),
      None => {}
    }
  }

  true
}

fn fix_update(update: &Update, graph: &DirectedGraph) -> Update {
  let mut sorted = update.sequence.clone();
  sorted.sort_by(|a, b| {
    let a_set = graph.get(a);
    let b_set = graph.get(b);

    if a_set.is_some() && a_set.unwrap().contains(b) {
      // b is required to happen before a
      std::cmp::Ordering::Greater
    } else if b_set.is_some() && b_set.unwrap().contains(a) {
      // a is required to happen before b
      std::cmp::Ordering::Less
    } else {
      std::cmp::Ordering::Equal
    }
  });

  Update { sequence: sorted }
}

pub fn part_1() -> io::Result<u32> {
  let Input { rules, updates } = input()?;
  let graph = make_topsort_graph(rules);

  let result = updates
    .iter()
    .filter(|update| match_rules(&graph, update))
    .filter_map(|update| get_middle_element(&update.sequence))
    .sum();

  Ok(result)
}

pub fn part_2() -> io::Result<u32> {
  let Input { rules, updates } = input()?;
  let graph = make_topsort_graph(rules);

  println!("{:?}", graph);

  let result = updates
    .iter()
    .filter(|update| !match_rules(&graph, update))
    .map(|update| fix_update(update, &graph))
    .filter_map(|update| get_middle_element(&update.sequence))
    .sum();

  Ok(result)
}
