use std::fs;
use std::collections::HashSet;

fn solve(positions: &Vec<usize>, is_pt2: bool) -> usize {
  let mut min_fuel = usize::MAX;
  let mut seen_pos: HashSet<usize> = HashSet::new();

  for pos in positions.iter() {
    if seen_pos.contains(pos) {
      continue
    }

    seen_pos.insert(*pos);

    let mut fuel = 0;
    for pos2 in positions.iter() {
      let mut curr_fuel = (*pos as isize - *pos2 as isize).abs() as usize;
      if is_pt2 {
        // cheated a little bit :) had to google to find the right equation
        // https://en.wikipedia.org/wiki/Triangular_number aka n * (n+1) / 2
        curr_fuel = curr_fuel * (curr_fuel + 1) / 2;
      }

      fuel += curr_fuel;
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
  println!("pt1: {:?}", solve(&positions, false));
  println!("pt2: {:?}", solve(&positions, true));
}
