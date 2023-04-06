use std::str::FromStr;

const SAMPLE: &'static str = "30373
25512
65332
33549
35390";

struct Grid<T> {
    values: Vec<T>,
    n_cols: usize,
    n_rows: usize,
}

impl FromStr for Grid<u8> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n_cols = s.find('\n').unwrap();
        let heights = s
            .matches(|c| c != '\n')
            .map(|c| u8::from_str(c).unwrap())
            .collect();
        Ok(Grid::from_vec(heights, n_cols))
    }
}

trait DoubleEndedExactSizeIterator: DoubleEndedIterator + ExactSizeIterator {}
impl<T: DoubleEndedIterator + ExactSizeIterator> DoubleEndedExactSizeIterator for T {}

impl<T> Grid<T> {
    fn from_vec(values: Vec<T>, n_cols: usize) -> Grid<T> {
        let n_rows = values.len() / n_cols;
        assert_eq!(values.len(), n_rows * n_cols);

        Grid {
            values,
            n_cols,
            n_rows,
        }
    }
    fn row_col(&self, idx: usize) -> (usize, usize) {
        assert!(idx < self.n_cols * self.n_rows);
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
    fn row(&self, row: usize) -> impl DoubleEndedIterator<Item = &T> + ExactSizeIterator {
        (0..self.n_cols).map(move |col| self.at((row, col)))
    }
    fn col(&self, col: usize) -> impl DoubleEndedIterator<Item = &T> + ExactSizeIterator {
        (0..self.n_rows).map(move |row| self.at((row, col)))
    }
    fn left_of(
        &self,
        row: usize,
        col: usize,
    ) -> impl DoubleEndedIterator<Item = &T> + ExactSizeIterator {
        Box::new(self.row(row).take(col).rev())
    }
    fn right_of(
        &self,
        row: usize,
        col: usize,
    ) -> impl DoubleEndedIterator<Item = &T> + ExactSizeIterator {
        Box::new(self.row(row).skip(col).take(self.n_cols - col).skip(1))
    }
    fn above_of(
        &self,
        row: usize,
        col: usize,
    ) -> impl DoubleEndedIterator<Item = &T> + ExactSizeIterator {
        Box::new(self.col(col).take(row).rev())
    }
    fn below_of(
        &self,
        row: usize,
        col: usize,
    ) -> impl DoubleEndedIterator<Item = &T> + ExactSizeIterator {
        Box::new(self.col(col).skip(row).take(self.n_rows - row).skip(1))
    }
    fn cross_out(
        &self,
        (row, col): (usize, usize),
    ) -> impl Iterator<Item = Box<dyn DoubleEndedExactSizeIterator<Item = &T> + '_>> {
        use std::iter::once;

        once(Box::new(self.left_of(row, col)) as _)
            .chain(once(Box::new(self.right_of(row, col)) as _))
            .chain(once(Box::new(self.above_of(row, col)) as _))
            .chain(once(Box::new(self.below_of(row, col)) as _))
    }
}

#[test]
fn grid() {
    let grid: Grid<u8> = SAMPLE.parse().unwrap();
    let mut cross = grid.cross_out((2, 2));
    assert!(cross.next().unwrap().eq([5u8, 6].iter()));
    assert!(cross.next().unwrap().eq([3u8, 2].iter()));
    assert!(cross.next().unwrap().eq([5u8, 3].iter()));
    assert!(cross.next().unwrap().eq([5u8, 3].iter()));
    assert!(cross.next().is_none());
}
mod part1 {
    use super::*;

    fn solution(s: &str) -> usize {
        let heights = s.parse::<Grid<u8>>().unwrap();
        heights
            .iter_row_col_val()
            .filter(|(row_col, h)| {
                heights
                    .cross_out(*row_col)
                    .any(|mut ray| ray.all(|o| o < h))
            })
            .count()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 21);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(include_str!("input")), 1823);
    }
}

mod part2 {
    use super::*;
    fn solution(s: &str) -> usize {
        let heights: Grid<u8> = s.parse().unwrap();
        heights
            .iter_row_col_val()
            .map(|(row_col, h)| {
                heights
                    .cross_out(row_col)
                    .map(
                        |ray|  // Also count the tree that stops iteration, but only if there is one
                        ray.len().min(ray.take_while(|o| h > o).count() + 1),
                    )
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
        assert_eq!(solution(include_str!("input")), 211680);
    }
}
