enum Solution {
  Part1,
  Part2,
}

#[derive(Clone, PartialEq, Eq)]
enum Material {
  Rock,
  Air,
  SandFall,
  SandRest,
}

impl std::fmt::Display for Material {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let ch = match self {
      Material::Rock => '⬜',
      Material::Air => '⬛',
      Material::SandFall => '🟨',
      Material::SandRest => '🟫',
    };

    write!(f, "{}", ch)
  }
}

#[allow(dead_code)]
fn pretty_print(grid: &Vec<Vec<Material>>, falling_sand: Option<(usize, usize)>) {
  std::thread::sleep(std::time::Duration::from_millis(100));
  print!("\x1B[2J\x1B[1;1H");
  let x_digits = grid[0].len().to_string().len();
  let y_digits = grid.len().to_string().len();

  for digit in 0..x_digits {
    print!("{:n$}", " ", n = y_digits + 1);
    for i in 0..grid[0].len() {
      print!(" {}", format!("{:n$}", i, n = x_digits).chars().nth(digit).unwrap());
    }
    println!();
  }
  for (i, row) in grid.iter().enumerate() {
    print!("{:n$} ", i, n = y_digits);
    for (j, col) in row.iter().enumerate() {
      if let Some((x, y)) = falling_sand {
        if i == y && j == x {
          print!("{}", Material::SandFall);
          continue;
        }
      }
      print!("{}", col);
    }
    println!();
  }
}

fn build_grid(input: &str) -> Vec<Vec<Material>> {
  let mut max = (0, 0);
  let paths = input.split("\n").map(|path| {
    path.split(" -> ").map(|point| {
      let point = point.split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
      max.0 = max.0.max(point[0]);
      max.1 = max.1.max(point[1]);
      (point[0], point[1])
    }).collect::<Vec<(usize, usize)>>()
  }).collect::<Vec<Vec<(usize, usize)>>>();

  let mut grid = vec![vec![Material::Air; max.0+1 as usize]; max.1+1 as usize];
  for path in paths {
    for window in path.windows(2) {
      let x_bounds = (window[0].0.min(window[1].0),  window[0].0.max(window[1].0));
      let y_bounds = (window[0].1.min(window[1].1),  window[0].1.max(window[1].1));

      for y in y_bounds.0..=y_bounds.1 {
        for x in x_bounds.0..=x_bounds.1 {
          grid[y][x] = Material::Rock;
        }
      }
    }
  }

  grid
}

fn drop(grid: &mut Vec<Vec<Material>>, sand: (usize, usize)) -> Option<Solution> {
  let (mut x, mut y) = sand;

  if grid[y][x] != Material::Air {
    return Some(Solution::Part2);
  }

  loop {
    if grid.get(y+1).is_none() {
      return Some(Solution::Part1);
    } else if grid[y+1][x] == Material::Air { // down
      y += 1;
    } else if grid[y+1][x-1] == Material::Air { // left
      y += 1;
      x -= 1;
    } else if grid[y+1][x+1] == Material::Air { // right
      y += 1;
      x += 1;
    } else {
      grid[y][x] = Material::SandRest;
      return None;
    }
  }
}

fn pt1(grid: &mut Vec<Vec<Material>>) -> usize {
  let mut i = 0;
  loop {
    match drop(grid, (500, 0)) {
      Some(Solution::Part1) => {
        return i
      }
      _ => {
        i += 1;
        if i == usize::MAX {
          panic!("ya goofed");
        }
      }
    }
  }
}

fn pt2(grid: &mut Vec<Vec<Material>>) -> usize {
  for row in grid.iter_mut() {
    // doubling it is close to infinitely wide...right?
    row.extend(vec![Material::Air; row.len()]);
  }

  grid.push(vec![Material::Air; grid[0].len()]);
  grid.push(vec![Material::Rock; grid[0].len()]);

  let mut i = 0;
  loop {
    match drop(grid, (500, 0)) {
      Some(Solution::Part2) => {
        return i
      }
      _ => {
        i += 1;
        if i == usize::MAX {
          panic!("ya goofed");
        }
      }
    }
  }
}

fn main() {
  let input = include_str!("../assets/input.txt").trim();
  let grid = build_grid(input);

  println!("pt1: {}", pt1(&mut grid.clone()));
  println!("pt2: {}", pt2(&mut grid.clone()));
}
