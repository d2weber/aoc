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
    fn is_free(&self, p: &Point) -> bool {
        *self.at(p) == Field::Free()
    }
}

impl World {
    fn spawn_sand(&mut self) -> Result<(), &str> {
        if *self.at(&SAND_SOURCE) == Field::Solid() {
            return Err("Source covered");
        }
        let mut x = SAND_SOURCE.x;
        for y in (SAND_SOURCE.y + 1)..self.size_y {
            if self.is_free(&Point { x, y }) {
                continue;
            } else if self.is_free(&Point { x: x - 1, y }) {
                x -= 1;
            } else if self.is_free(&Point { x: x + 1, y }) {
                x += 1;
            } else {
                // Make last step solid
                self.make_solid(&Point { x, y: y - 1 });
                return Ok(());
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
    #[test]
    #[ignore = "slow when unoptimized"]
    fn actual() {
        assert_eq!(solution(INPUT), 28145);
    }
}
