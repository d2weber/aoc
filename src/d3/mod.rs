use std::collections::HashSet;

fn priority(item: u8) -> u8 {
    match item {
        b'a'..=b'z' => item - 96,
        b'A'..=b'Z' => item - 38,
        _ => panic!("Invalid item `{item}`"),
    }
}

#[test]
fn priority_test() {
    assert_eq!(b'A', 65);
    assert_eq!(b'Z', 90);
    assert_eq!(b'a', 97);
    assert_eq!(b'z', 122);
    assert_eq!(priority(b'a'), 1);
    assert_eq!(priority(b'z'), 26);
    assert_eq!(priority(b'A'), 27);
    assert_eq!(priority(b'Z'), 52);
    assert_eq!(priority(b'p'), 16);
    assert_eq!(priority(b'L'), 38);
    assert_eq!(priority(b'P'), 42);
    assert_eq!(priority(b'v'), 22);
    assert_eq!(priority(b't'), 20);
    assert_eq!(priority(b's'), 19);
}
pub const SAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

pub const INPUT: &str = include_str!("input");

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> i32 {
        s.lines()
            .map(|line| {
                assert_eq!(line.len() % 2, 0);
                let (a, b) = line.split_at(line.len() / 2);
                let a: HashSet<u8> = a.bytes().collect();
                let b: HashSet<u8> = b.bytes().collect();
                let mut it = a.intersection(&b);
                let r = priority(*it.next().unwrap()) as i32;
                assert_eq!(it.next(), None);
                r
            })
            .sum()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 157);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 7568);
    }
}

pub mod part2 {
    use super::*;

    pub fn solution(s: &str) -> i32 {
        let mut hashsets = s.lines().map(|line| line.bytes().collect());

        let mut result = 0;
        while let Some(a) = hashsets.next() {
            let b: HashSet<u8> = hashsets.next().unwrap();
            let c = hashsets.next().unwrap();

            let inters = a.intersection(&b).cloned().collect();
            let mut inters = c.intersection(&inters);
            result += priority(*inters.next().unwrap()) as i32;
            assert_eq!(inters.next(), None);
        }
        result
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 70);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 2780);
    }
}
