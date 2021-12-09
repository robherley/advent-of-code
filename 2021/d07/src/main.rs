use std::fs;
use std::collections::HashSet;

fn pt1(positions: &Vec<usize>) -> usize {
  let mut min_fuel = usize::MAX;
  let mut seen_pos: HashSet<usize> = HashSet::new();

  for pos in positions.iter() {
    if seen_pos.contains(pos) {
      continue
    }

    seen_pos.insert(*pos);

    let mut fuel = 0;
    for pos2 in positions.iter() {
      fuel += (*pos as i32 - *pos2 as i32).abs() as usize;
      if fuel > min_fuel {
        break;
      }
    }

    if fuel < min_fuel {
      min_fuel = fuel;
    }
  }

  return min_fuel
}

fn main() {
  let text = fs::read_to_string("./assets/input.txt")
    .expect("Unable to read input file");

  let positions: Vec<usize> = text.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
  println!("pt1: {:?}", pt1(&positions));
}
