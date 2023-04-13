use itertools::Itertools;
use std::{collections::HashMap, fmt::Debug};

pub const SAMPLE: &str = include_str!("sample");
pub const INPUT: &str = include_str!("input");

type Id = [u8; 2];

type DirectConnections = HashMap<Id, Vec<Id>>;

#[derive(Clone, Hash, Debug)]
struct Valve {
    id: Id,
    flow_rate: u32,
    // distances: HashMap<ValveId, u32>
}

#[derive(Debug)]
struct Distances {
    d: HashMap<Id, HashMap<Id, u32>>,
    direct_connections: DirectConnections,
}

#[derive(Debug)]
struct StepArgs {
    start: Id,
    current: Id,
    distance: u32,
}

impl Distances {
    fn step(
        &mut self,
        StepArgs {
            start,
            current,
            distance,
        }: StepArgs,
    ) -> Vec<StepArgs> {
        let ds = &mut self.d.get_mut(&start).unwrap();
        if *ds.get(&current).unwrap_or(&u32::MAX) > distance {
            ds.insert(current, distance);
            self.direct_connections[&current]
                .iter()
                .map(|id| StepArgs {
                    start,
                    current: *id,
                    distance: distance + 1,
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn new<'a>(
        direct_connections: DirectConnections,
        ids: impl Iterator<Item = &'a Id> + 'a,
    ) -> Distances {
        let mut args: Vec<StepArgs> = direct_connections
            .iter()
            .flat_map(|(start, conns)| {
                conns.iter().map(|dst| StepArgs {
                    start: *start,
                    current: *dst,
                    distance: 1,
                })
            })
            .collect();
        let mut distances = Distances {
            d: direct_connections
                .keys()
                .map(|k| (*k, HashMap::new()))
                .collect(),
            direct_connections,
        };
        while !args.is_empty() {
            args = args.into_iter().flat_map(|a| distances.step(a)).collect();
        }
        // dbg!(&distances);
        distances
    }
}

fn parse(s: &str) -> (Vec<Valve>, Distances) {
    let (mut valves, direct_connections): (Vec<Valve>, HashMap<Id, Vec<Id>>) = s
        .lines()
        .map(|l| {
            let (id, l) = l
                .strip_prefix("Valve ")
                .unwrap()
                .split_once(" has flow rate=")
                .unwrap();
            let id: Id = id.as_bytes().try_into().unwrap();
            let (flow_rate, l) = l
                .split_once("; tunnels lead to valves ")
                .unwrap_or_else(|| l.split_once("; tunnel leads to valve ").unwrap());
            let valve = Valve {
                id,
                flow_rate: flow_rate.parse().unwrap(),
            };
            let connections = l
                .split(", ")
                .map(|v| v.as_bytes().try_into().unwrap())
                .collect();
            (valve, (id, connections))
        })
        .unzip();
    valves.retain(|v| v.flow_rate > 0);
    let distances = Distances::new(direct_connections, valves.iter().map(|v| &v.id));
    (valves, distances)
}

const N_STEPS: u32 = 30;

pub mod part1 {
    use super::*;

    pub fn slow_solution(s: &str) -> u32 {
        let (valves, distances) = parse(s);
        let k = valves.len();
        valves
            .into_iter()
            .permutations(k)
            .map(|x| {
                // dbg! {&x};
                let mut last_valve = [b'A', b'A'];
                x.iter()
                    .scan(N_STEPS, |steps_left, valve| {
                        if let Some(next_steps_left) = steps_left.checked_sub(
                            distances.d[&last_valve][&valve.id] /* time to walk */
                         + 1, /* time to open the valve */
                        ) {
                            *steps_left = next_steps_left;
                        } else {
                            return None;
                        };
                        last_valve = valve.id;
                        // dbg!(valve.flow_rate, &steps_left);
                        Some(valve.flow_rate * (*steps_left))
                    })
                    .sum()
            })
            // .count() as u32
            .max()
            .unwrap()
    }
    pub fn solution(s: &str) -> u32 {
        let (valves, distances) = parse(s);
        // let mut vs = vec!["AA".to_owned()];
        // let mut unused = valves;
        let k = valves.len();
        valves
            .into_iter()
            .permutations(k)
            .map(|x| {
                // dbg! {&x};
                let mut last_valve = [b'A', b'A'];
                x.iter()
                    .scan(N_STEPS, |steps_left, valve| {
                        if let Some(next_steps_left) = steps_left.checked_sub(
                            distances.d[&last_valve][&valve.id] /* time to walk */
                     + 1, /* time to open the valve */
                        ) {
                            *steps_left = next_steps_left;
                        } else {
                            return None;
                        };
                        last_valve = valve.id;
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
