#[macro_use]
extern crate log;
use env_logger;
use std::collections::VecDeque;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Debug)]
struct Monkey {
  operation: Vec<String>,
  divisor: u64,
  throw_to: (usize, usize),
}

fn build_monkeys(input: &str) -> (Vec<Monkey>, Vec<VecDeque<u64>>) {
  lazy_static! {
    static ref NEW_MONKEY_RE: Regex = Regex::new(r"Monkey (\d+):").unwrap();
  }

  let mut monkeys: Vec<Monkey> = vec![];
  let mut inventories: Vec<VecDeque<u64>> = vec![];

  NEW_MONKEY_RE.split(input.trim()).skip(1).for_each(|monkey| {
    let mut lines = monkey.trim().lines();

    let starting_items = lines.next().unwrap().split(": ").nth(1).unwrap().split(", ").map(|x| x.parse::<u64>().unwrap()).collect::<VecDeque<u64>>();
    let operation = lines.next().unwrap().split("= old ").nth(1).unwrap().split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
    let divisor = lines.next().unwrap().split("by ").nth(1).unwrap().parse::<u64>().unwrap();
    let true_monkey = lines.next().unwrap().split("monkey ").nth(1).unwrap().parse::<usize>().unwrap();
    let false_monkey = lines.next().unwrap().split("monkey ").nth(1).unwrap().parse::<usize>().unwrap();

    inventories.push(starting_items);
    monkeys.push(Monkey {
      operation: operation,
      divisor: divisor,
      throw_to: (true_monkey, false_monkey),
    });
  });

  (monkeys, inventories)
}

fn gcd(a: u64, b: u64) -> u64 {
  if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: u64, b: u64) -> u64 {
  a * b / gcd(a, b)
}

fn find_monkey_biz(monkeys: &mut Vec<Monkey>, inventories: &mut Vec<VecDeque<u64>>, is_pt2: bool) -> u64 {
  let mut inspections = vec![0u64;monkeys.len()];

  let lcm = monkeys.iter().fold(1u64, |a, b| lcm(a, b.divisor));

  let rounds = if is_pt2 { 10000 } else { 20 };
  for round in 1..=rounds {
    for (i, monkey) in monkeys.iter_mut().enumerate() {
      trace!("Monkey {}:", i);
      let items = inventories[i].drain(0..).collect::<VecDeque<u64>>();
      for item in items {
        inspections[i] += 1;
        trace!("  Monkey inspects an item with a worry level of {}.", item);
        let addend = if monkey.operation[1] == "old" { item } else { monkey.operation[1].parse::<u64>().unwrap() };
        let mut worry_level = match &monkey.operation[0] as &str {
          "+" => item + addend,
          "-" => item - addend,
          "*" => item * addend,
          _ => panic!("unknown operation"),
        };
        trace!("    Worry level is {} by {} to {}", monkey.operation[0], addend, worry_level);
        if is_pt2 {
          worry_level = worry_level % lcm;
        } else {
          worry_level /= 3;
        }
        trace!("    Monkey gets bored with item. Worry level is divided by 3 to {}.", worry_level);
        let divisible = worry_level % monkey.divisor == 0;
        trace!("    Current worry level is {}divisible by {}.", if divisible {""} else {"not "}, monkey.divisor);
        let receiver = if divisible { monkey.throw_to.0 } else { monkey.throw_to.1 };
        trace!("    Item with worry level {} is thrown to monkey {}.", worry_level, receiver);
        inventories[receiver].push_back(worry_level);
      }
    }

    if round % 1000 == 0 {
      debug!("== Round {} Inspections ==", round);
      for (i, count) in inspections.iter().enumerate() {
        debug!("Monkey {}: {}", i, count);
      }
    }
  }

  inspections.sort();
  inspections.iter().rev().take(2).fold(1u64, |a, b| a * b)
}

fn main() {
  // RUST_LOG=trace|debug|etc
  env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
    .format_target(false)
    .format_timestamp(None)
    .init();

  let input = include_str!("../assets/input.txt");
  let (monkeys, inventories) = build_monkeys(input);

  info!("[pt1]: {}", find_monkey_biz(&mut monkeys.clone(), &mut inventories.clone(), false));
  info!("[pt2]: {}", find_monkey_biz(&mut monkeys.clone(), &mut inventories.clone(), true));
}
