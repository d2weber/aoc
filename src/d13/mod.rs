pub const SAMPLE: &str = include_str!("sample");

pub const INPUT: &str = include_str!("input");

#[derive(Clone, Eq, PartialEq, Debug)]
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

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        match (self, other) {
            (List(slf), List(othr)) => {
                let mut other_it = othr.iter();
                for s in slf {
                    let Some(o) = other_it.next() else {
                        return Greater; // other went out of elements
                    };

                    match s.cmp(o) {
                        ord @ Less | ord @ Greater => {
                            return ord;
                        }
                        Equal => (),
                    }
                }
                if other_it.next().is_some() {
                    return Less; // self went out of elements
                }
                Equal
            }
            (Int(slf), Int(othr)) if slf == othr => Equal,
            (Int(slf), Int(othr)) => slf.cmp(othr),
            (List(_), Int(i)) => self.cmp(&List(vec![Int(*i)])),
            (Int(i), List(_)) => List(vec![Int(*i)]).cmp(other),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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
                Some(i).filter(|_| left < right)
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

pub mod part2 {
    use super::*;

    pub fn solution(s: &str) -> usize {
        let mut packets: Vec<Packet> = s.lines().filter(|l| !l.is_empty()).map(parse).collect();
        packets.sort_unstable();
        let distress_signals = [
            // sorted
            List(vec![List(vec![Int(2)])]),
            List(vec![List(vec![Int(6)])]),
        ];
        distress_signals
            .into_iter()
            .map(|p| {
                let pos = packets.binary_search(&p).unwrap_err();
                packets.insert(pos, p);
                pos + 1
            })
            .product()
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 140);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 20952);
    }
}
