pub const INPUT: &str = include_str!("input.txt");

pub const SAMPLE: &str = include_str!("sample.txt");

type Rgb = [usize; 3];

fn parse_rgb(s: &str) -> Rgb {
    let mut result = [0; 3];
    s.trim().split(',').for_each(|ss| {
        let (n, color) = ss.trim().split_once(' ').unwrap();
        let n = n.parse().unwrap();
        match color {
            "red" => result[0] = n,
            "green" => result[1] = n,
            "blue" => result[2] = n,
            _ => panic! {"Unexpected color `{color}`"},
        }
    });
    result
}
fn parse(s: &str) -> Vec<Rgb> {
    s.split_once(':')
        .unwrap()
        .1
        .split(';')
        .map(parse_rgb)
        .collect()
}
pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> usize {
        let bag: Rgb = [12, 13, 14];
        s.lines()
            .map(parse)
            .zip(1..)
            .filter(|(rgbs, _)| {
                rgbs.iter()
                    .all(|&rgb| rgb.into_iter().zip(bag).all(|(c, b)| c <= b))
            })
            .map(|(_, i)| i)
            .sum()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 8);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 2685);
    }
}
pub mod part2 {
    use super::*;
    use std::cmp::max;

    pub fn solution(s: &str) -> usize {
        s.lines()
            .map(parse)
            .map(|rgbs| {
                rgbs.into_iter()
                    .reduce(|acc, rgb| {
                        let mut rgb = rgb.into_iter();
                        acc.map(|a| max(a, rgb.next().unwrap()))
                    })
                    .unwrap()
                    .into_iter()
                    .product::<usize>()
            })
            .sum()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 2286);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 83707);
    }
}
