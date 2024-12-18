use std::str::FromStr;
use std::{collections::VecDeque, ops::IndexMut};

pub const INPUT: &str = include_str!("input.txt");

pub const SAMPLE: &str = include_str!("sample.txt");

pub mod part1 {
    use super::*;

    pub fn solution<'a, const N: usize>(s: impl Iterator<Item = &'a str>) -> u32 {
        Map::<N>::from_coords(s).flood()
    }

    #[test]
    fn sample() {
        assert_eq!(solution::<7>(SAMPLE.lines().take(12)), 22);
    }

    #[test]
    fn actual() {
        assert_eq!(solution::<71>(INPUT.lines().take(1024)), 326);
    }
}

pub mod part2 {
    use super::*;

    pub fn solution<const N: usize>(s: &str) -> &str {
        Map::<N>::blocking_coord(s)
    }

    #[test]
    fn sample() {
        assert_eq!(solution::<7>(SAMPLE), "6,1");
    }

    #[test]
    // #[ignore = "slow"]
    fn actual() {
        assert_eq!(solution::<71>(INPUT), "18,62");
    }
}

impl<const N: usize> Map<N> {
    fn blocking_coord(s: &str) -> &str {
        let mut map = Map::<N>::default();
        s.lines()
            .find(|coord| {
                map.put_block(coord);
                map.flood() == u32::MAX
            })
            .unwrap()
    }

    fn from_coords<'a>(s: impl Iterator<Item = &'a str>) -> Self {
        let mut map = Map::default();
        s.for_each(|coord| {
            map.put_block(coord);
        });
        map
    }

    fn put_block(&mut self, coord: &str) {
        let (x, y) = coord.split_once(',').unwrap();
        let idx = usize::from_str(x).unwrap() + usize::from_str(y).unwrap() * N;
        self.fields[idx] = Field::Wall
    }

    fn flood(&self) -> u32 {
        let mut distances = vec![Distance::default(); self.fields.len()];
        let start = State {
            position: self.start(),
            distance: 0,
        };
        distances[start.position.idx] = Distance(0);
        let mut queue = VecDeque::from([start]);
        while let Some(State { position, distance }) = queue.pop_front() {
            // dbg!(position.idx, distance);
            let new_states = [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
            .into_iter()
            .filter_map(|direction| {
                let position = position.step(direction)?;
                let distance = distance + 1;
                let stored_distance = distances.index_mut(position.idx);
                (distance < stored_distance.0).then(|| {
                    stored_distance.0 = distance;
                    State { position, distance }
                })
            });
            queue.extend(new_states)
        }
        distances[self.end().idx].0
    }

    fn start(&self) -> MapIndex<'_, N> {
        MapIndex { idx: 0, map: self }
    }
    fn end(&self) -> MapIndex<'_, N> {
        MapIndex {
            idx: N * N - 1,
            map: self,
        }
    }
}

#[derive(Clone, Debug)]
struct Distance(u32);

impl Default for Distance {
    fn default() -> Self {
        Self(u32::MAX)
    }
}

struct State<'a, const N: usize> {
    position: MapIndex<'a, N>,
    distance: u32,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy)]
struct MapIndex<'a, const N: usize> {
    idx: usize,
    map: &'a Map<N>,
}

impl<'a, const N: usize> MapIndex<'a, N> {
    fn field(&self) -> Field {
        self.map.fields[self.idx]
    }

    /// Return the index to the field in the desired direction if it's not a wall
    fn step(self, direction: Direction) -> Option<MapIndex<'a, N>> {
        Some(Self {
            idx: match direction {
                Direction::North => self.idx.checked_sub(N),
                Direction::East => self.idx.checked_add(1).filter(|i| i.rem_euclid(N) != 0),
                Direction::South => self
                    .idx
                    .checked_add(N)
                    .filter(|i| *i < self.map.fields.len()),
                Direction::West => self
                    .idx
                    .checked_sub(1)
                    .filter(|_| self.idx.rem_euclid(N) != 0),
            }?,
            map: self.map,
        })
        .filter(|i| !matches!(i.field(), Field::Wall))
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Field {
    Wall,
    Empty,
}

struct Map<const N: usize> {
    fields: Vec<Field>,
}

impl<const N: usize> Default for Map<N> {
    fn default() -> Self {
        Map {
            fields: vec![Field::Empty; N * N],
        }
    }
}
