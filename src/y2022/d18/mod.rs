use itertools::Itertools;
use std::collections::HashSet;

pub const SAMPLE: &str = include_str!("sample");
pub const INPUT: &str = include_str!("input");

const N_DIMS: usize = 3;
type Point = [i32; N_DIMS];

fn parse(s: &str) -> HashSet<Point> {
    s.lines()
        .map(|l| {
            let (x, y, z) = l
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap();
            [x, y, z]
        })
        .collect()
}

fn neighbors(p: &Point) -> impl Iterator<Item = Point> + '_ {
    p.iter().enumerate().flat_map(move |(i, v)| {
        let mut p = *p;
        [1, -1].into_iter().map(move |offs| {
            p[i] = v + offs;
            p
        })
    })
}

fn surface(ps: HashSet<Point>) -> usize {
    ps.iter()
        .flat_map(|p| neighbors(p).filter(|p| !ps.contains(p)))
        .count()
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> usize {
        let ps = parse(s);
        surface(ps)
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 64);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 4548);
    }
}

pub mod part2 {
    use super::*;

    pub fn solution(s: &str) -> usize {
        let lava = parse(s);

        let minmax: [(i32, i32); N_DIMS] = (0..N_DIMS)
            .map(|d| lava.iter().map(|p| p[d]).minmax().into_option().unwrap())
            .collect_vec()
            .try_into()
            .unwrap();

        let mut unkown: HashSet<Point> = minmax
            .iter()
            .map(|(min, max)| *min..=*max)
            .multi_cartesian_product()
            .map(|v| v.try_into().unwrap())
            .filter(|p| !lava.contains(p))
            .collect();

        // Remove all the _air_ form `unknown` by expanding starting from the surface
        while let Some(p) = unkown.iter().find(|p| {
            p.iter()
                .zip(minmax)
                .any(|(p, (min, max))| *p == min || *p == max)
        }) {
            let mut que = vec![*p];
            while let Some(p) = que.pop() {
                if let Some(p) = unkown.take(&p) {
                    que.extend(neighbors(&p));
                }
            }
        }
        surface(lava) - surface(unkown)
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 58);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 2588);
    }
}
