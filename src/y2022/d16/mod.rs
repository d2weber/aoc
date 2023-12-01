//! This solution was made ugly to experiment with certain optimizations.
//!

use itertools::Itertools;
use std::collections::BTreeSet;
use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub const SAMPLE: &str = include_str!("sample");
pub const INPUT: &str = include_str!("input");

#[derive(Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
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
                    direct_connections: l.split(", ").map(Id::new).collect(),
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
    valves
}

const START_ID: Id = Id::new_unchecked("AA");

struct NextArgs {
    current: Id,
    unvisited: Vec<Id>,
    steps_left: u32,
    total_flow: u32,
}

fn find_max(
    NextArgs {
        current,
        unvisited,
        steps_left,
        total_flow,
    }: NextArgs,
    valves: &ValveMap,
    memory: &mut HashMap<BTreeSet<usize>, u32>,
    max_flow: &mut u32,
) {
    let indices: HashMap<Id, usize> = valves
        .0
        .keys()
        .sorted_unstable()
        .cloned()
        .zip(0..)
        .collect();
    let ids: HashMap<usize, Id> = valves
        .0
        .keys()
        .sorted_unstable()
        .cloned()
        .enumerate()
        .collect();
    let u = unvisited.into_iter().map(|k| indices[&k]).collect();
    let v = valves
        .0
        .iter()
        .sorted_unstable_by_key(|(k, _)| *k)
        .enumerate()
        .inspect(|(i, (k, _))| {
            assert_eq!(*i, indices[k]);
        })
        .map(
            |(
                _,
                (
                    _,
                    Valve {
                        flow_rate,
                        distances,
                        ..
                    },
                ),
            )| {
                _Valve {
                    flow_rate: *flow_rate,
                    distances: (0..ids.len())
                        .map(|i| *distances.get(&ids[&i]).unwrap_or(&0u32))
                        .collect(),
                }
            },
        )
        .collect();
    _find_max(
        _NextArgs {
            current: indices[&current],
            unvisited: u,
            steps_left,
            total_flow,
        },
        &v,
        memory,
        max_flow,
    );
}

#[derive(Clone, Debug)]
struct _Valve {
    flow_rate: u32,
    distances: Vec<u32>,
}

struct _NextArgs {
    current: usize,
    unvisited: Vec<usize>,
    steps_left: u32,
    total_flow: u32,
}

fn _find_max(
    _NextArgs {
        current,
        unvisited,
        steps_left,
        total_flow,
    }: _NextArgs,
    valves: &Vec<_Valve>,
    memory: &mut HashMap<BTreeSet<usize>, u32>,
    max_flow: &mut u32,
) {
    let k = BTreeSet::from_iter(unvisited.clone());
    let f = *memory.get(&k).unwrap_or(&0).max(&total_flow);
    memory.insert(k, f);
    unvisited.iter().enumerate().for_each(|(i, next)| {
        let Some(steps_left) = steps_left
            .checked_sub(
                valves[current].distances[*next] /* time to walk */
                 + 1, /* time to open the valve */
            )
            .filter(|&steps_left| steps_left != 0)
        else {
            return;
        };
        let total_flow = total_flow + valves[*next].flow_rate * steps_left;
        *max_flow = std::cmp::max(total_flow, *max_flow);

        let mut unvisited = unvisited.clone();
        unvisited.swap_remove(i);
        _find_max(
            _NextArgs {
                current: *next,
                unvisited,
                steps_left,
                total_flow,
            },
            valves,
            memory,
            max_flow,
        );
    });
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> u32 {
        let valves = parse(s);
        let mut total_flow = 0;
        let mut memory = HashMap::new();
        find_max(
            NextArgs {
                current: START_ID,
                unvisited: valves
                    .clone()
                    .into_keys()
                    .filter(|k| *k != START_ID)
                    .collect(),
                steps_left: 30,
                total_flow: 0,
            },
            &valves,
            &mut memory,
            &mut total_flow,
        );
        total_flow
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
            .into_keys()
            .filter(|k| *k != START_ID)
            .collect();
        let mut _max = 0;
        let mut memory = HashMap::new();
        let mut max = 0;
        find_max(
            NextArgs {
                current: START_ID,
                unvisited: unvisited.clone(),
                steps_left: 26,
                total_flow: 0,
            },
            &valves,
            &mut memory,
            &mut max,
        );
        let indices: HashMap<Id, usize> = valves
            .0
            .keys()
            .sorted_unstable()
            .cloned()
            .zip(0..)
            .collect();
        let unvisited: Vec<usize> = unvisited.into_iter().map(|k| indices[&k]).collect();
        memory
            .iter()
            .flat_map(|(my_unvisited, my_max)| {
                memory.iter().filter_map(|(el_unvisited, el_max)| {
                    if unvisited
                        .iter()
                        .all(|u| my_unvisited.contains(u) | el_unvisited.contains(u))
                    {
                        Some(*my_max + *el_max)
                    } else {
                        None
                    }
                })
            })
            .max()
            .unwrap()
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
