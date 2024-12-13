//! Todo: implement proper algorithm to find adjacent edges
//! (current implementation is buggy and only works, because the ordering was tuned manually)

use either::Either;
use std::collections::HashMap;

pub const SAMPLE: &str = include_str!("sample");
pub const INPUT: &str = include_str!("input");

const fn div_floor(lhs: i32, rhs: i32) -> i32 {
    let d = lhs / rhs;
    let r = lhs % rhs;
    if (r > 0 && rhs < 0) || (r < 0 && rhs > 0) {
        d - 1
    } else {
        d
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct P {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Field {
    Wall,
    Free,
}

#[derive(Clone, Debug, PartialEq)]
#[repr(i8)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl From<i8> for Dir {
    fn from(value: i8) -> Self {
        match value {
            x if x == Dir::Right as i8 => Dir::Right,
            x if x == Dir::Down as i8 => Dir::Down,
            x if x == Dir::Left as i8 => Dir::Left,
            x if x == Dir::Up as i8 => Dir::Up,
            _ => panic!(),
        }
    }
}

impl Dir {
    fn shift(self, shift: Dir) -> Self {
        ((self as i8 + shift as i8) % 4).into()
    }
}

impl std::ops::Add for P {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        P {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Mul<i32> for P {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        P {
            x: rhs * self.x,
            y: rhs * self.y,
        }
    }
}

impl P {
    /// `dir` is interpreted as amount of rotation.
    /// The virtual center of the rotation is at `x: off as f32/2, y: off as f32/2`
    fn tfm_rotate(self, dir: &Dir, off: i32) -> Self {
        match dir {
            Dir::Right => self, // No Rotation
            Dir::Down => P {
                // 90 degree clockwise
                x: off - self.y,
                y: self.x,
            },
            Dir::Left => P {
                // 180 degree
                x: off - self.x,
                y: off - self.y,
            },
            Dir::Up => P {
                // 270 degree clockwise
                x: self.y,
                y: off - self.x,
            },
        }
    }
    fn step(self, dir: &Dir) -> Self {
        let P { x, y } = self;
        match dir {
            Dir::Right => P { x: x + 1, y },
            Dir::Down => P { x, y: y + 1 },
            Dir::Left => P { x: x - 1, y },
            Dir::Up => P { x, y: y - 1 },
        }
    }
}

#[derive(Debug)]
struct State {
    p: P,
    dir: Dir,
}

#[derive(Debug)]
struct Map<const TILE_SIZE: usize> {
    ps: HashMap<P, Field>,
    x_max: i32,
    y_max: i32,
}

#[derive(Debug)]
enum LR {
    L,
    R,
}

#[derive(Debug)]
enum Instruction {
    Turn(LR),
    Go(usize),
}
use Instruction::*;

fn parse<const TILE_SIZE: usize>(
    s: &str,
) -> (Map<TILE_SIZE>, impl Iterator<Item = Instruction> + '_) {
    let (s1, s2) = s.split_once("\n\n").unwrap();
    let mut x_max = 0;
    let mut y_max = 0;
    let ps = s1
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            x_max = x_max.max(l.len() as i32);
            y_max = y_max.max(y as i32);
            l.bytes()
                .enumerate()
                .filter(|(_i, b)| !b.is_ascii_whitespace())
                .map(move |(x, b)| {
                    (
                        P {
                            x: x as i32,
                            y: y as i32,
                        },
                        match b {
                            b'#' => Field::Wall,
                            b'.' => Field::Free,
                            _ => panic!(),
                        },
                    )
                })
        })
        .collect();
    use std::iter::once;
    let instr = s2.trim().split_inclusive(&['R', 'L']).flat_map(|inst| {
        let (go, lr) = if let Some(go) = inst.strip_suffix('L') {
            (go, Some(LR::L))
        } else if let Some(go) = inst.strip_suffix('R') {
            (go, Some(LR::R))
        } else {
            (inst, None)
        };
        once(Go(go.parse().unwrap())).chain(lr.map(Turn))
    });
    (Map { ps, x_max, y_max }, instr)
}

fn calc_password(state: State) -> i32 {
    1000 * (state.p.y + 1) + 4 * (state.p.x + 1) + state.dir as i32
}

impl<const TILE_SIZE: usize> Map<TILE_SIZE> {
    fn first_field(&self, state: State) -> (P, &Field) {
        self.wrapped_points(state)
            .find_map(|p| self.ps.get(&p).map(|f| (p, f)))
            .unwrap()
    }

    fn first_p(&self, state: State) -> P {
        self.wrapped_points(state)
            .find(|p| self.ps.contains_key(p))
            .unwrap()
    }

    fn wrapped_points(&self, State { p: P { x, y }, dir }: State) -> impl Iterator<Item = P> {
        use Either as E;
        match dir {
            Dir::Right => E::Left(E::Left((0..).map(move |x| P { x, y }))),
            Dir::Down => E::Left(E::Right((0..).map(move |y| P { x, y }))),
            Dir::Left => E::Right(E::Left((0..self.x_max).rev().map(move |x| P { x, y }))),
            Dir::Up => E::Right(E::Right((0..self.y_max).rev().map(move |y| P { x, y }))),
        }
    }

    fn wrap_step(&self, State { mut p, dir }: State) -> (P, &Field) {
        p = p.step(&dir);
        self.ps
            .get(&p)
            .map(|f| (p.clone(), f))
            .unwrap_or_else(|| self.first_field(State { p, dir }))
    }

    fn perform_part1(&self, State { mut p, dir }: State, instr: Instruction) -> State {
        match instr {
            Turn(lr) => State {
                p,
                dir: match lr {
                    LR::L => dir.shift(Dir::Up),
                    LR::R => dir.shift(Dir::Down),
                },
            },
            Go(n) => {
                for _ in 0..n {
                    match self.wrap_step(State {
                        p: p.clone(),
                        dir: dir.clone(),
                    }) {
                        (_, Field::Wall) => break,
                        (new_p, Field::Free) => p = new_p,
                    }
                }
                State { p, dir }
            }
        }
    }

    fn wrap_cube(&self, State { mut p, dir }: State) -> (State, &Field) {
        p = p.step(&dir);
        self.ps
            .get(&p)
            .map(|f| {
                (
                    State {
                        p: p.clone(),
                        dir: dir.clone(),
                    },
                    f,
                )
            })
            .unwrap_or_else(|| {
                let tile_size = TILE_SIZE as i32;
                let next_p_tile = P {
                    x: (p.x + tile_size) % tile_size,
                    y: (p.y + tile_size) % tile_size,
                };
                let tile_offset = P {
                    x: div_floor(p.x, tile_size) * tile_size,
                    y: div_floor(p.y, tile_size) * tile_size,
                };
                [
                    // const FACES: [(P, Dir); 23] = [
                    // len == 3
                    (P { x: 0, y: -1 }, Dir::Up),
                    (P { x: 0, y: 1 }, Dir::Down),
                    // len == 6
                    // (P { x: -2, y: -4 }, Dir::Right),
                    // (P { x: -2, y: 4 }, Dir::Right),
                    // (P { x: -3, y: -3 }, Dir::Right),
                    // (P { x: -3, y: 3 }, Dir::Right),
                    (P { x: -4, y: -2 }, Dir::Right),
                    // (P { x: -4, y: 2 }, Dir::Right),
                    // len == 5
                    // (P { x: 0, y: -3 }, Dir::Down),
                    // (P { x: 0, y: 3 }, Dir::Up),
                    // (P { x: -3, y: -2 }, Dir::Down),
                    // (P { x: -3, y: 2 }, Dir::Up),
                    (P { x: -4, y: -1 }, Dir::Down),
                    // (P { x: -4, y: 1 }, Dir::Up),
                    // (P { x: -2, y: -3 }, Dir::Down),
                    (P { x: -2, y: 3 }, Dir::Up),
                    // (P { x: -4, y: -1 }, Dir::Up), // Ambiguous
                    // (P { x: -4, y: 1 }, Dir::Down), // Ambiguous
                    // len == 4
                    // (P { x: -4, y: 0 }, Dir::Right), // Symmetric
                    // (P { x: -2, y: -2 }, Dir::Left),
                    (P { x: -2, y: 2 }, Dir::Left),
                    (P { x: 0, y: -2 }, Dir::Left),
                    // (P { x: 0, y: 2 }, Dir::Left),
                ]
                .into_iter()
                .find_map(|(f, next_dir)| {
                    let next_p = tile_offset.clone()
                        + next_p_tile.clone().tfm_rotate(&next_dir, tile_size - 1)
                        + f.tfm_rotate(&dir, 0) * tile_size;
                    self.ps.get(&next_p).map(|field| {
                        let next_dir = next_dir.shift(dir.clone());
                        (
                            State {
                                p: next_p,
                                dir: next_dir,
                            },
                            field,
                        )
                    })
                })
                .unwrap()
            })
    }

    fn perform_part2(&self, State { mut p, mut dir }: State, instr: Instruction) -> State {
        match instr {
            Turn(lr) => State {
                p,
                dir: match lr {
                    LR::L => dir.shift(Dir::Up),
                    LR::R => dir.shift(Dir::Down),
                },
            },
            Go(n) => {
                for _ in 0..n {
                    match self.wrap_cube(State {
                        p: p.clone(),
                        dir: dir.clone(),
                    }) {
                        (_, Field::Wall) => break,
                        (
                            State {
                                p: new_p,
                                dir: new_dir,
                            },
                            Field::Free,
                        ) => {
                            p = new_p;
                            dir = new_dir
                        }
                    }
                }
                State { p, dir }
            }
        }
    }
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> i32 {
        let (map, instructions) = parse::<0>(s);
        let dir = Dir::Right;
        let mut state = State {
            p: map.first_p(State {
                p: P { x: 0, y: 0 },
                dir: dir.clone(),
            }),
            dir,
        };
        state = instructions.fold(state, |s, instr| map.perform_part1(s, instr));
        calc_password(state)
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 6032);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 26558);
    }
}

pub mod part2 {
    use super::*;

    pub fn solution<const TILE_SIZE: usize>(s: &str) -> i32 {
        let (map, instructions) = parse::<TILE_SIZE>(s);
        let dir = Dir::Right;
        let mut state = State {
            p: map.first_p(State {
                p: P { x: 0, y: 0 },
                dir: dir.clone(),
            }),
            dir,
        };
        state = instructions.fold(state, |s, instr| map.perform_part2(s, instr));
        calc_password(state)
    }

    #[test]
    fn sample() {
        assert_eq!(solution::<4>(SAMPLE), 5031);
    }
    #[test]
    fn actual() {
        assert_eq!(solution::<50>(INPUT), 110400);
    }
}
