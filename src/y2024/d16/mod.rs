use std::{
    collections::{HashSet, VecDeque},
    ops::{Index, IndexMut},
    str::FromStr,
};

use itertools::Itertools;

pub const INPUT: &str = include_str!("input.txt");

pub const SAMPLE1: &str = include_str!("sample1.txt");
pub const SAMPLE2: &str = include_str!("sample2.txt");

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> u32 {
        Map::from_str(s).unwrap().minimum_score()
    }

    #[test]
    fn sample1() {
        assert_eq!(solution(SAMPLE1), 7036);
    }

    #[test]
    fn sample2() {
        assert_eq!(solution(SAMPLE2), 11048);
    }

    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 94436);
    }
}

pub mod part2 {
    use super::*;

    pub fn solution(s: &str) -> u32 {
        Map::from_str(s).unwrap().count_tiles()
    }

    #[test]
    fn sample1() {
        assert_eq!(solution(SAMPLE1), 45);
    }

    #[test]
    fn sample2() {
        assert_eq!(solution(SAMPLE2), 64);
    }

    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 481);
    }
}

impl Map {
    fn minimum_score(&self) -> u32 {
        let scores = self.flood();
        let end = self.find_singleton(Field::End);
        let score = scores[end.idx].minimum();
        assert!(score != u32::MAX, "Could not find a path");
        score
    }

    fn count_tiles(&self) -> u32 {
        let scores = self.flood();

        let mut queue = VecDeque::from_iter(self.all_end_states(&scores));
        let mut seen = HashSet::new();
        while let Some(State {
            position,
            direction,
            score,
        }) = queue.pop_front()
        {
            seen.insert(position.idx);
            let new_states = [
                (direction, score.checked_sub(1)),
                (direction.left(), score.checked_sub(1000 + 1)),
                (direction.right(), score.checked_sub(1000 + 1)),
            ]
            .into_iter()
            .filter_map(|(direction, score)| {
                let position = position.step(direction.reverse())?;
                let score = score?;
                (score == *scores[position.idx].index(direction)).then_some(State {
                    position,
                    direction,
                    score,
                })
            });
            queue.extend(new_states)
        }
        seen.len().try_into().unwrap()
    }

    fn flood(&self) -> Vec<Score> {
        let mut scores = vec![Score::new(); self.fields.len()];
        let start = State {
            position: self.find_singleton(Field::Start),
            direction: Direction::East,
            score: 0,
        };
        scores[start.position.idx][start.direction] = 0;
        let mut queue = VecDeque::from([start]);
        while let Some(State {
            position,
            direction,
            score,
        }) = queue.pop_front()
        {
            let new_states = [
                (position.step(direction), direction, score + 1),
                (Some(position), direction.left(), score + 1000),
                (Some(position), direction.right(), score + 1000),
                // We don't have to look back, because it has already been visited
                // and startpoint always has wall "behind" (west) it
            ]
            .into_iter()
            .filter_map(|(position, direction, score)| {
                // Use this inner loop to eliminate states earlier for less allocations
                let position = position?;
                let stored_score = scores[position.idx].index_mut(direction);
                (score <= *stored_score).then(|| {
                    *stored_score = score;
                    State {
                        position,
                        direction,
                        score,
                    }
                })
            });
            queue.extend(new_states)
        }
        scores
    }

    /// Get all end states that have the minimum score
    fn all_end_states<'a>(&'a self, scores: &'a [Score]) -> impl Iterator<Item = State<'a>> {
        const ALL_DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        let end = self.find_singleton(Field::End);
        let score = scores[end.idx].minimum();
        assert!(score != u32::MAX, "Could not find a path");
        scores[end.idx]
            .0
            .iter()
            .positions(move |&v| v == score)
            .map(move |i| State {
                position: end,
                direction: ALL_DIRECTIONS[i],
                score,
            })
    }

    #[must_use]
    fn find_singleton(&self, field: Field) -> MapIndex<'_> {
        let mut it = self.fields.iter();
        let idx = it
            .position(|f| *f == field)
            .expect("Could not find {field}");
        assert!(!it.any(|f| *f == field), "There should only be one");
        MapIndex { idx, map: self }
    }
}

struct State<'a> {
    position: MapIndex<'a>,
    direction: Direction,
    score: u32,
}

/// Loss
#[derive(Clone, Debug)]
struct Score([u32; 4] /* One for each direction */);

impl Score {
    fn new() -> Self {
        Score([u32::MAX; 4])
    }

    fn minimum(&self) -> u32 {
        *self.0.iter().min().unwrap()
    }
}

impl Index<Direction> for Score {
    type Output = u32;

    fn index(&self, index: Direction) -> &Self::Output {
        self.0.index(index.to_int())
    }
}

impl IndexMut<Direction> for Score {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        self.0.index_mut(index.to_int())
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_int(self) -> usize {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }

    fn left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn reverse(self) -> Self {
        self.left().left()
    }
}

#[derive(Clone, Copy)]
struct MapIndex<'a> {
    idx: usize,
    map: &'a Map,
}

impl<'a> MapIndex<'a> {
    fn field(&self) -> Field {
        self.map.fields[self.idx]
    }

    /// Return the index to the field in the desired direction if it's not a wall
    fn step(self, direction: Direction) -> Option<MapIndex<'a>> {
        Some(Self {
            idx: match direction {
                Direction::North => self.idx.checked_sub(self.map.width),
                Direction::East => self
                    .idx
                    .checked_add(1)
                    .filter(|i| i.rem_euclid(self.map.width) != 0),
                Direction::South => self
                    .idx
                    .checked_add(self.map.width)
                    .filter(|i| *i < self.map.fields.len()),
                Direction::West => self
                    .idx
                    .checked_sub(1)
                    .filter(|_| self.idx.rem_euclid(self.map.width) != 0),
            }
            .expect("We should never step out of the map"),
            map: self.map,
        })
        .filter(|i| !matches!(i.field(), Field::Wall))
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Field {
    Wall,
    Start,
    End,
    Empty,
}

struct Map {
    fields: Vec<Field>,
    width: usize,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map {
            fields: s
                .lines()
                .flat_map(|l| {
                    l.chars().map(|c| match c {
                        '#' => Field::Wall,
                        '.' => Field::Empty,
                        'S' => Field::Start,
                        'E' => Field::End,
                        c => panic!("Unknown char {c}"),
                    })
                })
                .collect(),
            width: s.find('\n').unwrap(),
        })
    }
}
