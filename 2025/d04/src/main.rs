const POSITIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub struct Grid {
    grid: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let mut grid = Vec::new();
        for line in input.lines() {
            let row = line.chars().map(|c| c == '@').collect();
            grid.push(row);
        }
        Grid { grid }
    }

    pub fn solve(&mut self) -> (i32, i32) {
        let pt1 = self.remove_paper();
        let mut pt2 = pt1;

        loop {
            let removed = self.remove_paper();
            pt2 += removed;
            if removed == 0 {
                break;
            }
        }

        (pt1, pt2)
    }

    pub fn remove_paper(&mut self) -> i32 {
        let mut count = 0;
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                if !self.is_paper(row as i32, col as i32) {
                    continue;
                }

                if self.surrounding_paper(row, col) < 4 {
                    to_remove.push((row, col));
                    count += 1;
                }
            }
        }

        for (row, col) in to_remove {
            self.grid[row][col] = false;
        }

        count
    }

    fn surrounding_paper(&self, row: usize, col: usize) -> usize {
        POSITIONS
            .iter()
            .filter(|&&(r, c)| self.is_paper(row as i32 + r, col as i32 + c))
            .count()
    }

    fn is_paper(&self, row: i32, col: i32) -> bool {
        if row < 0 || col < 0 {
            return false;
        }

        self.grid
            .get(row as usize)
            .map_or(false, |row| row.get(col as usize).cloned().unwrap_or(false))
    }
}

fn main() {
    let input = include_str!("../assets/input.txt");
    let mut grid = Grid::new(input);
    let (pt1, pt2) = grid.solve();
    println!("pt1: {}", pt1);
    println!("pt2: {}", pt2);
}
