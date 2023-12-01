use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops;

pub const SAMPLE: &str = include_str!("sample");
pub const INPUT: &str = include_str!("input");

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct P {
    x: i32,
    y: i32,
}

const fn add(lhs: &P, rhs: &P) -> P {
    P {
        x: lhs.x + rhs.x,
        y: lhs.y + rhs.y,
    }
}
impl_op_ex!(+ |a: &P, b: &P| -> P { add(a, b) });

fn move_elves(elves: &mut HashSet<P>, orientations: &mut [[P; 3]; 4]) -> Result<(), ()> {
    let (candidates, mut frozen): (Vec<P>, Vec<P>) = elves
        .iter()
        .cloned()
        .partition(|p| SURROUND.iter().any(|rel_p| elves.contains(&(p + rel_p))));
    let mut moved = HashMap::<P, P>::new();
    candidates.into_iter().for_each(|candidate| {
        if let Some(next_p) = orientations.iter().find_map(|ps| {
            if ps
                .iter()
                .all(|rel_p| !elves.contains(&(&candidate + rel_p)))
            {
                Some(&candidate + &ps[0])
            } else {
                None
            }
        }) {
            if let Some(conflicting) = moved.remove(&next_p) {
                frozen.push(conflicting);
                frozen.push(candidate);
            } else {
                moved.insert(next_p, candidate);
            }
        } else {
            frozen.push(candidate);
        }
    });
    let nothing_moved = moved.is_empty();
    *elves = HashSet::from_iter(moved.into_keys().chain(frozen));
    orientations.rotate_left(1);
    if nothing_moved {
        Err(())
    } else {
        Ok(())
    }
}

fn parse(s: &str) -> HashSet<P> {
    s.lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes().enumerate().filter_map(move |(x, b)| match b {
                b'#' => Some(P {
                    x: x as i32,
                    y: y as i32,
                }),
                b'.' => None,
                _ => panic!(),
            })
        })
        .collect()
}

const N: P = P { x: 0, y: -1 };
const S: P = P { x: 0, y: 1 };
const W: P = P { x: -1, y: 0 };
const E: P = P { x: 1, y: 0 };

const SURROUND: [P; 8] = [
    N,
    add(&N, &E),
    E,
    add(&S, &E),
    S,
    add(&S, &W),
    W,
    add(&W, &N),
];

type Orientation = [P; 3];
const INIT_ORIENTATIONS: [Orientation; 4] = [
    [N, add(&N, &E), add(&N, &W)],
    [S, add(&S, &E), add(&S, &W)],
    [W, add(&N, &W), add(&S, &W)],
    [E, add(&N, &E), add(&S, &E)],
];

fn _print_map(ps: &HashSet<P>) -> impl Iterator<Item = u8> + '_ {
    let (min_x, max_x) = ps.iter().map(|p| p.x).minmax().into_option().unwrap();
    let (min_y, max_y) = ps.iter().map(|p| p.y).minmax().into_option().unwrap();
    (min_y..=max_y).flat_map(move |y| {
        (min_x..=max_x)
            .map(move |x| if ps.contains(&P { x, y }) { b'#' } else { b'.' })
            .chain(std::iter::once(b'\n'))
    })
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> i32 {
        let mut elves = parse(s);

        let mut orientations = INIT_ORIENTATIONS;

        for _ in 0..10 {
            move_elves(&mut elves, &mut orientations).ok();
        }
        let (min_x, max_x) = elves.iter().map(|p| p.x).minmax().into_option().unwrap();
        let (min_y, max_y) = elves.iter().map(|p| p.y).minmax().into_option().unwrap();
        (max_y - min_y + 1) * (max_x - min_x + 1) - elves.len() as i32
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 110);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 4116);
    }
}
pub mod part2 {
    use super::*;

    pub fn solution(s: &str) -> i32 {
        let mut elves = parse(s);
        let mut orientations = INIT_ORIENTATIONS;

        (1..)
            .find(|_| move_elves(&mut elves, &mut orientations).is_err())
            .unwrap()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 20);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 984);
    }
}
