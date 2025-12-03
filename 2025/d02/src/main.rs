fn is_invalid_pt1(id: &String) -> bool {
    if id.len() % 2 != 0 {
        return false;
    }

    let (left, right) = id.split_at(id.len() / 2);
    left == right
}

fn is_invalid_pt2(id: &String) -> bool {
    if id.len() == 1 {
        return false;
    }
    let chars = id.chars().collect::<Vec<char>>();
    for i in 1..=chars.len() / 2 {
        let mut iter = chars.chunks(i);
        let first = iter.next().unwrap();
        if iter.all(|chunk| chunk == first) {
            return true;
        }
    }
    false
}

fn solve(input: &str) -> (u64, u64) {
    let mut pt1 = 0;
    let mut pt2 = 0;

    for range in input.split(",") {
        let (start, end) = range
            .split_once("-")
            .map(|(start, end)| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
            .unwrap();

        for n in start..=end {
            let id = n.to_string();
            if is_invalid_pt1(&id) {
                pt1 += n;
            }
            if is_invalid_pt2(&id) {
                pt2 += n;
            }
        }
    }

    (pt1, pt2)
}

fn main() {
    let input = include_str!("../assets/input.txt");
    let (pt1, pt2) = solve(input);
    println!("pt1: {}", pt1);
    println!("pt2: {}", pt2);
}
