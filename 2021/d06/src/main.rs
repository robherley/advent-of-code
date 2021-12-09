use std::fs;
use std::collections::VecDeque;
use std::time::Instant;

// brute force pt1 :)
fn naive(text: &String, last_day: usize) -> usize {
  let mut fish: Vec<usize> = text.split(',').map(|str| str.parse::<usize>().unwrap()).collect();

  for _ in 1..=last_day {
    let mut fish_to_add = 0;
    for fish in fish.iter_mut() {
      if *fish == 0 {
        *fish = 6;
        fish_to_add += 1;
      } else {
        *fish -= 1;
      }
    }
    if fish_to_add > 0 {
      (0..fish_to_add).for_each(|_| fish.push(8))
    }
  }

  return fish.len()
}

fn better(text: &String, last_day: usize) -> usize {
  // shoutout to https://stackoverflow.com/a/59273079 instead of me shifting a vec :)
  let mut fish: VecDeque<usize> = VecDeque::from([0usize;9]);
  text.split(',').for_each(|f| fish[f.parse::<usize>().unwrap()] += 1);

  for _ in 1..=last_day {
    let died = fish.pop_front().unwrap();
    fish.push_back(died);
    fish[6] += died;
  }

  return fish.iter().sum()
}

fn main() {
  let text = fs::read_to_string("./assets/input.txt")
    .expect("Unable to read input file");

  let now = Instant::now();
  println!("naive pt1: {:?} (took {}µs)", naive(&text, 80), now.elapsed().as_micros());
  let now = Instant::now();
  println!("better pt1: {:?} (took {}µs)", better(&text, 80), now.elapsed().as_micros());
  let now = Instant::now();
  println!("better pt2: {:?} (took {}µs)", better(&text, 256), now.elapsed().as_micros());
}
