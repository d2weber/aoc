pub const SAMPLE: &str = include_str!("sample");

pub const INPUT: &str = include_str!("input");

pub mod part1 {
    #[cfg(test)]
    use super::*;

    pub fn solution(s: &str) -> usize {
        let mut it = s.lines().map(|l| l.parse().unwrap());
        let mut last: i32 = it.next().unwrap();
        it.filter(move |d| {
            let result = d > &last;
            last = *d;
            result
        })
        .count()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 7);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 1709);
    }
}
pub mod part2 {
    #[cfg(test)]
    use super::*;

    pub fn solution(s: &str) -> usize {
        let v:Vec<_> = s.lines().map(|l| l.parse().unwrap()).collect();
        let mut it= v.windows(3).map(|w| w.iter().sum());
        let mut last: i32 = it.next().unwrap();
        it.filter(move |d| {
            let result = d > &last;
            last = *d;
            result
        })
        .count()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 5);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 1761);
    }
}
