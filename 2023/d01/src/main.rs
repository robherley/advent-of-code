fn pt1(input: &str) -> u32 {
    input.lines().map(|line| {
        let mut first = None;
        let mut last = None;

        line.chars().for_each(|c| {
            if c.is_digit(10) {
                let num = c.to_digit(10).unwrap();
                if first.is_none() {
                    first = Some(num);
                }
                last = Some(num);
            }
        });

        first.unwrap() * 10 + last.unwrap()
    }).sum()
}

fn pt2(input: &str) -> u32 {
    input.lines().map(|line| {
        let mut first = None;
        let mut last = None;

        line.char_indices().for_each(|(i, c)| {
            let n ;
            if c.is_digit(10) {
                n = c.to_digit(10);
            } else {
                n = match &line[i..] {
                    s if s.starts_with("one") => Some(1),
                    s if s.starts_with("two") => Some(2),
                    s if s.starts_with("three") => Some(3),
                    s if s.starts_with("four") => Some(4),
                    s if s.starts_with("five") => Some(5),
                    s if s.starts_with("six") => Some(6),
                    s if s.starts_with("seven") => Some(7),
                    s if s.starts_with("eight") => Some(8),
                    s if s.starts_with("nine") => Some(9),
                    _ => None
                };
            };

            if n.is_some() {
                if first.is_none() {
                    first = n;
                }
                last = n;
            }
        });

        first.unwrap() * 10 + last.unwrap()
    }).sum()
}

fn main() {
    let input = include_str!("../assets/input.txt");
    println!("Part 1: {}", pt1(input));
    println!("Part 2: {}", pt2(input));
}
