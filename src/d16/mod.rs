use itertools::Itertools;
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
    // distances.valves
}

const N_STEPS: u32 = 30;
const START_ID: Id = Id([b'A', b'A']);

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> u32 {
        let mut valves = parse(s);
        valves.retain(|k, v| *k == START_ID || v.flow_rate > 0);
        let k = valves.len();
        valves
            .iter()
            .filter(|(k, _)| **k != START_ID)
            .permutations(k - 1)
            .map(|mut x| {
                let mut last_k = None;
                let mut total = 0;
                x.retain(|(k, v)| {
                    if total > N_STEPS {
                        false
                    } else {
                        if let Some(lk) = last_k {
                            total += v.distances[lk];
                        }
                        last_k = Some(k);
                        true
                    }
                });
                x
            })
            .dedup()
            .map(|x| {
                // dbg! {&x};
                let mut last_valve = Id([b'A', b'A']);
                x.iter()
                    .scan(N_STEPS, |steps_left, (id, valve)| {
                        // dbg!(last_valve, id);
                        if let Some(next_steps_left) = steps_left.checked_sub(
                            valves[&last_valve].distances[*id] /* time to walk */
                     + 1, /* time to open the valve */
                        ) {
                            *steps_left = next_steps_left;
                        } else {
                            return None;
                        };
                        last_valve = **id;
                        // dbg!(valve.flow_rate, &steps_left);
                        Some(valve.flow_rate * (*steps_left))
                    })
                    .sum()
            })
            .max()
            .unwrap()
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 1651);
    }
    // #[test]
    // fn actual() {
    //     assert_eq!(solution(INPUT), 0);
    // }
}
