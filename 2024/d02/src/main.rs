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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../assets/input.txt")
        .lines()
        .map(|line| line.split_whitespace().map(|x| x.parse()).collect())
        .collect::<Result<Vec<Vec<i32>>, _>>()?;
    println!("pt1: {}", solve(&input, false));
    println!("pt2: {}", solve(&input, true));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
                is_safe_levels(&input, false),
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
                is_safe_levels(&input, true),
                want,
                "expected {:?} to be {}",
                input,
                want
            );
        }
    }
}
