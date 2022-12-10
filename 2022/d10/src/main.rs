fn tick(cycle: &mut i32, x: i32, sum: &mut i32) {
  print_char(*cycle, x);
  *cycle+=1;
  if [20,60,100,140,180,220].contains(cycle) {
    *sum += *cycle*x;
  }
}

fn print_char(cycle: i32, x: i32) {
  let pixel = cycle % 40;
  print!("{}", if [x-1, x, x+1].contains(&pixel) {'🟩'} else {'⬛'});
  if pixel == 39 {
    println!("");
  }
}

fn main() {
  let input = include_str!("../assets/input.txt");

  let mut x = 1;
  let mut cycle = 0;
  let mut sum = 0;

  for line in input.lines() {
    match line {
      "noop" => {
        tick(&mut cycle, x, &mut sum);
      },
      line if line.starts_with("addx") => {
        tick(&mut cycle, x, &mut sum);
        tick(&mut cycle, x, &mut sum);
        x += line.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
      },
      _ => panic!("unknown instruction")
    }
  }

  println!("sum: {}", sum)
}
