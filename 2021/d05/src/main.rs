use std::fmt;
use std::fs;
use std::cmp;

#[derive(Debug)]
enum LineDirection {
  Horizontal,
  Vertical,
  Diagonal
}

struct Line {
  x: (usize, usize),
  y: (usize, usize),
  dx: i32,
  dy: i32,
  direction: LineDirection
}

impl Line {
  fn new(x: (usize, usize), y: (usize, usize)) -> Self {
    let dx = x.1 as i32 - x.0 as i32;
    let dy = y.1 as i32 - y.0 as i32;

    // TODO alternate matching & param for diagonal
    let direction = match (dx, dy) {
      (_, 0) => LineDirection::Horizontal,
      (0, _) => LineDirection::Vertical,
      (_,_) => LineDirection::Diagonal
    };

    Self {
      x: x,
      y: y,
      dx: dx,
      dy: dy,
      direction: direction
    }
  }

  fn path(&self) -> Vec<(usize, usize)> {
    match self.direction {
      LineDirection::Horizontal => {
        let (start, end) = if self.dx > 0 {self.x} else {(self.x.1, self.x.0)};
        (start..=end).map(|x| (x, self.y.0)).collect::<Vec<(usize, usize)>>()
      },
      LineDirection::Vertical => {
        let (start, end) = if self.dy > 0 {self.y} else {(self.y.1, self.y.0)};
        (start..=end).map(|y| (self.x.0, y)).collect::<Vec<(usize, usize)>>()
      },
      LineDirection::Diagonal => {
        // TODO
        vec![]
      }
    }
  }
}

impl fmt::Display for Line {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "({}, {}) -> ({}, {})", self.x.0, self.y.0, self.x.1, self.y.1).unwrap();
      Ok(())
  }
}

struct Vents {
  grid: Vec<Vec<u8>>
}

impl Vents {
  fn new(width: usize, height: usize) -> Self {
    Self {
      grid: vec![vec![0u8; width]; height],
    }
  }

  fn draw(&mut self, line: &mut Line) {
    let path = &line.path();
    path.iter().for_each(|(x,y)| self.grid[*y][*x] = self.grid[*y][*x] + 1)
  }

  fn overlap(&mut self, num: u8) -> usize {
    self.grid.iter().flatten().filter(|&&v| v >= num).cloned().collect::<Vec<u8>>().len()
  }
}

impl fmt::Display for Vents {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      for row in &self.grid {
        for vent in row {
          let str = if *vent > 0 { vent.to_string() } else { ".".to_string() };
          write!(f, "{}", str).unwrap();
        }
        write!(f, "\n").unwrap();
      }
      Ok(())
  }
}

fn parse(text: String) -> (Vec<Line>, (usize, usize)) {
  let mut lines: Vec<Line> = Vec::new();
  let mut max_x = 0;
  let mut max_y = 0;

  for line in text.lines() {
    let (x1y1, x2y2) = line.split_once(" -> ").unwrap();
    let (x1, y1) = x1y1.split_once(",").unwrap();
    let (x2, y2) = x2y2.split_once(",").unwrap();
    let x = (x1.parse::<usize>().unwrap(), x2.parse::<usize>().unwrap());
    max_x = cmp::max(max_x, cmp::max(x.0, x.1));
    let y = (y1.parse::<usize>().unwrap(), y2.parse::<usize>().unwrap());
    max_y = cmp::max(max_y, cmp::max(y.0, y.1));
    lines.push(Line::new(x, y));
  }

  (lines, (max_x, max_y))
}

fn pt1(lines: &mut Vec<Line>, maxes: (usize, usize)) -> usize {
  let mut vents = Vents::new(maxes.0+1, maxes.1+1);
  lines.iter_mut().for_each(|line| vents.draw(line));

  vents.overlap(2)
}

fn main() {
  let text = fs::read_to_string("./assets/input.txt")
    .expect("Unable to read input file");

  let (mut lines, maxes) = parse(text);

  let pt1_sol = pt1(&mut lines, maxes);
  println!("pt1: {}", pt1_sol);
}
