fn count_and_sort_cals(input: std::string::String) -> std::vec::Vec<u32> {
  let (mut vals, last) = input
    .lines()
    .fold((vec![], 0), |(mut arr, sum), line| {
      match line {
        "" => { arr.push(sum); (arr, 0) },
        str => (arr, str.parse::<u32>().unwrap() + sum),
      }
    });

  vals.push(last);
  vals.sort();

  vals
}

fn pt1(cals: &std::vec::Vec<u32>) -> u32 {
  match cals.last() {
    Some(n) => *n,
    None => 0,
  }
}

fn pt2(cals: &std::vec::Vec<u32>) -> u32 {
  cals.iter().rev().take(3).sum()
}

fn main() {
  let input = std::fs::read_to_string("./assets/input.txt").expect("Unable to read input file");
  let cals = count_and_sort_cals(input);

  println!("[pt1]: {}", pt1(&cals));
  println!("[pt2]: {}", pt2(&cals));
}
