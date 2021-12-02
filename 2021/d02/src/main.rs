fn pt1(nums: &Vec<(&str, i32)>) -> i32 {
  let res = nums
    .iter()
    .fold((0, 0), |(x, y), curr| {
        match curr {
          ("forward", n) => (x+n, y),
          ("down", n) => (x, y+n),
          ("up", n) => (x, y-n),
          (_,_) => (x,y),
        }
    });

  res.0 * res.1
}

fn pt2(nums: &Vec<(&str, i32)>) -> i32 {
  let res = nums
    .iter()
    .fold((0, 0, 0), |(x, y, a), curr| {
        match curr {
          ("forward", n) if a > 0 => (x+n, y+(a*n), a),
          ("forward", n) => (x+n, y, a),
          ("down", n) => (x, y, a+n),
          ("up", n) => (x, y, a-n),
          (_,_) => (x,y,a),
        }
    });

  res.0 * res.1
}

fn main() {
  // TODO: make a common lib for all problems
  let text = std::fs::read_to_string("./assets/input.txt")
    .expect("Unable to read input file");

  let nums: Vec<(&str, i32)> = text.lines().map(|line| {
    let split: Vec<&str> = line.split_whitespace().collect();
    // TODO: should check bounds
    let n = split[1].parse::<i32>();
    (split[0], n.unwrap())
  }).collect();

  println!("[pt1]: {}", pt1(&nums));
  println!("[pt2]: {}", pt2(&nums));
}