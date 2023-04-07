use either::Either::{Left, Right};
use std::iter::once;
use std::mem;

pub const SAMPLE: &str = include_str!("sample");

pub const INPUT: &str = include_str!("input");

fn register_states(s: &str) -> impl Iterator<Item = i32> + '_ {
    let mut buff = 0;
    s.lines()
        .flat_map(|line| {
            if let Some(v) = line.strip_prefix("addx ") {
                Left(once(0).chain(once(v.parse::<i32>().unwrap())))
            } else {
                Right(once(0))
            }
        })
        .scan(1, move |x, mut v| {
            mem::swap(&mut v, &mut buff); // Buffer to delay v
            *x += v;
            Some(*x)
        })
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> i32 {
        register_states(s)
            .zip(1..)
            .skip(19)
            .step_by(40)
            .map(|(x, i)| x * i)
            .sum()
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 13140);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 15680);
    }
}

pub mod part2 {
    use super::*;

    const WIDTH: i32 = 40;

    pub fn solution(s: &str) -> String {
        register_states(s)
            .zip((0..WIDTH).cycle())
            .flat_map(|(x, crt_pos)| {
                let pixel = if (crt_pos - x).abs() < 2 { "#" } else { "." };
                let sep = if crt_pos == WIDTH - 1 { "\n" } else { "" };
                once(pixel).chain(once(sep))
            })
            .collect::<String>()
    }
    #[test]
    fn sample() {
        assert_eq!(
            solution(SAMPLE),
            concat!(
                "##..##..##..##..##..##..##..##..##..##..\n",
                "###...###...###...###...###...###...###.\n",
                "####....####....####....####....####....\n",
                "#####.....#####.....#####.....#####.....\n",
                "######......######......######......####\n",
                "#######.......#######.......#######.....\n",
            )
        );
    }
    #[test]
    fn actual() {
        assert_eq!(
            solution(INPUT),
            concat!(
                "####.####.###..####.#..#..##..#..#.###..\n",
                "...#.#....#..#.#....#..#.#..#.#..#.#..#.\n",
                "..#..###..###..###..####.#....#..#.#..#.\n",
                ".#...#....#..#.#....#..#.#.##.#..#.###..\n",
                "#....#....#..#.#....#..#.#..#.#..#.#....\n",
                "####.#....###..#....#..#..###..##..#....\n",
            )
        );
    }
}
