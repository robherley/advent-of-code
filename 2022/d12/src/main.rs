use std::collections::{VecDeque, HashMap};

fn possible_moves(pos: (usize, usize), map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
  let mut possible = vec![];

  if pos.1 != 0 {
    possible.push((pos.0,pos.1-1)) // up
  }

  if pos.1 + 1 < map.len() {
    possible.push((pos.0,pos.1+1)) // down
  }

  if pos.0 != 0 {
    possible.push((pos.0-1,pos.1)) // left
  }

  if pos.0 + 1 < map[pos.1].len() {
    possible.push((pos.0+1,pos.1)) // right
  }

  let curr = map[pos.1][pos.0];
  possible.into_iter().filter(|&next_pos| {
    let next = map[next_pos.1][next_pos.0];

    (next as i32) - (curr as i32) <= 1
  }).collect::<Vec<(usize,usize)>>()
}

fn bfs(start: (usize, usize), end: (usize, usize), map: &Vec<Vec<char>>) -> Option<usize> {
  let mut possible = VecDeque::new();
  let mut seen = HashMap::new();

  possible.push_front((start, 0));

  while !possible.is_empty() {
    let (curr, distance) = possible.pop_front().unwrap();

    if seen.contains_key(&curr) {
      continue;
    }
    seen.insert(curr, true);

    if curr == end {
      return Some(distance);
    }

    for next in possible_moves(curr, map) {
      possible.push_back((next, distance + 1));
    }
  }

  None
}

fn main() {
  let input = include_str!("../assets/input.txt");
  let mut start: Option<(usize,usize)> = None;
  let mut end: Option<(usize,usize)> = None;
  let mut pt2_starting_points = vec![];
  let map = input.lines().enumerate().map(|(i, line)| {
    line.chars().enumerate().map(|(j, ch)| {
      if ch == 'S' {
        start = Some((j, i));
        pt2_starting_points.push((j, i));
        return 'a'
      }
      if ch == 'E' {
        end = Some((j, i));
        return 'z'
      }

      if ch == 'a' {
        pt2_starting_points.push((j, i));
      }

      ch
    }).collect::<Vec<char>>()
  }).collect::<Vec<Vec<char>>>();

  let result = bfs(start.unwrap(), end.unwrap(),&map);
  println!("[pt1]: {}", result.unwrap());

  // brute force bc i'm tired
  let pt2 = pt2_starting_points.into_iter()
    .map(|start| bfs(start, end.unwrap(), &map))
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .min()
    .unwrap();

  println!("[pt2]: {}", pt2);
}
