pub const SAMPLE: &str = include_str!("sample");

pub const INPUT: &str = include_str!("input");

fn parse(s: &str) -> (Map, FieldIdx, FieldIdx) {
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

    (map, start, end)
}

#[derive(Debug)]
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
    fn field_from_idx(&self, i: usize) -> FieldIdx {
        assert!(i < self.fields.len());
        FieldIdx {
            row: i / self.n_cols,
            col: i % self.n_cols,
        }
    }

    fn idx_from_field(&self, FieldIdx { row, col }: &FieldIdx) -> usize {
        row * self.n_cols + col
    }

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
}

struct StepArguments {
    f: FieldIdx,
    distance: usize,
    last_height: u8,
}

impl Map {
    fn step(
        &mut self,
        StepArguments {
            f,
            distance,
            last_height,
        }: StepArguments,
        height_predicate: impl FnOnce(u8, u8) -> bool + Copy + 'static,
    ) -> Vec<StepArguments> {
        if self.at(&f).distance > distance && height_predicate(last_height, self.at(&f).height) {
            self.mut_at(&f).distance = distance;
            self.adjacent_to(&f)
                .into_iter()
                .map(move |ff| StepArguments {
                    f: ff,
                    distance: distance + 1,
                    last_height: self.at(&f).height,
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> usize {
        let (mut map, start, end) = parse(s);
        let start_height = map.at(&start).height;
        let mut args = vec![StepArguments {
            f: start,
            distance: 0,
            last_height: start_height,
        }];
        while !args.is_empty() {
            args = args
                .into_iter()
                .flat_map(|tt| map.step(tt, |h0, h1| h1 <= h0 + 1).into_iter())
                .collect();
        }
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
        let (mut map, _start, end) = parse(s);
        let init_height = map.at(&end).height;
        let mut args = vec![StepArguments {
            f: end,
            distance: 0,
            last_height: init_height,
        }];
        while !args.is_empty() {
            args = args
                .into_iter()
                .flat_map(|tt| map.step(tt, |h0, h1| h1 + 1 >= h0).into_iter())
                .collect();
        }
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
