type Range = (i32, i32);
type Pair = (Range, Range);

fn parse(input: &str) -> Result<Vec<Pair>, &'static str> {
  input.lines().map(|line| {
    let pairs: Vec<Result<Range, &'static str>> = line.split(",")
      .map(|p| {
        let range: Vec<i32> = p.split("-").map(|n| n.parse::<i32>().unwrap()).collect();

        match range[..] {
          [a, b] => Ok((a, b)),
          _ => Err("bad input"),
        }
      })
      .collect();

    match pairs[..] {
      [Ok(a), Ok(b)] => Ok((a, b)),
      _ => Err("bad input")
    }
  })
  .collect()
}

fn to_bits(range: &Range) -> u128 {
  let (a, b) = *range;

  let mut bits = 0;
  for i in a..b+1 {
    bits ^= 1 << i;
  }

  bits
}

fn pt1(assignments: &Vec<Pair>) -> i32 {
  assignments.into_iter().fold(0, |acc, (a, b)| {
    if a.0 >= b.0 && a.1 <= b.1 { // a is inside b
      return acc + 1
    } else if b.0 >= a.0 && b.1 <= a.1 { // b is inside a
      return acc + 1
    }

    acc
  })
}

fn pt2(assignments: &Vec<Pair>) -> i32 {
  assignments.into_iter().fold(0, |acc, (a, b)| {
    if (to_bits(a) & to_bits(b)) != 0 { acc + 1 } else { acc }
  })
}

fn main() {
  let input = include_str!("../assets/input.txt");
  let parsed = parse(input).unwrap();

  println!("pt1: {:?}", pt1(&parsed));
  println!("pt2: {:?}", pt2(&parsed));
}
