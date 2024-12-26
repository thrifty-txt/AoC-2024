use itertools::Itertools;

pub fn part_one(input: &str) -> i32 {
  let (mut first, mut second): (Vec<_>, Vec<_>) = input.lines()
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
    .unzip();

  first.sort_unstable();
  second.sort_unstable();

  first
    .into_iter()
    .zip_eq(second)
    .map(|(l, r)|
      (l - r).abs()
    )
    .sum()
}

pub fn part_two(){

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