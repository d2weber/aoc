use std::ops::Add;

pub const SAMPLE: &str = include_str!("sample");
pub const INPUT: &str = include_str!("input");

#[derive(Debug)]
struct Blueprint {
    n: u32,
    ore_robot: Resources,
    clay_robot: Resources,
    obsidian_robot: Resources,
    geode_robot: Resources,
}

#[derive(Clone, Debug, Default)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Resources {
    fn checked_sub(&self, other: &Self) -> Option<Self> {
        Some(Resources {
            ore: self.ore.checked_sub(other.ore)?,
            clay: self.clay.checked_sub(other.clay)?,
            obsidian: self.obsidian.checked_sub(other.obsidian)?,
            geode: self.geode.checked_sub(other.geode)?,
        })
    }
}

impl Add for Resources {
    type Output = Resources;

    fn add(self, rhs: Self) -> Self::Output {
        Resources {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Blueprint> {
    lines
        .map(|bp| {
            let (n, s) = bp
                .strip_prefix("Blueprint ")
                .unwrap()
                .split_once(": Each ore robot costs ")
                .unwrap();
            let n = n.parse().unwrap();
            let (ore_robot_ore, s) = s.split_once(" ore. Each clay robot costs ").unwrap();
            let (clay_robot_ore, s) = s.split_once(" ore. Each obsidian robot costs ").unwrap();
            let (obsidian_robot_ore, s) = s.split_once(" ore and ").unwrap();
            let (obsidian_robot_clay, s) = s.split_once(" clay. Each geode robot costs ").unwrap();
            let (geode_robot_ore, s) = s.split_once(" ore and ").unwrap();
            let geode_robot_obsidian = s.strip_suffix(" obsidian.").unwrap();
            Blueprint {
                n,
                ore_robot: Resources {
                    ore: ore_robot_ore.parse().unwrap(),
                    ..Default::default()
                },
                clay_robot: Resources {
                    ore: clay_robot_ore.parse().unwrap(),
                    ..Default::default()
                },
                obsidian_robot: Resources {
                    ore: obsidian_robot_ore.parse().unwrap(),
                    clay: obsidian_robot_clay.parse().unwrap(),
                    ..Default::default()
                },
                geode_robot: Resources {
                    ore: geode_robot_ore.parse().unwrap(),
                    obsidian: geode_robot_obsidian.parse().unwrap(),
                    ..Default::default()
                },
            }
        })
        .collect()
}

#[derive(Clone)]
struct State {
    inventory: Resources,
    n_robots: Resources,
}

impl State {
    fn new() -> Self {
        Self {
            inventory: Default::default(),
            n_robots: Resources {
                ore: 1,
                ..Default::default()
            },
        }
    }
}

fn max_geodes(
    bp: &Blueprint,
    State {
        inventory,
        n_robots,
    }: State,
    minutes_left: u32,
    skip_clay: bool,
    skip_ore: bool,
) -> u32 {
    if minutes_left == 1 {
        return inventory.geode + n_robots.geode;
    }
    if let Some(g) = inventory.checked_sub(&bp.geode_robot).map(|inven| {
        max_geodes(
            bp,
            State {
                inventory: inven + n_robots.clone(),
                n_robots: n_robots.clone()
                    + Resources {
                        geode: 1,
                        ..Default::default()
                    },
            },
            minutes_left - 1,
            false,
            false,
        )
    }) {
        return g;
    }

    let buy_obsidian = inventory
        .checked_sub(&bp.obsidian_robot)
        .filter(|_| minutes_left > 3)
        .map(|inven| {
            max_geodes(
                bp,
                State {
                    inventory: inven + n_robots.clone(),
                    n_robots: n_robots.clone()
                        + Resources {
                            obsidian: 1,
                            ..Default::default()
                        },
                },
                minutes_left - 1,
                false,
                false,
            )
        });

    let buy_clay = inventory
        .checked_sub(&bp.clay_robot)
        .filter(
            |_| !skip_clay && minutes_left > 5, /*improbable to improve when buying late*/
        )
        .map(|inven| {
            max_geodes(
                bp,
                State {
                    inventory: inven + n_robots.clone(),
                    n_robots: n_robots.clone()
                        + Resources {
                            clay: 1,
                            ..Default::default()
                        },
                },
                minutes_left - 1,
                false,
                false,
            )
        });

    if let Some(buy_obsidian_geodes) = buy_obsidian {
        return buy_obsidian_geodes.max(buy_clay.unwrap_or(0));
    }

    let buy_ore = inventory
        .checked_sub(&bp.ore_robot)
        .filter(
            |_| !skip_ore && minutes_left > 9, /*improbable to improve when buying late*/
        )
        .map(|inven| {
            max_geodes(
                bp,
                State {
                    inventory: inven + n_robots.clone(),
                    n_robots: n_robots.clone()
                        + Resources {
                            ore: 1,
                            ..Default::default()
                        },
                },
                minutes_left - 1,
                false,
                false,
            )
        });

    buy_clay
        .unwrap_or(0)
        .max(buy_ore.unwrap_or(0))
        .max(max_geodes(
            bp,
            State {
                inventory: inventory + n_robots.clone(),
                n_robots,
            },
            minutes_left - 1,
            buy_clay.is_some(),
            buy_ore.is_some(),
        ))
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> u32 {
        let bps = parse(s.lines());
        bps.iter()
            .map(|bp| bp.n * max_geodes(bp, State::new(), 24, false, false))
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
            .map(|bp| max_geodes(bp, State::new(), 32, false, false))
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
