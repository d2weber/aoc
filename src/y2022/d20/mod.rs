use std::cmp::Ordering;

use itertools::Itertools;

pub const SAMPLE: &str = include_str!("sample");
pub const INPUT: &str = include_str!("input");

fn parse(s: &str) -> Vec<i64> {
    s.lines().map(|l| l.parse().unwrap()).collect()
}

fn coordinates(indices: Vec<usize>, shifts: Vec<i64>) -> i64 {
    shifts
        .into_iter()
        .enumerate()
        .sorted_unstable_by_key(|(i, _)| indices[*i])
        .map(|(_, v)| v)
        .cycle()
        .skip_while(|v| *v != 0)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum()
}

fn mix(shifts: &[i64], count: usize) -> Vec<usize> {
    let mut indices = Vec::from_iter(0..shifts.len());
    for _ in 0..count {
        shifts.iter().enumerate().for_each(|(orig_i, &shift)| {
            let curr_i = indices[orig_i];
            let targ_i = wrap_idx(curr_i as i64 + shift, shifts.len());
            match curr_i.cmp(&targ_i) {
                Ordering::Less => indices.iter_mut().for_each(|i| {
                    if *i > curr_i && *i <= targ_i {
                        *i -= 1;
                    }
                }),
                Ordering::Greater => indices.iter_mut().for_each(|i| {
                    if *i >= targ_i && *i < curr_i {
                        *i += 1;
                    }
                }),
                Ordering::Equal => (),
            }
            indices[orig_i] = targ_i;
        });
    }
    indices
}

fn wrap_idx(mut i: i64, len: usize) -> usize {
    let wrap_len = len as i64 - 1;
    i %= wrap_len;
    if i < 0 {
        i += wrap_len;
    }
    i as usize
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> i64 {
        let shifts = parse(s);
        coordinates(mix(&shifts, 1), shifts)
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 3);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 7228);
    }
}

pub mod part2 {
    use super::*;

    pub fn solution(s: &str) -> i64 {
        let mut shifts = parse(s);
        let key = 811589153;
        shifts.iter_mut().for_each(|s| *s *= key);
        let indices = mix(&shifts, 10);
        coordinates(indices, shifts)
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 1623178306);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 4526232706281);
    }
}
