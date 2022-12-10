use std::collections::{HashMap};

#[derive(Debug, Clone)]
enum Direction {
  Up,
  Down,
  Right,
  Left,
}

type Instruction = (Direction, i32);

fn build_instructions(input: &str) -> Vec<Instruction> {
  input
    .lines()
    .map(|line| {
      let (direction, value) = line.split_once(" ").unwrap();
      let n = value.parse().unwrap();
      match direction {
        "U" => (Direction::Up, n),
        "D" => (Direction::Down, n),
        "R" => (Direction::Right, n),
        "L" => (Direction::Left, n),
        _ => panic!("unknown direction"),
      }
    })
    .collect()
}

#[allow(dead_code)]
fn debug_step(x_range: (i32, i32), y_range: (i32, i32), head: (i32, i32), tail: (i32, i32)) {
  let (xmin, xmax) = x_range;
  let (ymin, ymax) = y_range;
  println!();
  for y in (ymin..ymax).rev() {
    for x in xmin..xmax {
      if x == head.0 && y == head.1 {
        print!("H");
      } else if x == tail.0 && y == tail.1 {
        print!("T");
      } else if x == 0 && y == 0 {
        print!("s");
      } else {
        print!(".");
      }
    }
    println!();
  }
  println!("H:{:?}, T:{:?} | Δ:{:?}", head, tail, (head.0 - tail.0, head.1 - tail.1));
  println!();
}

#[allow(dead_code)]
fn debug_seen(x_range: (i32, i32), y_range: (i32, i32), seen_coords: &HashMap<(i32, i32), bool>) {
  let (xmin, xmax) = x_range;
  let (ymin, ymax) = y_range;
  println!();
  for y in (ymin..ymax).rev() {
    for x in xmin..xmax {
      if x == 0 && y == 0 {
        print!("s");
      } else if seen_coords.contains_key(&(x,y)) {
        print!("#");
      } else {
        print!(".");
      }
    }
    println!();
  }
}

fn move_knot(prev: (i32, i32), knot: (i32, i32)) -> (i32, i32) {
  let mut new_knot = knot;

  let dx = prev.0 - knot.0;
  let dy = prev.1 - knot.1;

  // shamelessly did if/else until i saw the pattern 🤦
  // also TIL signum() exists, thanks copilot

  if dx.abs() > 1 { // left or right
    new_knot.0 += dx.signum();

    if dy.abs() > 0 { // go diag
      new_knot.1 += dy.signum();
    }
  } else if dy.abs() > 1 { // up or down
    new_knot.1 += dy.signum();

    if dx.abs() > 0 { // go diag
      new_knot.0 += dx.signum();
    }
  }

  new_knot
}

fn swing_rope(instructions: &Vec<(Direction, i32)>, rope_length: usize) -> usize {
  let mut rope = vec![(0,0); rope_length];

  let mut seen_coords: HashMap<(i32, i32), bool> = HashMap::new();
  seen_coords.insert((0,0), true);

  for instr in instructions {
    let (direction, n) = instr;
    for _ in 0..*n {
      match direction {
        Direction::Up => { rope[0].1 += 1; },
        Direction::Down => { rope[0].1 -= 1; },
        Direction::Right => { rope[0].0 += 1; },
        Direction::Left => { rope[0].0 -= 1; },
      }
      for i in 1..rope_length {
        rope[i] = move_knot(rope[i-1], rope[i]);

        if i == rope_length-1 {
          // it's the tail
          seen_coords.insert(rope[i], true);
        }
      }
    }
  }

  seen_coords.len()
}

fn main() {
  let input = include_str!("../assets/input.txt");
  let instructions = build_instructions(input);

  // this can probably be solved by one swing_rope() call and two hashmaps but i'm tired
  println!("[pt1]: {:?}", swing_rope(&instructions, 2 as usize));
  println!("[pt2]: {:?}", swing_rope(&instructions, 10 as usize));
}
