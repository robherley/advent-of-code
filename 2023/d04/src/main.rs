use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    id: usize,
    matching: usize,
}

impl Card {
    fn from_str(raw: &str) -> Self {
        let (id, cards) = raw.split_once(":").unwrap();
        let id = id.split_whitespace().collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap();

        let (winning, actual) = cards.split_once("|").unwrap();
        let (winning, actual) = (Card::parse_numbers(winning), Card::parse_numbers(actual));

        Self {
            id,
            matching: winning.intersection(&actual).count(),
        }
    }

    fn parse_numbers(raw: &str) -> HashSet<i32> {
        raw.split_whitespace()
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<HashSet<i32>>()
    }

    fn points(&self) -> i32 {
        match self.matching {
            0 => 0,
            n => i32::from(2).pow(n as u32 - 1),
        }
    }
}

fn pt1(cards: &Vec<Card>) -> i32 {
    cards.iter().map(|c| c.points()).sum()
}

fn pt2(cards: &Vec<Card>) -> i32 {
    let mut sum = 0;
    let mut current_ids = cards.iter().map(|c| c.id).collect::<Vec<usize>>();

    loop {
        sum += current_ids.len() as i32;

        let mut winning_ids = vec![];
        current_ids.iter().for_each(|id| {
            let card = &cards[*id - 1];
            winning_ids.extend(card.id + 1..card.id + card.matching + 1);
        });

        if winning_ids.is_empty() {
            break;
        }
        current_ids = winning_ids;
    }

    sum
}

fn main() {
    let input = include_str!("../assets/input.txt");
    let cards = input.lines().map(|line| Card::from_str(line)).collect();

    println!("pt1: {}", pt1(&cards));
    println!("pt2: {}", pt2(&cards));
}
