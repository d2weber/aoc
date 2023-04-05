use std::ops::{Deref, DerefMut};
use std::str::FromStr;

const SAMPLE_INPUT: &'static str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

type Stack = Vec<u8>;

struct Stacks(Vec<Stack>);

impl FromStr for Stacks {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines_it = s.lines().rev();
        let header = lines_it.next().unwrap();

        let parse_offsets: Vec<_> = header
            .match_indices(char::is_numeric)
            .map(|(idx, _)| idx)
            .collect();

        let mut stacks = vec![Stack::new(); parse_offsets.len()];
        lines_it.for_each(|line| {
            stacks
                .iter_mut()
                .zip(&parse_offsets)
                .for_each(
                    |(stack, parse_offset)| match line.as_bytes()[*parse_offset] {
                        b' ' => (),
                        v @ b'A'..=b'Z' => stack.push(v),
                        v @ _ => panic!("Couldn't parse crate: `{v}`"),
                    },
                );
        });
        Ok(Stacks(stacks))
    }
}

impl Deref for Stacks {
    type Target = Vec<Stack>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stacks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Stacks {
    fn top_crates(&self) -> String {
        self.iter()
            .map(
                |stack| *stack.last().unwrap() as char, /*Char cast could waste memroy*/
            )
            .collect()
    }
}

struct Operation {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s
            .splitn(6, ' ')
            .filter(|s| !(s == &"move" || s == &"from" || s == &"to"))
            .map(|s| s.parse::<usize>().unwrap());
        Ok(Operation {
            count: it.next().unwrap(),
            from: it.next().unwrap() - 1,
            to: it.next().unwrap() - 1,
        })
    }
}

fn parse_input(input: &str) -> (Stacks, impl Iterator<Item = Operation> + '_) {
    assert_eq!(input.bytes().count(), input.len());
    let (stacks, procedure) = input.split_once("\n\n").unwrap();
    let stacks: Stacks = stacks.parse().unwrap();

    let procedure = procedure
        .lines()
        .map(|line| line.parse::<Operation>().unwrap());
    (stacks, procedure)
}

mod part1 {
    use super::*;

    fn solution(input: &str) -> String {
        let (mut stacks, procedure) = parse_input(input);
        for Operation { count, from, to } in procedure {
            let split_idx = stacks[from].len() - count;
            let crates = stacks[from].split_off(split_idx);
            stacks[to].extend(crates.iter().rev())
        }
        stacks.top_crates()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE_INPUT), "CMZ");
    }
    #[test]
    fn actual() {
        assert_eq!(solution(include_str!("input")), "FWSHSPJWM");
    }
}

mod part2 {
    use super::*;

    fn solution(input: &str) -> String {
        let (mut stacks, procedure) = parse_input(input);
        for Operation { count, from, to } in procedure {
            let split_idx = stacks[from].len() - count;
            let crates = stacks[from].split_off(split_idx);
            stacks[to].extend(crates.iter())
        }
        stacks.top_crates()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE_INPUT), "MCD");
    }
    #[test]
    fn actual() {
        assert_eq!(solution(include_str!("input")), "PWPWHGFZS");
    }
}
