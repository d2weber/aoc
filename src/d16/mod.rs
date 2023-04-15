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
    max_flow: &mut u32,
) {
    unvisited.iter().enumerate().for_each(|(i, next)| {
        let Some(steps_left) = steps_left.checked_sub(
                valves[&current].distances[next] /* time to walk */
                 + 1 /* time to open the valve */
            ).filter(|&steps_left| steps_left != 0) else {
                return;
            };
        let total_flow = total_flow + valves[next].flow_rate * steps_left;
        *max_flow = std::cmp::max(total_flow, *max_flow);

        let mut unvisited = unvisited.clone();
        unvisited.swap_remove(i);
        find_max(
            NextArgs {
                current: *next,
                unvisited,
                steps_left,
                total_flow,
            },
            valves,
            max_flow,
        );
    });
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> u32 {
        let valves = parse(s);
        let mut total_flow = 0;
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
        let mut max = 0;
        for n_elephant in 0..=(unvisited.len() / 2) {
            unvisited
                .clone()
                .into_iter()
                .combinations(n_elephant)
                .for_each(|unvisited_el| {
                    let mut curr_max = 0;
                    find_max(
                        NextArgs {
                            current: START_ID,
                            unvisited: unvisited
                                .clone()
                                .into_iter()
                                .filter(|k| !unvisited_el.contains(k))
                                .collect(),
                            steps_left: 26,
                            total_flow: 0,
                        },
                        &valves,
                        &mut curr_max,
                    );
                    find_max(
                        NextArgs {
                            current: START_ID,
                            unvisited: unvisited_el,
                            steps_left: 26,
                            total_flow: curr_max,
                        },
                        &valves,
                        &mut curr_max,
                    );
                    max = std::cmp::max(curr_max, max);
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
