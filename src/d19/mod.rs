use std::ops::{Add, AddAssign, Sub};

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

impl Sub for Resources {
    type Output = Resources;

    fn sub(self, rhs: Self) -> Self::Output {
        Resources {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
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

impl AddAssign for Resources {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
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

fn max_geodes(
    bp: &Blueprint,
    mut inventory: Resources,
    n_robots: Resources,
    minutes_left: u32,
    mut skip_ore: bool,
    mut skip_clay: bool,
) -> u32 {
    if minutes_left == 1 {
        return inventory.geode + n_robots.geode;
    }
    let mut geodes = 0;
    let new_robot = if let Some(inven) = inventory.checked_sub(&bp.geode_robot) {
        inventory = inven;
        skip_clay = false;
        skip_ore = false;
        Resources {
            geode: 1,
            ..Default::default()
        }
    } else if let Some(inven) = inventory.checked_sub(&bp.obsidian_robot) {
        inventory = inven;
        skip_clay = false;
        skip_ore = false;
        Resources {
            obsidian: 1,
            ..Default::default()
        }
    } else {
        if !skip_clay {
            if let Some(inven) = inventory.checked_sub(&bp.clay_robot) {
                skip_ore = false;
                geodes = geodes.max(max_geodes(
                    bp,
                    inven + n_robots.clone(),
                    n_robots.clone()
                        + Resources {
                            clay: 1,
                            ..Default::default()
                        },
                    minutes_left - 1,
                    skip_ore,
                    skip_clay,
                ));
                skip_clay = true;
            }
        }
        if !skip_ore {
            if let Some(inven) = inventory.checked_sub(&bp.ore_robot) {
                skip_clay = false;
                geodes = geodes.max(max_geodes(
                    bp,
                    inven + n_robots.clone(),
                    n_robots.clone()
                        + Resources {
                            ore: 1,
                            ..Default::default()
                        },
                    minutes_left - 1,
                    skip_ore,
                    skip_clay,
                ));
                skip_ore = true;
            }
        }
        Resources::default()
    };
    geodes.max(max_geodes(
        bp,
        inventory + n_robots.clone(),
        n_robots + new_robot,
        minutes_left - 1,
        skip_ore,
        skip_clay,
    ))
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> u32 {
        let bps = parse(s.lines());
        bps.iter()
            .map(|bp| {
                bp.n * max_geodes(
                    bp,
                    Resources::default(),
                    Resources {
                        ore: 1,
                        ..Default::default()
                    },
                    24,
                    false,
                    false,
                )
            })
            .inspect(|q| {
                dbg!(q);
            })
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
            .map(|bp| {
                max_geodes(
                    bp,
                    Resources::default(),
                    Resources {
                        ore: 1,
                        ..Default::default()
                    },
                    32,
                    false,
                    false,
                )
            })
            .inspect(|q| {
                dbg!(q);
            })
            .product()
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 33);
    }
    // #[test]
    // fn actual() {
    //     assert_eq!(solution(INPUT), 1382);
    // }
}
