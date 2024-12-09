use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Node {
    Antenna(char),
    AntiNode,
    Empty,
}

fn distance(a: (usize, usize), b: (usize, usize)) -> (isize, isize) {
    (
        (a.0 as isize - b.0 as isize) as isize,
        (a.1 as isize - b.1 as isize) as isize,
    )
}

fn delta(point: (usize, usize), d: (isize, isize)) -> (isize, isize) {
    let point = (point.0 as isize, point.1 as isize);
    (point.0 + d.0, point.1 + d.1)
}

fn set_anti(nodes: &mut Vec<Vec<Node>>, point: (isize, isize)) -> Option<bool> {
    if point.0.is_negative() || point.1.is_negative() {
        return None;
    }

    if let Some(row) = nodes.get_mut(point.1 as usize) {
        return match row.get(point.0 as usize) {
            Some(Node::Empty) => {
                row[point.0 as usize] = Node::AntiNode;
                Some(true)
            }
            Some(Node::Antenna(_)) => Some(true),
            Some(Node::AntiNode) => Some(false),
            None => None,
        };
    }

    None
}

fn main() {
    let mut antennas = HashMap::new();
    let mut antinodes = HashSet::new();

    let mut nodes = include_str!("../assets/input.txt")
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| match ch {
                    '.' => Node::Empty,
                    ch if ch.is_ascii_alphanumeric() => {
                        antennas.entry(ch).or_insert(vec![]).push((x, y));
                        Node::Antenna(ch)
                    }
                    _ => panic!("invalid char: {}", ch),
                })
                .collect::<Vec<Node>>()
        })
        .collect::<Vec<Vec<Node>>>();

    for k in antennas.keys() {
        if let Some(matches) = antennas.get(k) {
            let combos: Vec<Vec<&(usize, usize)>> = matches.iter().combinations(2).collect();
            for combo in &combos {
                let distance1 = distance(*combo[0], *combo[1]);
                let mut point = *combo[0];
                loop {
                    let anti = delta(point, distance1);
                    match set_anti(&mut nodes, anti) {
                        Some(true) => {
                            antinodes.insert(anti);
                        }
                        Some(false) => {}
                        None => {
                            break;
                        }
                    }
                    point = (anti.0 as usize, anti.1 as usize);
                }

                let distance2 = distance(*combo[1], *combo[0]);
                let mut point = *combo[1];
                loop {
                    let anti = delta(point, distance2);
                    match set_anti(&mut nodes, anti) {
                        Some(true) => {
                            antinodes.insert(anti);
                        }
                        Some(false) => {}
                        None => {
                            break;
                        }
                    }
                    point = (anti.0 as usize, anti.1 as usize);
                }
            }
        }
    }

    let mut total_anti = antinodes.len();
    nodes.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, node)| match node {
            Node::Empty => print!("."),
            Node::Antenna(ch) => {
                // this is ugly, add these to the hashset earlier
                if !antinodes.contains(&(x as isize, y as isize)) {
                    total_anti += 1;
                }

                print!("{}", ch)
            }
            Node::AntiNode => {
                print!("#")
            }
        });
        println!();
    });

    println!("antinodes: {}", total_anti);
}
