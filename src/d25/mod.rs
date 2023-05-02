pub const SAMPLE: &str = include_str!("sample");
pub const INPUT: &str = include_str!("input");

#[derive(Debug, PartialEq)]
enum Digit {
    NegTwo = -2,
    NegOne = -1,
    Zero = 0,
    One = 1,
    Two = 2,
}

fn digits_from_decimal(mut value: i64) -> impl Iterator<Item = Digit> {
    let mut max_representable = 0;
    let mut factor = 1;
    let n_digits = (1..)
        .find(|_| {
            max_representable += 2 * factor;
            let res = value.abs() <= max_representable;
            factor *= 5;
            res
        })
        .unwrap();
    factor /= 5;
    (0..n_digits).map(move |_| {
        max_representable -= 2 * factor;
        let v = if value.abs() <= max_representable {
            Digit::Zero
        } else if (value - 2 * factor).abs() <= max_representable {
            value -= 2 * factor;
            Digit::Two
        } else if (value - factor).abs() <= max_representable {
            value -= factor;
            Digit::One
        } else if (value + factor).abs() <= max_representable {
            value += factor;
            Digit::NegOne
        } else if (value + 2 * factor).abs() <= max_representable {
            value += 2 * factor;
            Digit::NegTwo
        } else {
            panic!()
        };
        factor /= 5;
        v
    })
}

#[test]
fn test_from_decimal() {
    use itertools::assert_equal;
    for (n, ds) in [
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0"),
    ] {
        assert_equal(digits_from_decimal(n), ds.chars().map(Digit::from))
    }
}

impl From<char> for Digit {
    fn from(value: char) -> Digit {
        match value {
            '=' => Digit::NegTwo,
            '-' => Digit::NegOne,
            '0' => Digit::Zero,
            '1' => Digit::One,
            '2' => Digit::Two,
            _ => panic!(),
        }
    }
}

impl From<Digit> for char {
    fn from(value: Digit) -> Self {
        match value {
            Digit::NegTwo => '=',
            Digit::NegOne => '-',
            Digit::Zero => '0',
            Digit::One => '1',
            Digit::Two => '2',
        }
    }
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> String {
        let n: i64 = s
            .lines()
            .flat_map(|l| {
                let mut factor = 1;
                l.chars().rev().map(move |d| {
                    let d = Digit::from(d) as i64 * factor;
                    factor *= 5;
                    d
                })
            })
            .sum();
        digits_from_decimal(n).map(char::from).collect::<String>()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), "2=-1=0");
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), "2=--=0000-1-0-=1=0=2");
    }
}
