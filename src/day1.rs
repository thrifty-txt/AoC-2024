use std::collections::HashMap;

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
  let (first, second) = input_to_two_lists(input);

  let map = second.into_iter()
    .fold(HashMap::new(), |mut acc, x| {
      let counter = acc.entry(x).or_insert(0);
      *counter += 1;
      acc
    });
  
  first.into_iter()
    .map(|x|{
      x * map.get(&x).unwrap_or(&0)
    })
    .sum()
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

  #[test]
  fn part_two_works() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";

    assert_eq!(part_two(input), 31);
  }
}