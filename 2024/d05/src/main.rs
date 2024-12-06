use std::{
    cmp::Ordering::*,
    collections::{HashMap, HashSet},
};

fn parse(
    input: &str,
) -> Result<(HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>), Box<dyn std::error::Error>> {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    if parts.len() != 2 {
        Err("bad input")?;
    }

    let mut rules = HashMap::new();
    for line in parts[0].lines() {
        let r = line
            .split("|")
            .map(|n| n.parse())
            .collect::<Result<Vec<i32>, _>>()
            .map_err(|_| "bad rule")?;
        match (r.get(0), r.get(1)) {
            (Some(n), Some(after)) => {
                let entry = rules.entry(*n).or_insert(HashSet::new());
                entry.insert(*after);
            }
            _ => Err("bad rule")?,
        }
    }

    let updates = parts[1]
        .lines()
        .map(|l| l.split(",").map(|n| n.parse()).collect())
        .collect::<Result<Vec<Vec<i32>>, _>>()?;

    Ok((rules, updates))
}

fn partition_updates(
    rules: &HashMap<i32, HashSet<i32>>,
    updates: &Vec<Vec<i32>>,
) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    updates.iter().cloned().partition(|update| {
        let mut seen = HashSet::new();
        update.iter().all(|&n| {
            if let Some(after) = rules.get(&n) {
                if after.intersection(&seen).count() != 0 {
                    return false;
                }
            }
            seen.insert(n);
            true
        })
    })
}

fn pt1(good: &Vec<Vec<i32>>) -> i32 {
    good.iter().map(|u| u[u.len() / 2]).sum()
}

fn pt2(bad: &Vec<Vec<i32>>, rules: &HashMap<i32, HashSet<i32>>) -> i32 {
    bad.iter()
        .map(|update| {
            let mut sorted = update.clone();
            sorted.sort_by(|a, b| {
                if let Some(after) = rules.get(a) {
                    if after.contains(b) {
                        return Less;
                    } else {
                        return Greater;
                    }
                }
                Equal
            });

            sorted[sorted.len() / 2]
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (rules, updates) = parse(include_str!("../assets/input.txt"))?;
    let (good, bad) = partition_updates(&rules, &updates);

    println!("pt1: {}", pt1(&good));
    println!("pt2: {}", pt2(&bad, &rules));
    Ok(())
}
