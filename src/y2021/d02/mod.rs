pub const SAMPLE: &str = include_str!("sample");
pub const INPUT: &str = include_str!("input");

pub mod part1 {
    #[cfg(test)]
    use super::*;

    pub fn solution(s: &str) -> i32 {
        let mut pos = 0;
        let mut depth = 0;
        s.lines().for_each(|l| {
            if let Some(v) = l.strip_prefix("up ") {
                depth -= v.parse::<i32>().unwrap();
            } else if let Some(v) = l.strip_prefix("down ") {
                depth += v.parse::<i32>().unwrap();
            } else if let Some(v) = l.strip_prefix("forward ") {
                pos += v.parse::<i32>().unwrap();
            }
        });
        pos * depth
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 150);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 1427868);
    }
}
