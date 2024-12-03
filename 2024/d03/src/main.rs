use regex::Regex;

fn main() {
    let input = include_str!("../assets/input.txt");
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").expect("skill issue");

    let mut enabled = true;
    let (pt1, pt2) = re
        .captures_iter(input)
        .fold((0, 0), |(mut pt1, mut pt2), cap| {
            match &cap[0] {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    let a = &cap[1].parse::<i64>().expect("bad input");
                    let b = &cap[2].parse::<i64>().expect("bad input");
                    pt1 += a * b;
                    if enabled {
                        pt2 += a * b;
                    }
                }
            }
            (pt1, pt2)
        });

    println!("pt1: {}", pt1);
    println!("pt2: {}", pt2);
}
