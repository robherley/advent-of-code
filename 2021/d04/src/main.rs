use std::fmt;
use std::collections::HashSet;

struct Board {
  board: Vec<Vec<(u32, bool)>>,
  solutions: Vec<Vec<u32>>
}

impl Board {
  fn new(board: Vec<Vec<(u32, bool)>>) -> Self {
    Self {
      board: board,
      solutions: Vec::new()
    }
  }

  fn is_winner(&self) -> bool {
    !self.solutions.is_empty()
  }

  fn draw(&mut self, drawn_num: u32) -> bool {
    let mut found: Option<(usize, usize)> = None;
    'draw_loop: for (row, cols) in self.board.iter().enumerate() {
      for (col, spot) in cols.iter().enumerate() {
        if spot.0 == drawn_num {
          self.board[row][col] = (spot.0, true);
          found = Some((row, col));
          break 'draw_loop
        }
      }
    }

    if found == None {
      return false
    }
    let found = found.unwrap();

    let mut possible_row: Vec<u32> = Vec::new();
    for col in &self.board[found.0] {
      if col.1 { possible_row.push(col.0) }
    }
    if possible_row.len() == self.board[found.1].len() {
      self.solutions.push(possible_row)
    }

    let mut possible_col: Vec<u32> = Vec::new();
    for row in &self.board {
      if row[found.1].1 {possible_col.push(row[found.0].0) }
    }
    if possible_col.len() == self.board.len() {
      self.solutions.push(possible_col);
    }

    self.is_winner()
  }


  fn unmarked(&self) -> Vec<u32> {
    let mut unmarked_nums: Vec<u32> = Vec::new();

    for row in &self.board {
      for num in row {
        if !num.1 {
          unmarked_nums.push(num.0)
        }
      }
    }

    unmarked_nums
  }
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let title = format!("win? {}", self.is_winner());
      write!(f, "{:^15}\n", title).unwrap();
      for row in &self.board {
        for spot in row {
          let format = if spot.1 { format!("\x1b[1;32m{:^3}\x1b[0m", spot.0) } else { format!("{:^3}", spot.0) };
          write!(f, "{}", format).unwrap();
        }
        write!(f, "\n").unwrap();
      }

      Ok(())
  }
}

fn parse(text: String) -> (Vec<u32>, Vec<Board>) {
  let mut boards: Vec<Board> = Vec::new();
  let mut bingo_nums: Vec<u32> = Vec::new();
  let mut row_buffer: Vec<Vec<(u32, bool)>> = Vec::new();

  for (i, line) in text.lines().enumerate() {
    if i == 0 {
      bingo_nums = line.split(",").map(|num| num.parse::<u32>().unwrap()).collect();
    } else if line.len() == 0 {
      if row_buffer.len() != 0 {
        let b = Board::new(row_buffer);
        boards.push(b);
        row_buffer = Vec::new();
      }
    } else {
      let row: Vec<(u32, bool)> = line.split(" ").filter(|str| str.len() != 0).map(|num| {
        let n = num.parse::<u32>().unwrap();
        (n, false)
      }).collect();
      row_buffer.push(row);
    }
  }

  if row_buffer.len() != 0 {
    let b = Board::new(row_buffer);
    boards.push(b);
  }

  (bingo_nums, boards)
}

fn pt1(bingo_nums: &Vec<u32>, boards: &mut Vec<Board>) -> Option<u32> {
  for num in bingo_nums {
    for board in boards.iter_mut() {
      let is_winner = board.draw(*num);
      if is_winner {
        let sum_unmarked: u32 = board.unmarked().iter().sum();
        return Some(sum_unmarked * num)
      }
    }
  }

  None
}

fn pt2(bingo_nums: &Vec<u32>, boards: &mut Vec<Board>) -> Option<u32> {
  let mut last_winner: Option<(usize, u32)> = None;
  let mut winning_boards = HashSet::new();
  let num_boards = boards.len();

  'draw_loop: for num in bingo_nums {
    for (i, board) in boards.iter_mut().enumerate() {
      let is_winner = board.draw(*num);
      if is_winner {
        last_winner = Some((i, *num));
        winning_boards.insert(i);
        if winning_boards.len() == num_boards {
          break 'draw_loop;
        }
      }
    }
  }

  match last_winner {
    None => None,
    Some(winner) => {
      let sol = boards[winner.0].unmarked().iter().sum::<u32>() * winner.1;
      Some(sol)
    }
  }
}

fn main() {
  let text = std::fs::read_to_string("./assets/input.txt")
    .expect("Unable to read input file");

  let (bingo_nums, mut boards) = parse(text);

  let pt1_sol = pt1(&bingo_nums, &mut boards);
  match pt1_sol {
    None => println!("pt1: no solution found :("),
    Some(sol) => println!("pt1: {}", sol),
  }

  let pt2_sol = pt2(&bingo_nums, &mut boards);
  match pt2_sol {
    None => println!("pt2: no solution found :("),
    Some(sol) => println!("pt2: {}", sol),
  }
}
