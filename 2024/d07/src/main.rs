fn valid(value: i64, ops: &[i64], acc: i64, pt2: bool) -> Result<bool, Box<dyn std::error::Error>> {
    if ops.is_empty() {
        return Ok(acc == value);
    }

    if acc > value {
        return Ok(false);
    }

    let mut concatted = false;
    if pt2 && acc != 0 {
        let concat = (acc.to_string() + &ops[0].to_string())
            .parse::<i64>()
            .or(Err("concat bad"))?;
        concatted = valid(value, &ops[1..], concat, pt2)?;
    }

    Ok(concatted
        || valid(value, &ops[1..], acc * ops[0], pt2)?
        || valid(value, &ops[1..], acc + ops[0], pt2)?)
}

fn solve(input: &Vec<(i64, Vec<i64>)>, pt2: bool) -> Result<i64, Box<dyn std::error::Error>> {
    input
        .into_iter()
        .map(|(goal, rest)| {
            if valid(*goal, &rest, 0, pt2)? {
                Ok(*goal)
            } else {
                Ok(0)
            }
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../assets/input.txt")
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() != 2 {
                return Err("expected 2 parts");
            }
            let value: i64 = parts[0].parse().or(Err("expected number"))?;
            let rest: Vec<i64> = parts[1]
                .split_whitespace()
                .map(|x| x.parse::<i64>())
                .collect::<Result<Vec<i64>, _>>()
                .or(Err("expected numbers"))?;
            Ok((value, rest))
        })
        .collect::<Result<Vec<(i64, Vec<i64>)>, _>>()?;

    println!("pt1: {}", solve(&input, false)?);
    println!("pt2: {}", solve(&input, true)?);

    Ok(())
}
