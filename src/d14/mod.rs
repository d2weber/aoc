use std::cmp::{max, min};

pub const SAMPLE: &str = include_str!("sample");

pub const INPUT: &str = include_str!("input");

fn iterate_shapes(s: &str) -> impl Iterator<Item = impl Iterator<Item = Point> + '_> {
    s.lines().map(|l| {
        l.split(" -> ").map(|p| {
            let (x, y) = p.split_once(',').unwrap();
            Point {
                x: x.parse::<usize>().unwrap(),
                y: y.parse::<usize>().unwrap(),
            }
        })
    })
}

fn incr_range(v0: usize, v1: usize) -> impl Iterator<Item = usize> {
    let from = min(v0, v1);
    let to = max(v0, v1);
    from..=to
}

fn parse(s: &str) -> World {
    let max_y_index = iterate_shapes(s)
        .flatten()
        .map(|Point { y, .. }| y)
        .max()
        .unwrap();
    let size_y = max_y_index + 1 + Y_MARGIN;
    let size_x = 700;
    let mut world = World {
        fields: vec![Field::Free(); size_y * size_x],
        size_y,
        size_x,
    };
    iterate_shapes(s).for_each(|mut s| {
        let mut p0 = s.next().unwrap();
        s.for_each(|p1| {
            if p0.x == p1.x {
                incr_range(p0.y, p1.y).for_each(|y| world.make_solid(&Point { x: p0.x, y }))
            } else {
                assert_eq!(p0.y, p1.y);
                incr_range(p0.x, p1.x).for_each(|x| world.make_solid(&Point { x, y: p0.y }))
            }
            p0 = p1;
        })
    });
    world
}

#[derive(Clone, PartialEq)]
enum Field {
    Solid(),
    Free(),
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn center(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y + 1,
        }
    }
    fn right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

struct World {
    fields: Vec<Field>,
    size_y: usize,
    size_x: usize,
}

const SAND_SOURCE: Point = Point { x: 500, y: 0 };
const Y_MARGIN: usize = 2;

impl World {
    fn point_to_idx(&self, &Point { x, y }: &Point) -> usize {
        (x) * self.size_y + y
    }
    fn at(&self, p: &Point) -> &Field {
        let idx = self.point_to_idx(p);
        &self.fields[idx]
    }
    fn mut_at(&mut self, p: &Point) -> &mut Field {
        let idx = self.point_to_idx(p);
        &mut self.fields[idx]
    }
    fn make_solid(&mut self, p: &Point) {
        *self.mut_at(p) = Field::Solid();
    }
    fn is_free(&self, p: Point) -> Option<Point> {
        Some(p).filter(|pp| *self.at(pp) == Field::Free())
    }
}

impl World {
    fn spawn_sand(&mut self) -> Result<(), &str> {
        let mut p = SAND_SOURCE;
        while p.y < self.size_y {
            if let Some(next_p) = self.is_free(p.center()) {
                p = next_p;
            } else if let Some(next_p) = self.is_free(p.left()) {
                p = next_p;
            } else if let Some(next_p) = self.is_free(p.right()) {
                p = next_p;
            } else if let Some(pp) = self.is_free(p) {
                self.make_solid(&pp);
                return Ok(());
            } else {
                return Err("Source covered");
            }
        }
        Err("Out of world")
    }

    fn count_spawned(&mut self) -> usize {
        let mut count = 0;
        while self.spawn_sand().is_ok() {
            count += 1
        }
        count
    }
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> usize {
        let mut world = parse(s);
        world.count_spawned()
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 24);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 737);
    }
}

pub mod part2 {
    use super::*;

    pub fn solution(s: &str) -> usize {
        let mut world = parse(s);
        (0..world.size_x).for_each(|x| {
            world.make_solid(&Point {
                x,
                y: world.size_y - 1,
            })
        });
        world.count_spawned()
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 93);
    }
    // This test seems to be slow only in debug mode
    // #[test]
    // fn actual() {
    //     assert_eq!(solution(INPUT), 28145);
    // }
}
