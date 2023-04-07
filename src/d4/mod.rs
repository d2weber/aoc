use std::ops::RangeInclusive;

fn make_range(s: &str) -> RangeInclusive<i32> {
    let (from, end) = s.split_once('-').unwrap();
    let from = from.parse::<i32>().unwrap();
    let end = end.parse::<i32>().unwrap();
    RangeInclusive::new(from, end)
}

fn parse_ranges(line: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let (a, b) = line.split_once(',').unwrap();
    (make_range(a), make_range(b))
}

pub const SAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

pub const INPUT: &str = include_str!("input");

pub mod part1 {
    use super::*;
    fn contains(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
        a.start() <= b.start() && a.end() >= b.end()
    }

    pub fn solution(s: &str) -> usize {
        s.lines()
            .map(parse_ranges)
            .filter(|(a, b)| contains(a, b) || contains(b, a))
            .count()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 2);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 562);
    }
}

pub mod part2 {
    use super::*;
    fn intersect(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
        (a.start() <= b.end() && a.end() >= b.start())
            || (b.start() <= a.end() && b.end() >= a.start())
    }
    pub fn solution(input: &str) -> usize {
        input
            .lines()
            .map(parse_ranges)
            .filter(|(a, b)| intersect(a, b))
            .count()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 4);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 924);
    }
}
