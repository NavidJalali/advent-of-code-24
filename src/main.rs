mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod fs;

fn main() {
  use day_06::*;
  println!("Start");
  let start = std::time::Instant::now();
  let result_1 = part_1();
  let checkpoint = std::time::Instant::now();
  let result_2 = part_2();
  let end = std::time::Instant::now();

  println!(
    "Part 1 took {:?}. Result: {:?}",
    checkpoint.duration_since(start),
    result_1
  );

  println!(
    "Part 2 took {:?}. Result: {:?}",
    end.duration_since(checkpoint),
    result_2
  );
}
