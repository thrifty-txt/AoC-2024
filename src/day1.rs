use itertools::Itertools;

fn input_to_two_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
  input.lines()
    .filter_map(|line| 
      line
        .split_whitespace()
        .filter(|substring| !substring.is_empty())
        .filter_map(|substring| substring
          .parse::<i32>()
          .ok()
        )
        .collect_tuple::<(i32, i32)>()
    )
    .unzip()
}

pub fn part_one(input: &str) -> i32 {
  let (mut first, mut second) = input_to_two_lists(input);
  first.sort_unstable();
  second.sort_unstable();

  let sum = first
    .into_iter()
    .zip_eq(second)
    .map(|(l, r)|
      (l - r).abs()
    )
    .sum();
  sum
}

pub fn part_two(input: &str) -> i32 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_one_works() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    assert_eq!(part_one(input), 11);
  }
}