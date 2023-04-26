pub const SAMPLE: &str = include_str!("sample");
pub const INPUT: &str = include_str!("input");

fn parse(s: &str) -> impl Iterator<Item = i64> + '_ {
    s.lines().map(|l| l.parse().unwrap())
}

fn mix(shifts: &Vec<i64>, count: usize) -> i64 {
    let mut indices = Vec::from_iter(0..shifts.len());
    let wrap = |mut i: i64| -> usize {
        i &= shifts.len() as i64 - 1;
        if i < 0 {
            i += shifts.len() as i64 - 1;
        }
        i as usize
    };
    for _ in 0..count {
        shifts.iter().enumerate().for_each(|(orig_i, &shift)| {
            let curr_i = indices.iter().position(|&i| i == orig_i).unwrap();
            indices.remove(curr_i);
            indices.insert(wrap(curr_i as i64 + shift), orig_i);
        });
    }
    indices
        .into_iter()
        .map(|i| shifts[i])
        .cycle()
        .skip_while(|v| *v != 0)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum()
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> i64 {
        mix(&parse(s).collect(), 1)
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
        let shifts = parse(s).map(|s| s * 811589153).collect();
        mix(&shifts, 10)
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
