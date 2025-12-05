fn pt1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut jolts = (0, 0);
        for (idx, digit) in digits.iter().enumerate() {
            if idx != digits.len() - 1 {
                if jolts.0 < *digit {
                    jolts.0 = *digit;
                    jolts.1 = 0;
                    continue;
                }
            }

            if jolts.1 < *digit {
                jolts.1 = *digit;
                continue;
            }
        }

        sum += (jolts.0 * 10) + jolts.1;
    }

    sum
}

fn pt2(input: &str) -> u64 {
    let n_batteries = 12;
    let mut sum = 0;
    for line in input.lines() {
        let mut joltstack = Vec::new();
        let max_pops = line.len() - n_batteries;
        let mut n_pops = 0;

        for digit in line.chars() {
            loop {
                if n_pops >= max_pops {
                    break;
                }

                match joltstack.last() {
                    Some(&last) if last < digit => {
                        joltstack.pop();
                        n_pops += 1;
                    }
                    _ => break,
                }
            }

            joltstack.push(digit);
        }

        sum += joltstack
            .into_iter()
            .take(n_batteries)
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
    }

    sum
}

fn main() {
    let input = include_str!("../assets/input.txt");
    println!("pt1: {}", pt1(input));
    println!("pt1: {}", pt2(input));
}
