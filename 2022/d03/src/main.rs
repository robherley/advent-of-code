use std::collections::HashSet;

fn to_offset(c: char) -> i32 {
  let n = c as i32;
  if n > 96 { // a-z
    n - 96
  } else { // A-z
    n - 38
  }
}

fn pt1(input: &str) -> i32 {
  input.lines().map(|line| {
    let chars: Vec<char> = line.chars().collect();
    let mid = chars.len() / 2;

    let a: HashSet<char> = chars[..mid].to_vec().into_iter().collect();
    let b = chars[mid..].to_vec().into_iter().collect();

    match a.intersection(&b).next() {
      Some(c) => to_offset(*c),
      None => 0
    }
  }).sum()
}

fn pt2(input: &str) -> i32 {
  input.lines().collect::<Vec<&str>>().chunks(3).map(|chunk| {
    let dupes = chunk.into_iter().fold(HashSet::new(), |mut acc, line| {
      if acc.is_empty() {
        acc = line.chars().collect();
      } else {
        let chars: HashSet<char> = line.chars().collect();
        let dupes = acc.intersection(&chars);
        acc = dupes.into_iter().cloned().collect();
      }

      acc
    });

    match dupes.into_iter().next() {
      Some(c) => to_offset(c),
      None => 0
    }
  }).sum()
}

fn main() {
  let input = include_str!("../assets/input.txt");
  println!("[pt1]: {}", pt1(input));
  println!("[pt2]: {}", pt2(input));
}
