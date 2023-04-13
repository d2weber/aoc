use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

pub const SAMPLE: &str = include_str!("sample");
pub const INPUT: &str = include_str!("input");

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Id([u8; 2]);

impl From<&[u8]> for Id {
    fn from(value: &[u8]) -> Self {
        Id(value.try_into().unwrap())
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(std::str::from_utf8(&self.0).unwrap())
    }
}

impl Deref for Id {
    type Target = [u8; 2];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Id {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
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

            let id: Id = id.as_bytes().into();
            let (flow_rate, l) = l
                .split_once("; tunnels lead to valves ")
                .unwrap_or_else(|| l.split_once("; tunnel leads to valve ").unwrap());
            (
                id,
                Valve {
                    flow_rate: flow_rate.parse().unwrap(),
                    direct_connections: l.split(", ").map(|v| v.as_bytes().into()).collect(),
                    distances: HashMap::new(),
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
    for _ in 0..100 {
        if args.is_empty() {
            return valves;
        }
        args = args
            .into_iter()
            .flat_map(|a| step(a, &mut valves))
            .collect();
    }
    panic!("Too many steps");
}

const N_STEPS: u32 = 30;
const START_ID: Id = Id([b'A', b'A']);

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

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> u32 {
        let mut valves = parse(s);
        valves.retain(|k, v| *k == START_ID || v.flow_rate > 0);

        let mut args = vec![NextArgs {
            current: START_ID,
            unvisited: valves
                .clone()
                .into_iter()
                .map(|(k, _)| k)
                .filter(|k| *k != START_ID)
                .collect(),
            steps_left: N_STEPS,
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
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 1651);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 1880);
    }
}
