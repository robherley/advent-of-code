const MAX: i32 = 100;
const START: i32 = 50;

fn solve(input: &str) -> (i32, i32) {
    let mut pt1 = 0;
    let mut pt2 = 0;
    let mut result = START;
    for line in input.lines() {
        let mut chars = line.chars();
        let op = if chars.next().unwrap() == 'R' { 1 } else { -1 };
        let n = chars.as_str().parse::<i32>().unwrap();

        result = {
            let mut res = result + n * op;

            if op.is_positive() {
                pt2 += res.abs() / MAX;
            } else {
                pt2 += n / MAX;
                if result != 0 && n % MAX >= result {
                    pt2 += 1;
                }
            }

            res %= MAX;
            if res < 0 { res + MAX } else { res }
        };

        if result == 0 {
            pt1 += 1;
        }
    }

    (pt1, pt2)
}

fn main() {
    let input = include_str!("../assets/input.txt");
    let (pt1, pt2) = solve(&input);
    println!("pt1: {}", pt1);
    println!("pt2: {}", pt2);
}
