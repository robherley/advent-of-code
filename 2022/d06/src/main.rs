use std::collections::{HashSet};

fn find_marker(input: &str, size: usize) -> Option<usize> {
  let chars: Vec<char> = input.chars().collect();
  let found = chars.windows(size).enumerate().find(|(_i, window)| {
    HashSet::<char>::from_iter(window.iter().cloned()).len() == size
  });

  match found {
    Some((i, _)) => Some(i+size),
    None => None
  }
}

fn pt1(input: &str) -> Option<usize> {
  find_marker(input, 4)
}

fn pt2(input: &str) -> Option<usize> {
  find_marker(input, 14)
}

fn main() {
  let input = include_str!("../assets/input.txt").trim();
  println!("[pt1]: {:?}", pt1(input).unwrap());
  println!("[pt2]: {:?}", pt2(input).unwrap());
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUTS: [&str; 7] = [
    "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
    "bvwbjplbgvbhsrlpgdmjqwftvncz",
    "nppdvjthqldpwncqszvftbrmjlhg",
    "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
    "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    "foo",
    "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz",
  ];

  #[test]
  fn find_markers_pt1() {
    let expected = [Some(7), Some(5), Some(6), Some(10), Some(11), None, None];
    for (input, want) in INPUTS.iter().zip(expected.iter()) {
      assert_eq!(pt1(input), *want, "input: {}", input);
    }
  }

  #[test]
  fn find_markers_pt2() {
    let expected = [Some(19), Some(23), Some(23), Some(29), Some(26), None, None];
    for (input, want) in INPUTS.iter().zip(expected.iter()) {
      assert_eq!(pt2(input), *want, "input: {}", input);
    }
  }
}
