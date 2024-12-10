use itertools::Itertools;
use std::collections::{HashMap, HashSet};

struct Scanner {
    nodes: Vec<Vec<Option<char>>>,
    antennas: HashMap<char, Vec<(usize, usize)>>,
}

impl Scanner {
    fn solve(&self, pt2: bool) -> usize {
        self.find_antinodes(pt2).len()
    }

    fn find_antinodes(&self, pt2: bool) -> HashSet<(usize, usize)> {
        let mut antinodes = HashSet::new();

        if pt2 {
            self.antennas.values().for_each(|antenna| {
                antinodes.extend(antenna);
            });
        }

        for freq in self.antennas.keys() {
            if let Some(matches) = self.antennas.get(freq) {
                let combos: Vec<Vec<&(usize, usize)>> = matches.iter().combinations(2).collect();
                for combo in &combos {
                    self.resonance(
                        &mut antinodes,
                        *combo[0],
                        Scanner::distance(*combo[0], *combo[1]),
                        pt2,
                    );
                    self.resonance(
                        &mut antinodes,
                        *combo[1],
                        Scanner::distance(*combo[1], *combo[0]),
                        pt2,
                    );
                }
            }
        }

        antinodes
    }

    fn resonance(
        &self,
        antinodes: &mut HashSet<(usize, usize)>,
        point: (usize, usize),
        distance: (isize, isize),
        pt2: bool,
    ) {
        let mut point = point;
        loop {
            match self.delta(point, distance) {
                Some(antinode) => {
                    antinodes.insert(antinode);
                    point = antinode;
                }
                None => return,
            }
            if !pt2 {
                return;
            }
        }
    }

    fn delta(&self, p: (usize, usize), d: (isize, isize)) -> Option<(usize, usize)> {
        let delta = (p.0 as isize + d.0, p.1 as isize + d.1);

        if delta.0.is_negative() || delta.1.is_negative() {
            return None;
        }

        match self.nodes.get(delta.1 as usize)?.get(delta.0 as usize) {
            Some(_) => Some((delta.0 as usize, delta.1 as usize)),
            None => None,
        }
    }

    fn distance(a: (usize, usize), b: (usize, usize)) -> (isize, isize) {
        (
            (a.0 as isize - b.0 as isize) as isize,
            (a.1 as isize - b.1 as isize) as isize,
        )
    }

    #[allow(dead_code)]
    fn debug(&self, pt2: bool) {
        let antinodes = self.find_antinodes(pt2);
        for (y, row) in self.nodes.iter().enumerate() {
            for (x, node) in row.iter().enumerate() {
                let is_antinode = antinodes.contains(&(x, y));
                match (node, is_antinode) {
                    (None, false) => print!("."),
                    (Some(ch), _) => print!("{}", ch),
                    (_, true) => print!("#"),
                }
            }
            println!();
        }
    }
}

impl TryFrom<&str> for Scanner {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut antennas = HashMap::new();
        let nodes = value
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, ch)| match ch {
                        '.' => Ok(None),
                        ch if ch.is_ascii_alphanumeric() => {
                            antennas.entry(ch).or_insert(vec![]).push((x, y));
                            Ok(Some(ch))
                        }
                        _ => Err("bad char"),
                    })
                    .collect::<Result<Vec<Option<char>>, _>>()
            })
            .collect::<Result<Vec<Vec<Option<char>>>, _>>()?;

        Ok(Scanner { nodes, antennas })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scanner = Scanner::try_from(include_str!("../assets/input.txt"))?;
    println!("pt1: {}", scanner.solve(false));
    println!("pt2: {}", scanner.solve(true));
    Ok(())
}
