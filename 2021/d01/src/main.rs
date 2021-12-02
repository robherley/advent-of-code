fn pt1(nums: &Vec<u32>) -> u32 {
  nums
    .iter()
    .fold((0, None), |(inc, prev), curr| {
        match prev {
          Some(n) if curr > n => (inc+1, Some(curr)),
          _ => (inc, Some(curr)),
        }
    })
    // this tuple indexing syntax is wack
    .0
}

fn pt2(nums: &Vec<u32>) -> u32 {
  nums
    // std::slice::Windows is OP
    .windows(3)
    .fold((0, None), |(inc, prev), window| {
        let curr: u32 = window.iter().sum();
        match prev {
          Some(n) if curr > n => (inc+1, Some(curr)),
          _ => (inc, Some(curr)),
        }
    })
    .0
}

fn main() {
  // there's std::io::BufReader if this file was huge
  let text = std::fs::read_to_string("./assets/input.txt")
    .expect("Unable to read input file");

  let nums = text.lines().map(|line| line.parse::<u32>().unwrap()).collect();

  println!("[pt1]: {}", pt1(&nums));
  println!("[pt2]: {}", pt2(&nums));
}