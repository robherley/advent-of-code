#[derive(Copy, Clone, PartialEq)]
enum Outcome {
  Win = 6,
  Draw = 3,
  Lose = 0,
}

impl Outcome {
  pub fn from_str(s: &str) -> Option<Self> {
    match s {
      "X" => Some(Outcome::Lose),
      "Y" => Some(Outcome::Draw),
      "Z" => Some(Outcome::Win),
      _ => None,
    }
  }
}

#[derive(Copy, Clone, PartialEq)]
enum Shape {
  Rock = 1,
  Paper = 2,
  Scissors = 3,
}

impl Shape {
  pub fn from_str(s: &str) -> Option<Self> {
    match s {
      "A" | "X" => Some(Shape::Rock),
      "B" | "Y" => Some(Shape::Paper),
      "C" | "Z" => Some(Shape::Scissors),
      _ => None,
    }
  }
}

fn pt1(input: &str) -> i32 {
  fn to_round(line: &str) -> (Shape, Shape) {
    let split = line.split(" ").collect::<Vec<&str>>();
    if split.len() != 2 {
      panic!("invalid input: {}", line);
    }
    match (Shape::from_str(split[0]), Shape::from_str(split[1])) {
      (Some(a), Some(b)) => (a, b),
      (_, _) => panic!("one or more invalid shapes ({}, {})", split[0], split[1]),
    }
  }

  fn outcome(them: Shape, me: Shape) -> Outcome {
    match (them, me) {
      (Shape::Rock, Shape::Paper)
      | (Shape::Paper, Shape::Scissors)
      | (Shape::Scissors, Shape::Rock) => Outcome::Win,
      (a, b) if a == b => Outcome::Draw,
      (_, _) => Outcome::Lose,
    }
  }

  input.lines().fold(0, |sum, line| {
    let (them, me) = to_round(line);
    sum + me as i32 + outcome(them, me) as i32
  })
}

fn pt2(input: &str) -> i32 {
  fn to_round(line: &str) -> (Shape, Outcome) {
    let split = line.split(" ").collect::<Vec<&str>>();
    if split.len() != 2 {
      panic!("invalid input: {}", line);
    }
    match (Shape::from_str(split[0]), Outcome::from_str(split[1])) {
      (Some(a), Some(b)) => (a, b),
      (_, _) => panic!("one or more invalid values ({}, {})", split[0], split[1]),
    }
  }

  fn shape(them: Shape, expected: Outcome) -> Shape {
    match (them, expected) {
      (Shape::Rock, Outcome::Win) => Shape::Paper,
      (Shape::Rock, Outcome::Lose) => Shape::Scissors,
      (Shape::Paper, Outcome::Win) => Shape::Scissors,
      (Shape::Paper, Outcome::Lose) => Shape::Rock,
      (Shape::Scissors, Outcome::Win) => Shape::Rock,
      (Shape::Scissors, Outcome::Lose) => Shape::Paper,
      (shape, Outcome::Draw) => shape,
    }
  }

  input.lines().fold(0, |sum, line| {
    let (them, expected) = to_round(line);
    sum + expected as i32 + shape(them, expected) as i32
  })
}

fn main() {
  let input = include_str!("../assets/input.txt");
  println!("[pt1]: {}", pt1(input));
  println!("[pt2]: {}", pt2(input));
}
