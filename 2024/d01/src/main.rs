use std::collections::HashMap;

fn pt1(left: &Vec<i64>, right: &Vec<i64>) -> i64 {
    left.into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i64>()
}

fn pt2(left: &Vec<i64>, rightset: &HashMap<i64, i64>) -> i64 {
    left.into_iter()
        .map(|l| {
            if let Some(count) = rightset.get(&l) {
                l * count
            } else {
                0
            }
        })
        .sum::<i64>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../assets/input.txt");
    let len = input.lines().count();
    let mut left = Vec::with_capacity(len);
    let mut right = Vec::with_capacity(len);
    let mut rightset = HashMap::new();

    for line in input.lines() {
        let parts: Vec<Option<i64>> = line
            .split_whitespace()
            .map(|n| n.parse::<i64>().ok())
            .collect();
        if parts.len() != 2 {
            return Err("bad input".into());
        }

        match (parts[0], parts[1]) {
            (Some(l), Some(r)) => {
                left.push(l);
                right.push(r);
                if rightset.contains_key(&r) {
                    let count = rightset.get_mut(&r).unwrap();
                    *count += 1;
                } else {
                    rightset.insert(r, 1);
                }
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
