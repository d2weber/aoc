use std::ops::{RangeInclusive, Sub};

pub const SAMPLE: &str = include_str!("sample");

pub const INPUT: &str = include_str!("input");

#[derive(Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Sub<&Point> for &Point {
    type Output = u32;

    fn sub(self, rhs: &Point) -> Self::Output {
        self.x.abs_diff(rhs.x) + self.y.abs_diff(rhs.y)
    }
}

struct Sensor {
    pos: Point,
    range: u32, // within which no other beacon can be
}

#[derive(PartialEq)]
struct Beacon(Point);

fn parse(s: &str) -> (Sensor, Beacon) {
    let (x, s) = s
        .strip_prefix("Sensor at x=")
        .unwrap()
        .split_once(", y=")
        .unwrap();
    let (y, s) = s.split_once(": closest beacon is at x=").unwrap();
    let pos = Point {
        x: x.parse().unwrap(),
        y: y.parse().unwrap(),
    };
    let (x, y) = s.split_once(", y=").unwrap();
    let beacon = Point {
        x: x.parse().unwrap(),
        y: y.parse().unwrap(),
    };
    let range = &pos - &beacon;
    (Sensor { pos, range }, Beacon(beacon))
}

impl Sensor {
    fn x_idxs(&self, y: i32) -> Option<RangeInclusive<i32>> {
        let distance = self.pos.y.abs_diff(y);
        let overlap = self.range as i32 - distance as i32;
        if overlap < 0 {
            None
        } else {
            Some((self.pos.x - overlap)..=(self.pos.x + overlap))
        }
    }

    fn is_in_range(&self, p: &Point) -> bool {
        (&self.pos - p) <= self.range
    }
}

pub mod part1 {
    use super::*;

    // use std::collections::HashSet;
    // pub fn solution(s: &str, y_inspect: i32) -> usize {
    //     let (sensors, beacons): (Vec<Sensor>, Vec<Beacon>) = s.lines().map(|l| parse(l)).unzip();
    //     let mut s = sensors
    //         .into_iter()
    //         .flat_map(|s| s.x_idxs(y_inspect))
    //         .collect::<HashSet<_>>();
    //     beacons
    //         .into_iter()
    //         .map(|Beacon(Point { x, y })| (x, y))
    //         .filter(|(_, y)| *y == y_inspect)
    //         .for_each(|(x, _)| {
    //             s.remove(&x);
    //         });
    //     s.len()
    // }
    pub fn solution(s: &str, y_inspect: i32) -> i32 {
        let (sensors, _): (Vec<Sensor>, Vec<Beacon>) = s.lines().map(parse).unzip();
        let mut ranges: Vec<_> = sensors.iter().filter_map(|s| s.x_idxs(y_inspect)).collect();
        ranges.sort_unstable_by_key(|r| *r.start());
        let mut ranges = ranges.into_iter();
        let mut merged_r = ranges.next().unwrap();
        ranges.for_each(|r| {
            assert!(*r.start() <= merged_r.end() + 1);
            let end = std::cmp::max(r.end(), merged_r.end());
            merged_r = *merged_r.start()..=*end;
        });
        merged_r.end() - merged_r.start()
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE, 10), 26);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT, 2000000), 4665948);
    }
}

pub mod part2 {
    use super::*;
    pub fn solution(s: &str, max_coord: i32) -> i64 {
        let (sensors, _): (Vec<Sensor>, Vec<Beacon>) = s.lines().map(parse).unzip();
        (0..=max_coord)
            .find_map(|y| {
                let mut ranges: Vec<_> = sensors
                    .iter()
                    .filter_map(|s| s.x_idxs(y))
                    .filter(|r| *r.end() + 1 > 0 || *r.start() <= y)
                    .collect();
                ranges.sort_unstable_by_key(|r| *r.start());
                let mut last_end = i32::MIN;
                ranges.iter().find_map(|r| {
                    if *r.start() == last_end + 2 {
                        let x = last_end as i64 + 1;
                        Some(x * 4000000 + y as i64)
                    } else {
                        last_end = std::cmp::max(*r.end(), last_end);
                        None
                    }
                })
            })
            .unwrap()
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE, 20), 56000011);
    }
    #[test]
    // #[ignore = "slow"]
    fn actual() {
        assert_eq!(solution(INPUT, 4000000), 13543690671045);
    }
}
