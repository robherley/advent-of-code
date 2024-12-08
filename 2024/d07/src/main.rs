fn valid(value: i64, ops: &[i64], acc: i64, pt2: bool) -> bool {
    if ops.is_empty() {
        return acc == value;
    }

    if acc > value {
        return false;
    }

    let mut concatted = false;
    if pt2 && acc != 0 {
        let first_op = ops[0];
        let digits = (first_op as f64).log10().floor() as i64 + 1;
        let concat = acc * 10_i64.pow(digits as u32) + first_op;
        concatted = valid(value, &ops[1..], concat, pt2);
    }

    concatted
        || valid(value, &ops[1..], acc * ops[0], pt2)
        || valid(value, &ops[1..], acc + ops[0], pt2)
}

fn solve(input: &Vec<(i64, Vec<i64>)>, pt2: bool) -> i64 {
    input
        .into_iter()
        .map(|(goal, rest)| {
            if valid(*goal, &rest, 0, pt2) {
                *goal
            } else {
                0
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

    println!("pt1: {}", solve(&input, false));
    println!("pt2: {}", solve(&input, true));

    Ok(())
}
