fn process_input(input: &str) -> Vec<Vec<i32>> {
  input.lines()
    .map(|line| {
      line.split_whitespace()
        .filter_map(|ss| ss.parse().ok())
        .collect()
    })
    .collect()
}

pub fn part_one(input: &str) -> usize {
  let processed_input = process_input(input);
  processed_input.into_iter()
    .map(|report| {
      let is_continuous = report.is_sorted() || report.iter().rev().is_sorted();
      is_continuous &&
      report.windows(2)
        .all(|window| {
          let diff = window[0].abs_diff(window[1]);
          diff < 4 && diff > 0
        })
    })
    .filter(|x| *x)
    .count()
} 

pub fn part_two(input: &str) -> i32 {
  0
}

#[cfg(test)]
mod tests{
  use super::*;

  #[test]
  fn part_one_works() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    assert_eq!(part_one(input), 2);
  }
}