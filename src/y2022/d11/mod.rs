use std::collections::VecDeque;
use std::str::FromStr;

pub const SAMPLE: &str = include_str!("sample");

pub const INPUT: &str = include_str!("input");

#[derive(Debug)]
enum Operation {
    Mul(u64),
    Add(u64),
    Square(),
}

impl Operation {
    fn perform(&self, v: u64) -> u64 {
        match self {
            Operation::Mul(o) => o * v,
            Operation::Add(o) => o + v,
            Operation::Square() => v * v,
        }
    }
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut it = line.split_whitespace().skip(4);
        Ok(match it.next().unwrap() {
            "*" => match it.next().unwrap() {
                "old" => Operation::Square(),
                o => Operation::Mul(o.parse().unwrap()),
            },
            "+" => Operation::Add(it.next().unwrap().parse().unwrap()),
            op => panic!("Couldn't parse operation {op}"),
        })
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    op: Operation,
    divisor: u64,
    destinations: (usize, usize),
    n_inspected: usize,
}

impl Monkey {
    fn destination_for(&self, item: u64) -> usize {
        if (item % self.divisor) == 0 {
            self.destinations.0
        } else {
            self.destinations.1
        }
    }
}

fn next_skip_parse<'a, F: FromStr>(it: &mut impl Iterator<Item = &'a str>, n: usize) -> F
where
    <F as FromStr>::Err: std::fmt::Debug,
{
    it.next()
        .unwrap()
        .trim()
        .splitn(n + 1, ' ')
        .nth(n)
        .unwrap()
        .parse::<F>()
        .unwrap()
}

fn parse_monkeys(s: &str) -> Vec<Monkey> {
    s.split("\n\n")
        .map(|group| {
            let mut it = group.lines().skip(1);
            Monkey {
                items: it
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .skip(2)
                    .map(|n| n.trim_end_matches(',').parse().unwrap())
                    .collect(),
                op: next_skip_parse(&mut it, 0),
                divisor: next_skip_parse(&mut it, 3),
                destinations: (next_skip_parse(&mut it, 5), next_skip_parse(&mut it, 5)),
                n_inspected: 0,
            }
        })
        .collect()
}

fn monkey_buisness(mut monkeys: Vec<Monkey>) -> usize {
    monkeys.sort_unstable_by_key(|m| m.n_inspected);
    monkeys
        .into_iter()
        .rev()
        .map(|m| m.n_inspected)
        .take(2)
        .product()
}

const CAN_BE_SQUARED: u64 = 1u64 << 32; // squaring this is safe

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> usize {
        let mut monkeys = parse_monkeys(s);
        (0..20).for_each(|_| {
            (0..monkeys.len()).for_each(|i| {
                monkeys[i].n_inspected += monkeys[i].items.len();
                while let Some(mut item) = monkeys[i].items.pop_front() {
                    let m = &mut monkeys[i];
                    item = m.op.perform(item) / 3;
                    let dst = m.destination_for(item);
                    monkeys[dst].items.push_back(item);
                }
            });
        });
        monkey_buisness(monkeys)
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 10605);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 50830);
    }
}

pub mod part2 {
    use super::*;

    pub fn solution(s: &str) -> usize {
        let mut monkeys = parse_monkeys(s);
        let product = monkeys
            .iter()
            .map(|m| m.divisor)
            .reduce(std::ops::Mul::mul)
            .unwrap();
        (0..10000).for_each(|_| {
            (0..monkeys.len()).for_each(|i| {
                monkeys[i].n_inspected += monkeys[i].items.len();
                while let Some(mut item) = monkeys[i].items.pop_front() {
                    let m = &mut monkeys[i];
                    if item > CAN_BE_SQUARED {
                        item %= product;
                    }
                    item = m.op.perform(item);
                    let dst = m.destination_for(item);
                    monkeys[dst].items.push_back(item);
                }
            });
        });
        monkey_buisness(monkeys)
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 2713310158);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 14399640002);
    }
}
