use serde_json::Value;
use serde_json::Value::{Array, Number};
use std::cmp::Ordering;

fn compare(first: &Value, second: &Value) -> Ordering {
  match (first, second) {
    (Array(a), Array(b)) => {
      let min = std::cmp::min(a.len(), b.len());
      for i in 0..min {
        match compare(&a[i], &b[i]) {
          Ordering::Equal => continue,
          order => return order,
        }
      }
      a.len().cmp(&b.len())
    }
    (Number(num), Array(_)) => {
      let wrapped = serde_json::json!(vec![num]);
      compare(&wrapped, &second)
    }
    (Array(_), Number(num)) => {
      let wrapped = serde_json::json!(vec![num]);
      compare(&first, &wrapped)
    }
    (Number(a), Number(b)) => {
      let nums = (a.as_i64(), b.as_i64());
      nums.0.cmp(&nums.1)
    }
    _ => panic!("bad input"),
  }
}

fn pt1(values: &Vec<Value>) -> usize {
  values.chunks(2).enumerate().fold(0, |sum, (i, pair)| {
    match compare(&pair[0], &pair[1]) {
      Ordering::Less => sum + i + 1,
      _ => sum,
    }
  })
}

fn pt2(values: &Vec<Value>) -> usize {
  let mut cloned = values.clone();

  let decoder_a = serde_json::json!(vec![vec![2]]);
  let decoder_b = serde_json::json!(vec![vec![6]]);

  cloned.push(decoder_a.clone());
  cloned.push(decoder_b.clone());

  cloned.sort_by(|a, b| {
    compare(a, b)
  });

  let mut mult: Option<usize> = None;
  for (i, val) in cloned.into_iter().enumerate() {
    if val == decoder_a {
      mult = Some(i+1);
    }

    if val == decoder_b {
      mult = mult.map(|m| m * (i+1));
      break;
    }
  }

  if mult.is_none() {
    panic!("could not find decoders")
  }

  mult.unwrap()
}

fn main() {
  let input = include_str!("../assets/input.txt");
  let parsed = input.split("\n\n").flat_map(|pairs| {
    pairs
      .split('\n')
      .map(|s| serde_json::from_str(s).unwrap())
  }).collect::<Vec<Value>>();

  println!("[pt1]: {}", pt1(&parsed));
  println!("[pt2]: {}", pt2(&parsed));
}
