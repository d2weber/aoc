pub const SAMPLE: &str = include_str!("sample");
pub const INPUT: &str = include_str!("input");

pub mod part1 {
    #[cfg(test)]
    use super::*;

    pub fn solution(s: &str) -> i32 {
        let n_bits = s.find('\n').unwrap();
        let counts = s.lines().fold(vec![0; n_bits], |mut counts, l| {
            counts.iter_mut().zip(l.bytes()).for_each(|(c, b)| match b {
                b'0' => (),
                b'1' => *c += 1,
                b => panic!("invalid {b}"),
            });
            counts
        });
        let thresh = s.lines().count() / 2;
        let (gamma, epsilon) = counts.iter().fold((0, 0), |(mut gamma, mut epsilon), c| {
            gamma <<= 1;
            epsilon <<= 1;
            if *c > thresh {
                (gamma + 1, epsilon)
            } else {
                (gamma, epsilon + 1)
            }
        });
        gamma * epsilon
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 198);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 2724524);
    }
}
