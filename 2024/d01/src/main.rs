use std::collections::HashMap;

fn pt1(left: &Vec<i64>, right: &Vec<i64>) -> i64 {
    left.into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn pt2(left: &Vec<i64>, rightset: &HashMap<i64, i64>) -> i64 {
    left.into_iter()
        .map(|l| rightset.get(l).map_or(0, |count| *count * l))
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../assets/input.txt");
    let len = input.lines().count();
    let mut left = Vec::with_capacity(len);
    let mut right = Vec::with_capacity(len);
    let mut rightset = HashMap::new();

    for line in input.lines() {
        let parts: Vec<Option<i64>> = line.split_whitespace().map(|n| n.parse().ok()).collect();
        if parts.len() != 2 {
            return Err("bad input".into());
        }

        match (parts[0], parts[1]) {
            (Some(l), Some(r)) => {
                left.push(l);
                right.push(r);
                *rightset.entry(r).or_insert(0) += 1;
            }
            _ => {
                return Err("bad input".into());
            }
        }
    }

    left.sort();
    right.sort();

    println!("pt1: {}", pt1(&left, &right));
    println!("pt2: {}", pt2(&left, &rightset));
    Ok(())
}
