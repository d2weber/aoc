use std::str::FromStr;

pub const INPUT: &str = include_str!("input.txt");

pub const SAMPLE1: &str = include_str!("sample1.txt");
pub const SAMPLE2: &str = include_str!("sample2.txt");
pub const SAMPLE3: &str = include_str!("sample3.txt");

#[derive(Debug)]
struct Map {
    fields: Vec<u8>,
    width: usize,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = None;
        Ok(Map {
            fields: s
                .lines()
                .flat_map(|l| {
                    if let Some(w) = width {
                        assert!(w == l.len());
                    } else {
                        width = Some(l.len());
                    }
                    l.bytes()
                })
                .collect(),
            width: width.unwrap(),
        })
    }
}

type Count = u32;
type Perimeter = u32;

impl Map {
    fn north(&self, field_idx: usize) -> Option<usize> {
        field_idx.checked_sub(self.width)
    }
    fn south(&self, field_idx: usize) -> Option<usize> {
        field_idx
            .checked_add(self.width)
            .filter(|i| *i < self.fields.len())
    }
    fn west(&self, field_idx: usize) -> Option<usize> {
        field_idx
            .checked_sub(1)
            .filter(|_| field_idx.rem_euclid(self.width) != 0)
    }
    fn east(&self, field_idx: usize) -> Option<usize> {
        field_idx
            .checked_add(1)
            .filter(|i| i.rem_euclid(self.width) != 0)
    }

    fn is_same(&self, field_idx: usize, other_idx: usize) -> bool {
        self.fields[field_idx] == self.fields[other_idx]
    }

    fn north_if_is_same(&self, field_idx: usize) -> Option<usize> {
        self.north(field_idx)
            .filter(|i| self.is_same(field_idx, *i))
    }
    fn south_if_is_same(&self, field_idx: usize) -> Option<usize> {
        self.south(field_idx)
            .filter(|i| self.is_same(field_idx, *i))
    }
    fn west_if_is_same(&self, field_idx: usize) -> Option<usize> {
        self.west(field_idx).filter(|i| self.is_same(field_idx, *i))
    }
    fn east_if_is_same(&self, field_idx: usize) -> Option<usize> {
        self.east(field_idx).filter(|i| self.is_same(field_idx, *i))
    }

    fn perimeter(&self, field_idx: usize) -> u32 {
        [
            self.north_if_is_same(field_idx),
            self.south_if_is_same(field_idx),
            self.west_if_is_same(field_idx),
            self.east_if_is_same(field_idx),
        ]
        .iter()
        .filter(|f| f.is_none())
        .count()
        .try_into()
        .unwrap()
    }
    fn calculate_cost(&self) -> u32 {
        let mut regions = Vec::<(Count, Perimeter, char)>::new();
        let mut field_to_region = Vec::with_capacity(self.fields.len());
        for field_idx in 0..self.fields.len() {
            let perimeter = self.perimeter(field_idx);
            match (
                self.north_if_is_same(field_idx),
                self.west_if_is_same(field_idx),
            ) {
                (None, None) => {
                    // New region
                    field_to_region.push(regions.len());
                    regions.push((1, perimeter, self.fields[field_idx] as char))
                }
                (Some(other_field_idx), None) | (None, Some(other_field_idx)) => {
                    // Add current field to region of other_field_idx
                    let region_idx = field_to_region[other_field_idx];
                    field_to_region.push(region_idx);
                    regions[region_idx].0 += 1;
                    regions[region_idx].1 += perimeter;
                }
                (Some(other_field_idx), Some(purge_field_idx)) => {
                    // Merge two regions
                    let region_idx = field_to_region[other_field_idx];
                    field_to_region.push(region_idx);
                    regions[region_idx].0 += 1;
                    regions[region_idx].1 += perimeter;

                    let purge_region_idx = field_to_region[purge_field_idx];
                    if region_idx != purge_region_idx {
                        field_to_region
                            .iter_mut()
                            .filter(|idx| **idx == purge_region_idx)
                            .for_each(|idx| *idx = region_idx);
                        regions[region_idx].0 += regions[purge_region_idx].0;
                        regions[region_idx].1 += regions[purge_region_idx].1;
                        regions[purge_region_idx].0 = 0;
                        regions[purge_region_idx].1 = 0;
                    }
                }
            }
        }
        regions
            .iter()
            .map(|(c, p, ch)| {
                dbg!(ch);
                dbg!(c * p)
            })
            .sum()
    }
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> u32 {
        let map = Map::from_str(s).unwrap();
        map.calculate_cost()
    }

    #[test]
    fn sample1() {
        assert_eq!(solution(SAMPLE1), 140);
    }
    #[test]
    fn sample2() {
        assert_eq!(solution(SAMPLE2), 772);
    }
    #[test]
    fn sample3() {
        assert_eq!(solution(SAMPLE3), 1930);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 0);
    }
}
