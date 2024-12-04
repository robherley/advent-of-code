const PT1_POSITIONS: [[(isize, isize); 3]; 8] = [
    // right to left
    [(0, 1), (0, 2), (0, 3)],
    // left to right
    [(0, -1), (0, -2), (0, -3)],
    // top to bottom
    [(1, 0), (2, 0), (3, 0)],
    // bottom to top
    [(-1, 0), (-2, 0), (-3, 0)],
    // top right
    [(1, 1), (2, 2), (3, 3)],
    // bottom right
    [(-1, 1), (-2, 2), (-3, 3)],
    // top left
    [(1, -1), (2, -2), (3, -3)],
    // bottom left
    [(-1, -1), (-2, -2), (-3, -3)],
];

const PT2_POSITIONS: [[(isize, isize); 2]; 2] = [
    // top left bottom right
    [(1, 1), (-1, -1)],
    // bottom left top right
    [(-1, 1), (1, -1)],
];

fn get(input: &Vec<Vec<char>>, i: usize, j: usize, delta: (isize, isize)) -> Option<char> {
    let di = i as isize + delta.0;
    let dj = j as isize + delta.1;
    if di.is_negative() || dj.is_negative() {
        return None;
    }

    let ch = input
        .get(di as usize)
        .and_then(|row| row.get(dj as usize))?;
    Some(*ch)
}

fn pt1(input: &Vec<Vec<char>>) -> i32 {
    let mut found = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] != 'X' {
                continue;
            }

            for [m, a, s] in PT1_POSITIONS {
                if get(&input, i, j, m) != Some('M') {
                    continue;
                }

                if get(&input, i, j, a) != Some('A') {
                    continue;
                }

                if get(&input, i, j, s) != Some('S') {
                    continue;
                }

                found += 1;
            }
        }
    }

    found
}

fn pt2(input: &Vec<Vec<char>>) -> i32 {
    let mut found = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] != 'A' {
                continue;
            }

            let mut bad = false;
            for [l, r] in PT2_POSITIONS {
                let neighbors = (get(&input, i, j, l), get(&input, i, j, r));
                match neighbors {
                    (Some('S'), Some('M')) => (),
                    (Some('M'), Some('S')) => (),
                    _ => {
                        bad = true;
                        break;
                    }
                }
            }
            if !bad {
                found += 1;
            }
        }
    }

    found
}

fn main() {
    let input: Vec<Vec<char>> = include_str!("../assets/input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    println!("pt1: {:?}", pt1(&input));
    println!("pt2: {:?}", pt2(&input));
}
