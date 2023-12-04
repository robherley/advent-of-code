use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
enum Kind {
    Point,
    Symbol(char),
    Number(char),
}

fn parse(input: &str) -> Vec<Vec<Kind>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Kind::Point,
                    '0'..='9' => Kind::Number(c),
                    _ => Kind::Symbol(c),
                })
                .collect()
        })
        .collect()
}

// caveat: if the same part number is around the symbol more than once, it'll only be counted once
fn find_surrounding(input: &Vec<Vec<Kind>>, x: i32, y: i32) -> HashSet<i32> {
    let possible = vec![
        // above
        (x, y - 1),
        // upper right
        (x + 1, y - 1),
        // right
        (x + 1, y),
        // lower right
        (x + 1, y + 1),
        // below
        (x, y + 1),
        // lower left
        (x - 1, y + 1),
        // left
        (x - 1, y),
        // upper left
        (x - 1, y - 1),
    ];

    possible
        .into_iter()
        .map(|(x, y)| {
            if x < 0 || y < 0 {
                return None;
            }
            if y >= input.len() as i32 || x >= input[y as usize].len() as i32 {
                return None;
            }
            match &input[y as usize][x as usize] {
                Kind::Number(_) => Some(find_whole_number(input, x as usize, y as usize)),
                _ => None,
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect()
}

fn find_whole_number(input: &Vec<Vec<Kind>>, x: usize, y: usize) -> i32 {
    let mut result = VecDeque::new();
    match &input[y][x] {
        Kind::Number(c) => result.push_front(*c),
        _ => panic!("not a number"),
    }

    let mut lx = x;
    loop {
        if lx == 0 {
            break;
        }
        lx -= 1;
        match &input[y][lx] {
            Kind::Number(c) => result.push_front(*c),
            _ => break,
        }
    }

    let mut rx = x;
    loop {
        if rx == input[y].len() - 1 {
            break;
        }
        rx += 1;
        match &input[y][rx] {
            Kind::Number(c) => result.push_back(*c),
            _ => break,
        }
    }

    result.iter().collect::<String>().parse::<i32>().unwrap()
}

fn solve(parsed: &Vec<Vec<Kind>>) -> (i32, i32) {
    let mut part_numbers = vec![];
    let mut ratios = vec![];

    for (y, line) in parsed.iter().enumerate() {
        for (x, kind) in line.iter().enumerate() {
            match kind {
                Kind::Symbol(c) => {
                    let surrounding = find_surrounding(&parsed, x as i32, y as i32);
                    part_numbers.extend(&surrounding);
                    if *c == '*' && surrounding.len() == 2 {
                        ratios.push(surrounding.iter().product::<i32>());
                    }
                }
                _ => (),
            }
        }
    }

    (part_numbers.iter().sum::<i32>(), ratios.iter().sum::<i32>())
}

fn main() {
    let input = include_str!("../assets/input.txt");
    let parsed = parse(input);
    let (pt1, pt2) = solve(&parsed);
    println!("pt1: {:?}", pt1);
    println!("pt2: {:?}", pt2);
}
