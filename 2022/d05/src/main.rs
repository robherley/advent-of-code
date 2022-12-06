use std::num::ParseIntError;
use regex::Regex;

type Instruction = (usize, usize, usize);
type Crate = Vec<char>;

fn parse_input(input: &str) -> (Vec<Crate>, Vec<Instruction>) {
  let mut crates_strs = vec![];
  let mut instructions: Vec<Instruction> = vec![];
  let num_re = Regex::new(r"\d+").unwrap();

  let mut is_instructions = false;
  input.lines().for_each(|line| {
    if line.is_empty() {
      crates_strs.pop(); // pesky line with numbers
      is_instructions = true;
    } else if is_instructions {
      let nums: Vec<Result<usize, ParseIntError>> = num_re.captures_iter(line).map(|cap| cap[0].parse::<usize>()).collect();
      match nums[..] {
        [Ok(x), Ok(y), Ok(z)] => instructions.push((x, y-1, z-1)), // -1 to stay 0 indexed
        _ => panic!("Invalid instruction: {}", line),
      }
    } else {
      crates_strs.push(line);
    }
  });

  // subtract 1 from the length and 2 from either space end, divide by 4 to skip brackets/spaces
  let num_crates = (crates_strs[0].len() - 3) / 4;
  // build a vec of vec (inner vecs will be used as stacks)
  let mut crates = (0..num_crates+1).map(|_| vec![]).collect::<Vec<Crate>>();

  for line in crates_strs.iter().rev() {
    let mut iter = line.chars();
    // skip the first bracket/space
    iter.next();
    // get every 4th character from the line
    iter.step_by(4).enumerate().for_each(|(i, c)| {
      if c != ' ' {
        crates[i].push(c);
      }
    });
  }

  (crates, instructions)
}

fn pt1(mut crates: Vec<Vec<char>>, instructions: &Vec<Instruction>) -> String {
  for (n, src, dst) in instructions {
    for _ in 0..*n {
      let c = crates[*src].pop().unwrap();
      crates[*dst].push(c);
    }
  }

  crates.iter().map(|c| *c.last().unwrap()).collect::<String>()
}

fn pt2(mut crates: Vec<Vec<char>>, instructions: &Vec<Instruction>) -> String {
  for (n, src, dst) in instructions {
    let start = crates[*src].len() - *n;
    let drained: Vec<char> = crates[*src].drain(start..).collect();
    crates[*dst].extend(drained);
  }

  crates.iter().map(|c| *c.last().unwrap()).collect::<String>()
}

fn main() {
  let input = include_str!("../assets/input.txt");
  let (crates, instructions) = parse_input(input);

  println!("[pt1]: {}", pt1(crates.clone(), &instructions));
  println!("[pt2]: {}", pt2(crates.clone(), &instructions));
}
