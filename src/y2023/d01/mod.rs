pub const INPUT: &str = include_str!("input.txt");

pub mod part1 {
    pub const SAMPLE: &str = include_str!("sample_part1.txt");
    #[cfg(test)]
    use super::*;
    fn get_calibration_value(s: &str) -> u32 {
        let first = s.chars().find_map(|c| c.to_digit(10)).unwrap();
        let last = s.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
        first * 10 + last
    }

    pub fn solution(s: &str) -> u32 {
        s.lines().map(get_calibration_value).sum()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 142);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 54940);
    }
}
pub mod part2 {
    pub const SAMPLE: &str = include_str!("sample_part2.txt");
    const NUMBERS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    const DIGITS: [&str; 10] = ["", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    #[cfg(test)]
    use super::*;
    fn position(s: &str, num: usize, num_str: &str) -> Option<(usize, usize)> {
        match (s.find(DIGITS[num]), s.find(num_str)) {
            (Some(p1), Some(p2)) => Some((num, std::cmp::min(p1, p2))),
            (Some(p), None) | (None, Some(p)) => Some((num, p)),
            _ => None,
        }
    }
    fn rposition(s: &str, num: usize, num_str: &str) -> Option<(usize, usize)> {
        match (
            s.rfind(DIGITS[num]),
            s.rfind(&num_str).map(|p| p + num_str.len() - 1),
        ) {
            (Some(p1), Some(p2)) => Some((num, std::cmp::max(p1, p2))),
            (Some(p), None) | (None, Some(p)) => Some((num, p)),
            _ => None,
        }
    }
    fn get_calibration_value(s: &str) -> usize {
        let first = (1..)
            .zip(NUMBERS)
            .filter_map(|(num, num_str)| position(s, num, num_str))
            .min_by_key(|(_, p)| *p)
            .unwrap()
            .0;
        let last = (1..)
            .zip(NUMBERS)
            .filter_map(|(num, num_str)| rposition(s, num, num_str))
            .max_by_key(|(_, p)| *p)
            .unwrap()
            .0;
        first * 10 + last
    }

    pub fn solution(s: &str) -> usize {
        s.lines().map(get_calibration_value).sum()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 281);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 54208);
    }
}
