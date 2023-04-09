use either::Either;
use std::iter;

pub const SAMPLE: &str = include_str!("sample");

pub const INPUT: &str = include_str!("input");

fn parse(s: &str) -> (FieldIdx, FieldIdx, Map) {
    let n_cols = s.find('\n').unwrap();
    let mut start = None;
    let mut end = None;
    let fields: Vec<_> = s
        .bytes()
        .filter(|&b| b != b'\n')
        .enumerate()
        .map(|(i, b)| match b {
            b'S' => {
                assert!(start.replace(i).is_none());
                b'a'
            }
            b'E' => {
                assert!(end.replace(i).is_none());
                b'z'
            }
            x => x,
        })
        .map(|b| match b {
            b'a'..=b'z' => b - 97,
            _ => panic!("Couldn't parse {b}"),
        })
        .map(|height| Field {
            height,
            distance: usize::MAX,
        })
        .collect();
    let n_rows = fields.len() / n_cols;
    assert_eq!(fields.len(), n_cols * n_rows);
    let map = Map {
        fields,
        n_rows,
        n_cols,
    };
    let start = map.field_from_idx(start.unwrap());
    let end = map.field_from_idx(end.unwrap());
    (start, end, map)
}

#[derive(Debug, Clone)]
struct FieldIdx {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Field {
    height: u8,
    distance: usize,
}

struct Map {
    fields: Vec<Field>,
    n_rows: usize,
    n_cols: usize,
}

impl Map {
    fn at(&self, f: &FieldIdx) -> &Field {
        &self.fields[self.idx_from_field(f)]
    }

    fn mut_at(&mut self, f: &FieldIdx) -> &mut Field {
        let idx = self.idx_from_field(f);
        &mut self.fields[idx]
    }

    fn adjacent_to(&self, &FieldIdx { row, col }: &FieldIdx) -> Vec<FieldIdx> {
        let mut result = Vec::new();
        if col <= self.n_cols - 2 {
            result.push(FieldIdx { row, col: col + 1 })
        };
        if row <= self.n_rows - 2 {
            result.push(FieldIdx { row: row + 1, col })
        };
        if row >= 1 {
            result.push(FieldIdx { row: row - 1, col })
        };
        if col >= 1 {
            result.push(FieldIdx { row, col: col - 1 })
        };
        result
    }

    fn field_from_idx(&self, i: usize) -> FieldIdx {
        FieldIdx {
            row: i / self.n_cols,
            col: i % self.n_cols,
        }
    }

    fn idx_from_field(&self, FieldIdx { row, col }: &FieldIdx) -> usize {
        row * self.n_cols + col
    }
}

struct StepArguments {
    field_idx: FieldIdx,
    distance: usize,
    last_height: u8,
}

impl Map {
    fn step(
        &mut self,
        StepArguments {
            field_idx,
            distance,
            last_height,
        }: StepArguments,
        height_predicate: impl FnOnce(u8, u8) -> bool + Copy + 'static,
    ) -> impl Iterator<Item = StepArguments> {
        if self.at(&field_idx).distance > distance
            && height_predicate(last_height, self.at(&field_idx).height)
        {
            self.mut_at(&field_idx).distance = distance;
            let height = self.at(&field_idx).height;
            Either::Left(
                self.adjacent_to(&field_idx)
                    .into_iter()
                    .map(move |ff| StepArguments {
                        field_idx: ff,
                        distance: distance + 1,
                        last_height: height,
                    }),
            )
        } else {
            Either::Right(iter::empty())
        }
    }

    fn next_depth(
        &mut self,
        args: Vec<StepArguments>,
        height_predicate: impl FnOnce(u8, u8) -> bool + Copy + 'static,
    ) {
        if !args.is_empty() {
            let next_args: Vec<_> = args
                .into_iter()
                .flat_map(|tt| self.step(tt, height_predicate))
                .collect();
            self.next_depth(next_args, height_predicate);
        }
    }

    /// Expects distances to be initialized to u8::MAX
    fn calculate_distances(
        &mut self,
        start: FieldIdx,
        height_predicate: impl FnOnce(u8, u8) -> bool + Copy + 'static,
    ) {
        let start_height = self.at(&start).height;
        let args = vec![StepArguments {
            field_idx: start,
            distance: 0,
            last_height: start_height,
        }];
        self.next_depth(args, height_predicate);
    }
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> usize {
        let (start, end, mut map) = parse(s);
        map.calculate_distances(start, |h0, h1| h1 <= h0 + 1);
        map.at(&end).distance
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 31);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 330);
    }
}

pub mod part2 {
    use super::*;

    pub fn solution(s: &str) -> usize {
        let (_start, end, mut map) = parse(s);
        map.calculate_distances(end, |h0, h1| h1 + 1 >= h0);
        map.fields
            .into_iter()
            .filter(|&Field { height, .. }| height == 0)
            .map(|Field { distance, .. }| distance)
            .min()
            .unwrap()
    }
    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 29);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 321);
    }
}
