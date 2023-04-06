use std::collections::HashSet;
use std::ops::{Add, AddAssign, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Point {
    fn from_op(op: &str) -> Point {
        match op {
            "R" => Point { x: 1, y: 0 },
            "L" => Point { x: -1, y: 0 },
            "D" => Point { x: 0, y: -1 },
            "U" => Point { x: 0, y: 1 },
            _ => panic!("Unkown op `{op}`"),
        }
    }
}

fn parse_operations(s: &str) -> impl Iterator<Item = Point> + '_ {
    s.lines().flat_map(|line| {
        let (op, count) = line.split_once(' ').unwrap();
        std::iter::repeat(Point::from_op(op)).take(count.parse().unwrap())
    })
}

impl Point {
    fn follow(&mut self, other: &Point) {
        let Point { x, y } = *other - *self;
        if x.abs() > 1 || y.abs() > 1 {
            *self += Point {
                x: x.signum(),
                y: y.signum(),
            }
        }
    }
}

mod part1 {
    use super::*;

    fn solution(s: &str) -> usize {
        let start = Point { x: 0, y: 0 };
        let mut head = start;
        let mut tail = start;
        parse_operations(s)
            .map(|op| {
                head += op;
                tail.follow(&head);
                tail
            })
            .chain(std::iter::once(start))
            .collect::<HashSet<Point>>()
            .len()
    }

    const SAMPLE: &'static str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 13);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(include_str!("input")), 6212);
    }
}

mod part2 {
    use super::*;
    fn solution(s: &str) -> usize {
        let start = Point { x: 0, y: 0 };
        let mut knots = [start; 10];
        parse_operations(s)
            .map(|op| {
                *knots.first_mut().unwrap() += op;
                let mut last_knot = *knots.first().unwrap();
                knots.iter_mut().skip(1).for_each(|k| {
                    k.follow(&last_knot);
                    last_knot = *k;
                });
                *knots.last().unwrap() // tail
            })
            .chain(std::iter::once(start))
            .collect::<HashSet<Point>>()
            .len()
    }

    const SAMPLE: &'static str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 36);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(include_str!("input")), 2522);
    }
}
