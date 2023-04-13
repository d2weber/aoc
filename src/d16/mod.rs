use itertools::Itertools;
use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub const SAMPLE: &str = include_str!("sample");
pub const INPUT: &str = include_str!("input");

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Id(u16);

impl Id {
    fn new(value: &str) -> Self {
        assert_eq!(value.len(), 2);
        Self::new_unchecked(value)
    }
    const fn new_unchecked(value: &str) -> Self {
        let value = value.as_bytes();
        Id(value[0] as u16 | (value[1] as u16) << 8)
    }
}

impl From<&str> for Id {
    fn from(value: &str) -> Self {
        assert_eq!(value.len(), 2);
        Id::new(value)
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = (self.0 & 0xFF) as u8;
        let b = (self.0 >> 8) as u8; // & 0xFF;
        f.write_str(std::str::from_utf8(&[a, b]).unwrap())
    }
}

#[test]
fn id_debug() {
    assert_eq!(format!("{:?}", Id::new("AZ")), "AZ")
}

#[derive(Debug)]
struct ValveMap(HashMap<Id, Valve>);

impl FromIterator<(Id, Valve)> for ValveMap {
    fn from_iter<T: IntoIterator<Item = (Id, Valve)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

impl Deref for ValveMap {
    type Target = HashMap<Id, Valve>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ValveMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Valve {
    flow_rate: u32,
    distances: HashMap<Id, u32>,
    direct_connections: Vec<Id>,
}

#[derive(Debug)]
struct StepArgs {
    start: Id,
    current: Id,
    distance: u32,
}

fn step(
    StepArgs {
        start,
        current,
        distance,
    }: StepArgs,
    valves: &mut ValveMap,
) -> Vec<StepArgs> {
    let ds = &mut valves.get_mut(&start).unwrap().distances;
    if *ds.get(&current).unwrap_or(&u32::MAX) > distance {
        ds.insert(current, distance);
        let r = valves[&current]
            .direct_connections
            .iter()
            .filter(|id| **id != start)
            .map(|id| StepArgs {
                start,
                current: *id,
                distance: distance + 1,
            })
            .collect();
        r
    } else {
        vec![]
    }
}

fn parse(s: &str) -> ValveMap {
    let mut valves: ValveMap = s
        .lines()
        .map(|l| {
            let (id, l) = l
                .strip_prefix("Valve ")
                .unwrap()
                .split_once(" has flow rate=")
                .unwrap();

            let id = Id::new(id);
            let (flow_rate, l) = l
                .split_once("; tunnels lead to valves ")
                .unwrap_or_else(|| l.split_once("; tunnel leads to valve ").unwrap());
            (
                id,
                Valve {
                    flow_rate: flow_rate.parse().unwrap(),
                    direct_connections: l.split(", ").map(|v| Id::new(v)).collect(),
                    distances: HashMap::default(),
                },
            )
        })
        .collect();
    let mut args: Vec<StepArgs> = valves
        .iter()
        .flat_map(
            |(
                id,
                Valve {
                    direct_connections, ..
                },
            )| {
                direct_connections.iter().map(|dst| StepArgs {
                    start: *id,
                    current: *dst,
                    distance: 1,
                })
            },
        )
        .collect();
    while !args.is_empty() {
        args = args
            .into_iter()
            .flat_map(|a| step(a, &mut valves))
            .collect();
    }
    valves.retain(|k, v| *k == START_ID || v.flow_rate > 0);
    return valves;
}

const START_ID: Id = Id::new_unchecked("AA");

struct NextArgs {
    current: Id,
    unvisited: Vec<Id>,
    steps_left: u32,
    total_flow: u32,
}

fn next(
    NextArgs {
        current,
        unvisited,
        steps_left,
        total_flow,
    }: NextArgs,
    valves: &ValveMap,
    max_flow: &mut u32,
) -> Vec<NextArgs> {
    unvisited
        .iter()
        .enumerate()
        .filter_map(|(i, next)| {
            let Some(steps_left) = steps_left.checked_sub(
                valves[&current].distances[&next] /* time to walk */
                 + 1, /* time to open the valve */
            ) else {
                return None;
            };
            let total_flow = total_flow + valves[&next].flow_rate * steps_left;
            *max_flow = std::cmp::max(total_flow, *max_flow);

            let mut unvisited = unvisited.clone();
            unvisited.swap_remove(i);
            Some(NextArgs {
                current: *next,
                unvisited,
                steps_left,
                total_flow,
            })
        })
        .collect()
}

fn start_iteration(valves: &ValveMap, unvisited: Vec<Id>, n_steps: u32) -> u32 {
    let mut args = vec![NextArgs {
        current: START_ID,
        unvisited,
        steps_left: n_steps,
        total_flow: 0,
    }];
    let mut max_flow = 0;
    while !args.is_empty() {
        args = args
            .into_iter()
            .flat_map(|a| next(a, &valves, &mut max_flow))
            .collect();
    }
    max_flow
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> u32 {
        let valves = parse(s);
        start_iteration(
            &valves,
            valves
                .clone()
                .into_iter()
                .map(|(k, _)| k)
                .filter(|k| *k != START_ID)
                .collect(),
            30,
        )
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 1651);
    }
    #[test]
    #[ignore = "slow"]
    fn actual() {
        assert_eq!(solution(INPUT), 1880);
    }
}

pub mod part2 {
    use super::*;

    pub fn solution(s: &str) -> u32 {
        let valves = parse(s);
        let unvisited: Vec<Id> = valves
            .clone()
            .into_iter()
            .map(|(k, _)| k)
            .filter(|k| *k != START_ID)
            .collect();
        let mut max = 0;
        let ceil_half = (unvisited.len() + 1) / 2;
        for n_elephant in 0..=ceil_half {
            unvisited
                .clone()
                .into_iter()
                .combinations(n_elephant)
                .for_each(|unvisited_el| {
                    let max_my = start_iteration(
                        &valves,
                        unvisited
                            .clone()
                            .into_iter()
                            .filter(|k| !unvisited_el.contains(k))
                            .collect(),
                        26,
                    );
                    let max_el = start_iteration(&valves, unvisited_el, 26);
                    max = std::cmp::max(max_my + max_el, max);
                })
        }
        max
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 1707);
    }
    #[test]
    #[ignore = "slow"]
    fn actual() {
        assert_eq!(solution(INPUT), 2520);
    }
}
