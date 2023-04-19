use std::collections::HashMap;
use std::iter::{Cycle, Enumerate};

pub const SAMPLE: &str = include_str!("sample");
pub const INPUT: &str = include_str!("input");

fn rocks() -> [Rock; N_ROCKS] {
    "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##"
    .split("\n\n")
    .map(|s| {
        Rock(
            s.lines()
                .map(|l| {
                    l.chars().fold(0, |acc, c| match c {
                        '#' => (acc << 1) + 1,
                        '.' => acc << 1,
                        _ => panic!("Invalid `{c}`"),
                    }) << (WIDTH - X_OFF - l.len())
                })
                .collect(),
        )
    })
    .collect::<Vec<_>>()
    .try_into()
    .unwrap()
}

/// One horizontal line in a _grid_
type Line = u8;

/// Empty line with wall on the left
const EMPTY: Line = 1 << 7;
/// Width of the chamber
const WIDTH: usize = 7;
/// Offset of spawned rock from left wall
const X_OFF: usize = 2;
/// Offset of spawned rock from highest solid rock
const Y_OFF: usize = 3;
/// Number of different rocks
const N_ROCKS: usize = 5;

struct World<'a> {
    solid_rocks: Vec<Line>,
    height: usize,
    winds: RememberIdxCycle<'a>,
    rocks: std::iter::Cycle<std::array::IntoIter<Rock, N_ROCKS>>,
}

struct RememberIdxCycle<'a> {
    it: Cycle<Enumerate<std::slice::Iter<'a, u8>>>,
    last_idx: usize,
}

impl<'a> RememberIdxCycle<'a> {
    fn next_unwrap(&mut self) -> u8 {
        let (i, r) = self.it.next().unwrap();
        self.last_idx = i;
        *r
    }
}

impl std::fmt::Debug for World<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("World [\n")?;
        for l in self.solid_rocks.iter().rev() {
            f.write_fmt(format_args!("  {l:0>8b}\n"))?;
        }
        f.write_str("]")?;
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
struct Rock(Vec<Line>);

impl std::fmt::Debug for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Rock [\n")?;
        for l in self.0.iter() {
            f.write_fmt(format_args!("  {l:0>8b}\n"))?;
        }
        f.write_str("]")?;
        Ok(())
    }
}

impl Rock {
    fn rotate_left(self, n: u32) -> Self {
        Self(self.0.into_iter().map(|l| l.rotate_left(n)).collect())
    }
    fn rotate_right(self, n: u32) -> Self {
        Self(self.0.into_iter().map(|l| l.rotate_right(n)).collect())
    }

    /// Checks if `self` and `win` have fields that overlap.
    /// The lines in the window `win` are represented upside down
    fn overlaps(&self, win: &[Line]) -> bool {
        !self.0.iter().rev().zip(win).all(|(a, b)| (a & b) == 0)
    }
}

impl<'a> World<'a> {
    fn new(winds: &'a str) -> Self {
        World {
            solid_rocks: Vec::new(),
            height: 0,
            winds: RememberIdxCycle {
                it: winds.trim().as_bytes().iter().enumerate().cycle(),
                last_idx: 0,
            },
            rocks: rocks().into_iter().cycle(),
        }
    }

    fn add_rock(&mut self) {
        let mut rock = self.rocks.next().unwrap();
        let mut rock_altitude = self.height + rock.0.len() + Y_OFF;
        while (self.solid_rocks.len()) < rock_altitude {
            self.solid_rocks.push(EMPTY);
        }
        for win in self.solid_rocks[0..rock_altitude]
            .windows(rock.0.len())
            .rev()
        {
            if rock.overlaps(win) {
                break;
            }

            let next_rock = match self.winds.next_unwrap() {
                b'<' => rock.clone().rotate_left(1),
                b'>' => rock.clone().rotate_right(1),
                b => panic!("Invalid `{b}`"),
            };

            if !next_rock.overlaps(win) {
                rock = next_rock;
            }

            rock_altitude -= 1;
        }
        rock_altitude += 1; // use last altitude without overlap/wihin world
        self.solid_rocks[rock_altitude - rock.0.len()..rock_altitude]
            .iter_mut()
            .zip(rock.0.into_iter().rev())
            .for_each(|(w, r)| *w |= r);
        self.height = self.height.max(rock_altitude);
    }
}

fn solve(s: &str, count: usize) -> usize {
    let mut world = World::new(s);
    let mut mem = HashMap::<usize, Vec<(usize, usize)>>::new();
    for n_rocks_left in (0..count).rev() {
        world.add_rock();

        // Just record the `height` and `n_rocks_left` for all wind idxs
        // We could also take into account the rock idx, but it doesn't seem to matter
        if let Some(hist) = mem.get_mut(&world.winds.last_idx) {
            if hist.len() == 3 {
                hist.remove(0);
            };
            hist.push((world.height, n_rocks_left));
        } else {
            mem.insert(world.winds.last_idx, vec![(world.height, n_rocks_left)]);
        }

        if let Some(hist) = mem.get(&world.winds.last_idx) {
            // Look for two deltas that are the same
            if let [(h0, i0), (h1, i1), (h2, i2)] = hist.as_slice() {
                let dh = h1 - h0;
                let di = i0 - i1; // delta of n_left_rocks (`di` is positive)
                if dh == h2 - h1 && di == i1 - i2 {
                    (0..n_rocks_left % di).for_each(|_| world.add_rock());
                    return dh * (n_rocks_left / di) // integer multiple part
                    + world.height; // Parts before and after that
                }
            }
        }
    }
    world.height
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> usize {
        solve(s, 2022)
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 3068);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 3200);
    }
}
pub mod part2 {
    use super::*;

    pub fn solution(s: &str) -> usize {
        solve(s, 1_000_000_000_000)
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 1514285714288);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 1584927536247);
    }
}
