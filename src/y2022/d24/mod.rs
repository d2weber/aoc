use std::collections::HashSet;
use std::iter::once;
use std::ops;

pub const SAMPLE: &str = include_str!("sample");
pub const INPUT: &str = include_str!("input");

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct P {
    x: i32,
    y: i32,
}

impl_op_ex!(+ |l: &P, r: &P| -> P {P {x: l.x + r.x, y: l.y + r.y}});

impl P {
    fn within_bbox(&self, other: &P) -> bool {
        other.x >= 0 && other.y >= 0 && other.x < self.x && other.y < self.y
    }

    fn tick(&mut self, t_max: &P) {
        self.x = (self.x + 1) % t_max.x;
        self.y = (self.y + 1) % t_max.y;
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct State {
    /// current position
    p: P,
    /// time wrapped separately for x and y
    t: P,
}

fn wrap(v: i32, wrap: i32) -> i32 {
    if v < 0 {
        v + wrap
    } else {
        v
    }
}

type Blizzards = [HashSet<P>; 4];

#[derive(Debug)]
struct Map {
    blizzards: Blizzards,
    bounds: P,
}

const ZERO: P = P { x: 0, y: 0 };
const N: P = P { x: 0, y: -1 };
const E: P = P { x: 1, y: 0 };
const S: P = P { x: 0, y: 1 };
const W: P = P { x: -1, y: 0 };

impl Map {
    /// We do not include the real starting point in the simulation.
    /// We start simulationg at one below the real starting point and end
    /// one above the real end point. The `p_init` is inserted in each timestep,
    /// to take into account, that it's allowed to wait on the real staring point.
    fn count_ticks(&self, State { p: p_init, mut t }: State, target: P) -> Result<usize, &str> {
        let mut states = HashSet::<State>::new();
        let mut states_hist = HashSet::new();
        for round in 1..1000 {
            t.tick(&self.bounds);
            states_hist.extend(states.clone());
            states = states
                .into_iter()
                .flat_map(|s| self.next(State { p: s.p, t }))
                .chain(once(State { p: p_init, t }))
                .filter(|s| !states_hist.contains(s))
                .collect();

            if states.iter().any(|s| s.p == target) {
                return Ok(round + 1); // +1 to make another step down to _real_ end point
            }
            if states.is_empty() {
                return Err("No states left");
            }
        }
        Err("Exceeded max iterations")
    }

    fn next(&self, state: State) -> impl Iterator<Item = State> + '_ {
        [ZERO, N, E, S, W].into_iter().filter_map(move |offs| {
            let next_state = State {
                p: state.p + offs,
                t: state.t,
            };
            if self.bounds.within_bbox(&next_state.p)
                && self
                    .hor_vert_fwd_bwd(&next_state)
                    .into_iter()
                    .zip(self.blizzards.iter())
                    .all(|(p, b)| !b.contains(&p))
            {
                Some(next_state)
            } else {
                None
            }
        })
    }

    fn hor_vert_fwd_bwd(&self, State { p, t }: &State) -> [P; 4] {
        [
            P {
                x: wrap(p.x - t.x, self.bounds.x),
                y: p.y,
            },
            P {
                x: (p.x + t.x) % self.bounds.x,
                y: p.y,
            },
            P {
                x: p.x,
                y: wrap(p.y - t.y, self.bounds.y),
            },
            P {
                x: p.x,
                y: (p.y + t.y) % self.bounds.y,
            },
        ]
    }
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> usize {
        let map = parse(s);
        map.count_ticks(State { p: ZERO, t: ZERO }, map.bounds + P { x: -1, y: -1 })
            .unwrap()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 18);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 299);
    }
}

pub mod part2 {
    use super::*;

    pub fn solution(s: &str) -> usize {
        let map = parse(s);
        let end_point = map.bounds + P { x: -1, y: -1 };
        let mut n = map
            .count_ticks(State { p: ZERO, t: ZERO }, end_point)
            .unwrap();
        n += map
            .count_ticks(
                State {
                    p: end_point,
                    t: P {
                        x: n as i32 % map.bounds.x,
                        y: n as i32 % map.bounds.y,
                    },
                },
                ZERO,
            )
            .unwrap();
        n += map
            .count_ticks(
                State {
                    p: ZERO,
                    t: P {
                        x: n as i32 % map.bounds.x,
                        y: n as i32 % map.bounds.y,
                    },
                },
                end_point,
            )
            .unwrap();
        n
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 54);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 899);
    }
}

fn parse(s: &str) -> Map {
    let mut blizzards: Blizzards = std::array::from_fn(|_| HashSet::new());
    let mut bounds = P { x: 0, y: 0 };
    let mut lines = s.lines().skip(1).enumerate().peekable();
    while let Some((y, l)) = lines.next() {
        if lines.peek().is_none() {
            break;
        }
        let y = y as i32;
        bounds.y = bounds.y.max(y + 1);
        bounds.x = bounds.x.max(l.len() as i32 - 2);
        l.bytes()
            .filter(|b| *b != b'#')
            .enumerate()
            .for_each(|(x, b)| {
                let x = x as i32;
                match b {
                    b'>' => {
                        blizzards[0].insert(P { x, y });
                    }
                    b'<' => {
                        blizzards[1].insert(P { x, y });
                    }
                    b'v' => {
                        blizzards[2].insert(P { x, y });
                    }
                    b'^' => {
                        blizzards[3].insert(P { x, y });
                    }
                    b'.' => (),
                    _ => panic!(),
                }
            });
    }
    Map { blizzards, bounds }
}
