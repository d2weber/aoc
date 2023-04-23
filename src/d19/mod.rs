use std::ops::{Add, Index, IndexMut};

pub const SAMPLE: &str = include_str!("sample");
pub const INPUT: &str = include_str!("input");

#[derive(Clone, Copy)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

use Resource::*;

#[derive(Default, Clone)]
struct Re<T>([T; 4]);

impl<T> Index<Resource> for Re<T> {
    type Output = T;

    fn index(&self, index: Resource) -> &Self::Output {
        &self.0[index as usize]
    }
}
impl<T> IndexMut<Resource> for Re<T> {
    fn index_mut(&mut self, index: Resource) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl Add<&Re<u32>> for Re<u32> {
    type Output = Re<u32>;

    fn add(self, rhs: &Re<u32>) -> Self::Output {
        // This notation is verbose but more performant
        Re([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
        ])
    }
}

impl Re<u32> {
    fn add_one(mut self, r: Resource) -> Self {
        self[r] += 1;
        self
    }

    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        Some(Re([
            self.0[0].checked_sub(rhs.0[0])?,
            self.0[1].checked_sub(rhs.0[1])?,
            self.0[2].checked_sub(rhs.0[2])?,
            self.0[3].checked_sub(rhs.0[3])?,
        ]))
    }
}

type Blueprint = Re<Re<u32>>;

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
    inventory: Re<u32>,
    n_robots: Re<u32>,
    minutes_left: u32,
    skip_clay: bool,
    skip_ore: bool,
}

impl<'a> State<'a> {
    fn new(costs: &'a Blueprint, minutes_left: u32) -> State<'a> {
        Self {
            costs,
            inventory: Default::default(),
            n_robots: Re::<u32>::default().add_one(Ore),
            minutes_left,
            skip_clay: false,
            skip_ore: false,
        }
    }

    fn buy(&self, r: Resource) -> Option<State> {
        self.inventory.checked_sub(&self.costs[r]).map(|inv| State {
            costs: self.costs,
            inventory: inv + &self.n_robots,
            n_robots: self.n_robots.clone().add_one(r),
            minutes_left: self.minutes_left - 1,
            skip_clay: false,
            skip_ore: false,
        })
    }

    fn buy_none(self, skip_clay: bool, skip_ore: bool) -> Self {
        State {
            costs: self.costs,
            inventory: self.inventory + &self.n_robots,
            n_robots: self.n_robots,
            minutes_left: self.minutes_left - 1,
            skip_clay,
            skip_ore,
        }
    }

    fn max_geodes(self) -> u32 {
        if self.minutes_left == 1 {
            return self.inventory[Geode] + self.n_robots[Geode];
        }
        if let Some(n) = self.buy(Geode).map(|s| s.max_geodes()) {
            return n;
        }

        let buy_clay = Some(&self)
            .filter(|s| !s.skip_clay)
            .filter(|s| s.minutes_left > 5) // improbable to improve when buying late
            .and_then(|s| s.buy(Clay))
            .map(|s| s.max_geodes());

        if let Some(n) = Some(&self)
            .filter(|s| s.minutes_left > 3)
            .and_then(|s| s.buy(Obsidian))
            .map(|s| s.max_geodes())
        {
            return n.max(buy_clay.unwrap_or(0));
        }

        let buy_ore = Some(&self)
            .filter(|s| !s.skip_ore)
            .filter(|s| s.minutes_left > 9) // improbable to improve when buying late
            .and_then(|s| s.buy(Ore))
            .map(|s| s.max_geodes());

        self.buy_none(buy_clay.is_some(), buy_ore.is_some())
            .max_geodes()
            .max(buy_clay.unwrap_or(0))
            .max(buy_ore.unwrap_or(0))
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
