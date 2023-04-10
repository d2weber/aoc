pub const SAMPLE: &str = include_str!("sample");

pub const INPUT: &str = include_str!("input");

#[derive(Clone, PartialEq, Debug)]
enum Packet {
    List(Vec<Packet>),
    Int(i32),
}
use Packet::*;

fn find_next_comma(s: &str) -> Option<usize> {
    let mut depth = 0;
    s.bytes().position(move |c| {
        if c == b'[' {
            depth += 1;
        } else if c == b']' {
            depth -= 1;
        }

        c == b',' && depth == 0
    })
}

fn parse(s: &str) -> Packet {
    if let Some(mut s) = s.strip_prefix('[') {
        let mut v = Vec::new();
        while s != "]" {
            let delimiter = find_next_comma(s).unwrap_or(s.len() - 1);
            v.push(parse(&s[..delimiter]));
            s = s[delimiter..].trim_start_matches(',');
        }
        Packet::List(v)
    } else {
        Int(s.parse().unwrap())
    }
}

fn are_in_order((left, right): (Packet, Packet)) -> Option<bool> {
    match (left, right) {
        (List(left), List(right)) => {
            let mut right_it = right.into_iter();
            left.into_iter()
                .find_map(|left| {
                    if let Some(right) = right_it.next() {
                        are_in_order((left, right))
                    } else {
                        Some(false) // right went out of elements
                    }
                })
                .or(
                    Some(true).filter(|_| right_it.next().is_some()), // left went out of elements
                )
        }
        (Int(left), Int(right)) if left == right => None,
        (Int(left), Int(right)) => Some(left < right),
        (List(l), Int(i)) => are_in_order((List(l), List(vec![Int(i)]))),
        (Int(i), List(l)) => are_in_order((List(vec![Int(i)]), List(l))),
    }
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> usize {
        s.split("\n\n")
            .zip(1..)
            .filter_map(|(ll, i)| {
                let (left, right) = ll.trim().split_once('\n').unwrap();
                let (left, right) = (parse(left), parse(right));
                Some(i).filter(|_| are_in_order((left, right)).unwrap())
            })
            .sum()
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 13);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 5503);
    }
}
