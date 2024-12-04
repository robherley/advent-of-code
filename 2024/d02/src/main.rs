use std::num::ParseIntError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Inc,
    Dec,
}

fn is_safe_levels_alt(input: &Vec<i32>, dampener: bool) -> bool {
    let mut state: Option<State> = None;
    let mut iter = input.into_iter().enumerate().peekable();

    while let Some((ia, a)) = iter.next() {
        let (ib, b) = match iter.peek() {
            Some((ib, b)) => (ib, *b),
            None => break,
        };

        match (state, b - a) {
            (None, diff) if diff.abs() >= 1 && diff.abs() <= 3 => {
                state = Some(if diff.is_positive() {
                    State::Inc
                } else {
                    State::Dec
                });
            }
            (Some(State::Inc), diff) if diff >= 1 && diff <= 3 => continue,
            (Some(State::Dec), diff) if diff >= -3 && diff <= -1 => continue,
            _ => {
                if !dampener {
                    return false;
                }

                let mut without_a = input.clone();
                without_a.remove(ia);
                let mut without_b = input.clone();
                without_b.remove(*ib);

                return is_safe_levels_alt(&without_a, false)
                    || is_safe_levels_alt(&without_b, false);
            }
        }
    }

    true
}

fn is_safe_levels(input: &Vec<i32>, dampener: bool) -> bool {
    let diff = input.windows(2).map(|w| w[1] - w[0]);
    let inc = diff.clone().all(|d| d >= 1 && d <= 3);
    let dec = diff.clone().all(|d| d >= -3 && d <= -1);

    if inc || dec {
        return true;
    }

    if !dampener {
        return false;
    }

    // instead of brute force removal, can we do this more efficiently?
    for i in 0..input.len() {
        let mut without = input.clone();
        without.remove(i);
        if is_safe_levels(&without, false) {
            return true;
        }
    }

    false
}

fn solve(input: &Vec<Vec<i32>>, dampener: bool) -> usize {
    input
        .into_iter()
        .filter(|row| is_safe_levels(row, dampener))
        .count()
}

fn parse_input(input: &str) -> Result<Vec<Vec<i32>>, ParseIntError> {
    input
        .lines()
        .map(|line| line.split_whitespace().map(|x| x.parse()).collect())
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = parse_input(include_str!("../assets/input.txt"))?;
    println!("pt1: {}", solve(&input, false));
    println!("pt2: {}", solve(&input, true));
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn pt1() {
        let input = parse_input(include_str!("../assets/input.txt")).expect("bad input");
        assert_eq!(solve(&input, false), 442);
    }

    #[test]
    fn pt2() {
        let input = parse_input(include_str!("../assets/input.txt")).expect("bad input");
        assert_eq!(solve(&input, true), 493);
    }

    #[test]
    fn diff() {
        let input = parse_input(include_str!("../assets/input.txt")).expect("bad input");
        let original: HashSet<usize> = input
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, row)| is_safe_levels(row, true))
            .map(|(i, _)| i)
            .collect();
        let alt: HashSet<usize> = input
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, row)| is_safe_levels_alt(row, true))
            .map(|(i, _)| i)
            .collect();

        let intersect: Vec<&usize> = original.difference(&alt).collect();
        for i in &intersect {
            println!("{}: {:?}", i, input[**i]);
        }

        assert!(intersect.is_empty(), "expected {:?} to be empty", intersect);
    }

    #[test]
    fn safe_levels() {
        let cases = vec![
            (vec![7, 6, 4, 2, 1], true),
            (vec![1, 2, 7, 8, 9], false),
            (vec![9, 7, 6, 2, 1], false),
            (vec![1, 3, 2, 4, 5], false),
            (vec![8, 6, 4, 4, 1], false),
            (vec![1, 3, 6, 7, 9], true),
        ];

        for (input, want) in cases {
            assert_eq!(
                is_safe_levels_alt(&input, false),
                want,
                "expected {:?} to be {}",
                input,
                want
            );
        }
    }

    #[test]
    fn safe_levels_with_damper() {
        let cases = vec![
            (vec![7, 6, 4, 2, 1], true),
            (vec![1, 2, 7, 8, 9], false),
            (vec![9, 7, 6, 2, 1], false),
            (vec![1, 3, 2, 4, 5], true),
            (vec![8, 6, 4, 4, 1], true),
            (vec![1, 3, 6, 7, 9], true),
        ];

        for (input, want) in cases {
            assert_eq!(
                is_safe_levels_alt(&input, true),
                want,
                "expected {:?} to be {}",
                input,
                want
            );
        }
    }
}
