use std::ops::{Add, Index, IndexMut};

pub const SAMPLE: &str = include_str!("sample");
pub const INPUT: &str = include_str!("input");

#[derive(Clone, Copy)]
enum Kind {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

use Kind::*;

#[derive(Default, Clone)]
struct KindMap<T>([T; 4]);

impl<T> Index<Kind> for KindMap<T> {
    type Output = T;

    fn index(&self, index: Kind) -> &Self::Output {
        &self.0[index as usize]
    }
}
impl<T> IndexMut<Kind> for KindMap<T> {
    fn index_mut(&mut self, index: Kind) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl Add<&KindMap<u32>> for KindMap<u32> {
    type Output = KindMap<u32>;

    fn add(self, rhs: &KindMap<u32>) -> Self::Output {
        // This notation is verbose but more performant
        KindMap([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
        ])
    }
}

impl KindMap<u32> {
    fn add_one(mut self, r: Kind) -> Self {
        self[r] += 1;
        self
    }

    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        Some(KindMap([
            self.0[0].checked_sub(rhs.0[0])?,
            self.0[1].checked_sub(rhs.0[1])?,
            self.0[2].checked_sub(rhs.0[2])?,
            self.0[3].checked_sub(rhs.0[3])?,
        ]))
    }
}

type Blueprint = KindMap<KindMap<u32>>;

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Blueprint> {
    lines
        .map(|bp| {
            let (_, s) = bp
                .strip_prefix("Blueprint ")
                .unwrap()
                .split_once(": Each ore robot costs ")
                .unwrap();
            let mut bp: Blueprint = Default::default();
            let (v, s) = s.split_once(" ore. Each clay robot costs ").unwrap();
            bp[Ore][Ore] = v.parse().unwrap();
            let (v, s) = s.split_once(" ore. Each obsidian robot costs ").unwrap();
            bp[Clay][Ore] = v.parse().unwrap();
            let (v, s) = s.split_once(" ore and ").unwrap();
            bp[Obsidian][Ore] = v.parse().unwrap();
            let (v, s) = s.split_once(" clay. Each geode robot costs ").unwrap();
            bp[Obsidian][Clay] = v.parse().unwrap();
            let (v, s) = s.split_once(" ore and ").unwrap();
            bp[Geode][Ore] = v.parse().unwrap();
            bp[Geode][Obsidian] = s.strip_suffix(" obsidian.").unwrap().parse().unwrap();
            bp
        })
        .collect()
}

#[derive(Clone)]
struct State<'a> {
    costs: &'a Blueprint,
    inventory: KindMap<u32>,
    n_robots: KindMap<u32>,
    minutes_left: u32,
    skip_clay: bool,
    skip_ore: bool,
}

/// Minimum number of minutes that have to be left to construct a robot of
/// this type. To find all possible solutions these numbers should smaller
/// but it's much faster like this
const MIN_CLAY: u32 = 6;
const MIN_ORE: u32 = 10;

impl<'a> State<'a> {
    fn new(costs: &'a Blueprint, minutes_left: u32) -> State<'a> {
        Self {
            costs,
            inventory: Default::default(),
            n_robots: KindMap::<u32>::default().add_one(Ore),
            minutes_left,
            skip_clay: false,
            skip_ore: false,
        }
    }

    #[inline(always)]
    fn construct(&self, r: Kind) -> Option<State> {
        self.inventory.checked_sub(&self.costs[r]).map(|inv| State {
            costs: self.costs,
            inventory: inv + &self.n_robots,
            n_robots: self.n_robots.clone().add_one(r),
            minutes_left: self.minutes_left - 1,
            skip_clay: false,
            skip_ore: false,
        })
    }

    #[inline(always)]
    fn max_geodes_constructing(&self, r: Kind) -> Option<u32> {
        self.construct(r).map(|s| s.max_geodes())
    }

    /// Run the simulation, recurring eagerly
    fn max_geodes(self) -> u32 {
        if self.minutes_left == 1 {
            return self.inventory[Geode] + self.n_robots[Geode];
        }

        if let Some(n) = self.max_geodes_constructing(Geode) {
            return n;
        }

        if self.minutes_left == 2 {
            return self.inventory[Geode] + self.n_robots[Geode] + self.n_robots[Geode];
        }

        let construct_clay = if self.skip_clay || self.minutes_left < MIN_CLAY {
            None
        } else {
            self.max_geodes_constructing(Clay)
        };

        if let Some(n) = self.max_geodes_constructing(Obsidian) {
            return n.max(construct_clay.unwrap_or(0));
        }

        let construct_ore = if self.skip_ore || self.minutes_left < MIN_ORE {
            None
        } else {
            self.max_geodes_constructing(Ore)
        };

        State {
            // Don't construct anything (wait)
            inventory: self.inventory + &self.n_robots,
            minutes_left: self.minutes_left - 1,
            // The reason we will never consider constructing one of these robots is
            // that if we are not constructing any robot but we could, it would never
            // make sense to construct that robot again before constructing anything else
            skip_clay: construct_clay.is_some(),
            skip_ore: construct_ore.is_some(),
            ..self
        }
        .max_geodes()
        .max(construct_clay.unwrap_or(0))
        .max(construct_ore.unwrap_or(0))
    }
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> u32 {
        let bps = parse(s.lines());
        bps.iter()
            .zip(1..)
            .map(|(bp, n)| n * State::new(bp, 24).max_geodes())
            .sum()
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 33);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 1382);
    }
}

pub mod part2 {
    use super::*;

    pub fn solution(s: &str) -> u32 {
        let bps = parse(s.lines().take(3));
        bps.iter()
            .map(|bp| State::new(bp, 32).max_geodes())
            .product()
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 3472);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 31740);
    }
}
