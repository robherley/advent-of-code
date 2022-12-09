fn pt1(trees: &Vec<Vec<u32>>, debug: bool) -> i32 {
  let height = trees.len();
  let width = trees[0].len();

  let mut visible = vec![vec![false; width]; height];
  for i in 0..height {
    for j in 0..width {
      if i == 0 || j == 0 || i == height - 1 || j == width - 1 {
        visible[i][j] = true;
        continue;
      }

      let tree_height = trees[i][j];

      // left
      if *trees[i][0..j].iter().max().unwrap() < tree_height {
        visible[i][j] = true;
        continue;
      }
      // right
      if *trees[i][j+1..width].iter().max().unwrap() < tree_height {
        visible[i][j] = true;
        continue;
      }
      // top
      if trees[0..i].iter().map(|row| row[j]).max().unwrap() < tree_height {
        visible[i][j] = true;
        continue;
      }
      // bottom
      if trees[i+1..height].iter().map(|row| row[j]).max().unwrap() < tree_height {
        visible[i][j] = true;
        continue;
      }
    }
  }

  let mut count = 0;
  for i in 0..height {
    for j in 0..width {
      if visible[i][j] {
        count += 1;
      }
      if debug { print!("{}", if visible[i][j] {1} else {0}) };
    }
    if debug { println!() };
  }

  count
}


fn pt2(trees: &Vec<Vec<u32>>, debug: bool) -> i32 {
  let height = trees.len();
  let width = trees[0].len();

  fn view_score(tree_height: u32, trees: &Vec<u32>) -> i32 {
    let mut score = 0;
    for tree in trees {
      score += 1;
      if *tree >= tree_height {
        break;
      }
    }
    score
  }

  let mut score = 0;
  for i in 0..height {
    for j in 0..width {
      let tree_height = trees[i][j];

      // left score
      let left = trees[i][0..j].iter().rev().map(|x| *x).collect::<Vec<u32>>();
      let left_score = view_score(tree_height, &left);
      // right
      let right = trees[i][j+1..width].to_vec();
      let right_score = view_score(tree_height, &right);
      // top
      let top =  trees[0..i].iter().map(|row| row[j]).rev().collect::<Vec<u32>>();
      let top_score = view_score(tree_height, &top);
      // bottom
      let bottom = trees[i+1..height].iter().map(|row| row[j]).collect::<Vec<u32>>();
      let bottom_score = view_score(tree_height, &bottom);

      if debug {
        println!("{} => {} {} {} {}", tree_height, top_score, left_score, right_score, bottom_score);
        println!("  up: {:?} {}", top, top_score);
        println!("  left: {:?} {}", left, left_score);
        println!("  right: {:?} {}", right, right_score);
        println!("  down: {:?} {}", bottom, bottom_score);
      }

      let new_score = left_score * right_score * top_score * bottom_score;
      if new_score > score {
        score = new_score;
      }
    }
  }

  score
}

fn main() {
  let input = include_str!("../assets/input.txt");
  let trees: Vec<Vec<u32>> = input
    .lines()
    .map(|line| {
      line.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
    })
    .collect();

  println!("[pt1]: {}", pt1(&trees, false));
  println!("[pt2]: {}", pt2(&trees, false));
}
