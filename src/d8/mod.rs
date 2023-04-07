use std::str::FromStr;

struct Grid<T> {
    values: Vec<T>,
    n_cols: usize,
    n_rows: usize,
}

impl FromStr for Grid<u8> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n_cols = s.find('\n').unwrap();
        let values: Vec<_> = s
            .matches(|c| c != '\n')
            .map(|c| u8::from_str(c).unwrap())
            .collect();
        let n_rows = values.len() / n_cols;

        Ok(Grid {
            values,
            n_cols,
            n_rows,
        })
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    col: usize,
    row: usize,
    dir: Direction,
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.dir {
            Direction::Up => {
                if self.row == 0 {
                    return None;
                }
                self.row -= 1;
            }
            Direction::Down => {
                if self.row == self.grid.n_rows - 1 {
                    return None;
                }
                self.row += 1;
            }
            Direction::Left => {
                if self.col == 0 {
                    return None;
                }
                self.col -= 1;
            }
            Direction::Right => {
                if self.col == self.grid.n_cols - 1 {
                    return None;
                }
                self.col += 1;
            }
        }
        Some(self.grid.at((self.row, self.col)))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let l = match self.dir {
            Direction::Up => self.row,
            Direction::Down => self.grid.n_rows - self.row - 1,
            Direction::Left => self.col,
            Direction::Right => self.grid.n_cols - self.col - 1,
        };
        (l, Some(l))
    }
}

impl<'a, T> ExactSizeIterator for GridIterator<'a, T> {}

impl<T> Grid<T> {
    fn row_col(&self, idx: usize) -> (usize, usize) {
        (idx / self.n_cols, idx % self.n_cols)
    }
    fn iter_row_col_val(&self) -> impl Iterator<Item = ((usize, usize), &T)> + '_ {
        self.values
            .iter()
            .enumerate()
            .map(|(i, v)| (self.row_col(i), v))
    }
    fn at(&self, (row, col): (usize, usize)) -> &T {
        &self.values[row * self.n_cols + col]
    }
    fn iter_from(&self, (row, col): (usize, usize), dir: Direction) -> GridIterator<'_, T> {
        GridIterator {
            grid: self,
            col,
            row,
            dir,
        }
    }
    fn cross(&self, (row, col): (usize, usize)) -> impl Iterator<Item = GridIterator<'_, T>> {
        [
            self.iter_from((row, col), Direction::Up),
            self.iter_from((row, col), Direction::Down),
            self.iter_from((row, col), Direction::Left),
            self.iter_from((row, col), Direction::Right),
        ]
        .into_iter()
    }
}

#[test]
fn grid() {
    let grid: Grid<u8> = SAMPLE.parse().unwrap();
    let mut cross = grid.cross((2, 2));
    assert!(cross.next().unwrap().eq([5u8, 3].iter()));
    assert!(cross.next().unwrap().eq([5u8, 3].iter()));
    assert!(cross.next().unwrap().eq([5u8, 6].iter()));
    assert!(cross.next().unwrap().eq([3u8, 2].iter()));
    assert!(cross.next().is_none());
}

pub const SAMPLE: &str = "30373
25512
65332
33549
35390";

pub const INPUT: &str = include_str!("input");

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> usize {
        let heights = s.parse::<Grid<u8>>().unwrap();
        heights
            .iter_row_col_val()
            .filter(|(row_col, h)| heights.cross(*row_col).any(|mut ray| ray.all(|o| o < h)))
            .count()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 21);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 1823);
    }
}

pub mod part2 {
    use super::*;
    pub fn solution(s: &str) -> usize {
        let heights: Grid<u8> = s.parse().unwrap();
        heights
            .iter_row_col_val()
            .map(|(row_col, h)| {
                heights
                    .cross(row_col)
                    .map(|ray|
                            // Also count the tree that stops iteration
                            // but only if there is one
                            ray.len().min(ray.take_while(|o| h > o).count() + 1))
                    .product()
            })
            .max()
            .unwrap()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 8);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 211680);
    }
}
