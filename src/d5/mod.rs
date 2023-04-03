#![cfg(test)]

use std::ops::{Deref, DerefMut};
use std::str;

const SAMPLE_INPUT: &'static str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

struct Stack {
    parse_offset: usize,
    crates: Vec<u8>,
}

impl Stack {
    fn with_parse_offset(parse_offset: usize) -> Stack {
        Stack {
            parse_offset,
            crates: Vec::new(),
        }
    }
}

impl Deref for Stack {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.crates
    }
}

impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.crates
    }
}

fn parse_stacks(input: &str) -> Vec<Stack> {
    let mut lines_it = input.lines().rev();
    let header = lines_it.next().unwrap();

    let mut stacks: Vec<_> = header
        .match_indices(char::is_numeric)
        .map(|(idx, _)| idx)
        .map(Stack::with_parse_offset)
        .collect();

    assert!(!stacks.is_empty());
    lines_it.for_each(|line| {
        stacks
            .iter_mut()
            .for_each(|stack| match line.as_bytes()[stack.parse_offset] {
                b' ' => (),
                v @ b'A'..=b'Z' => stack.push(v),
                v @ _ => panic!("Couldn't parse crate: `{v}`"),
            });
    });
    stacks
}

mod part1 {
    use super::*;

    fn solution(input: &str) -> String {
        assert_eq!(input.bytes().count(), input.len());
        let (stacks, procedure) = input.split_once("\n\n").unwrap();
        let mut stacks = parse_stacks(stacks);

        procedure.lines().for_each(|line| {
            let mut it = line
                .splitn(6, ' ')
                .filter(|s| !(s == &"move" || s == &"from" || s == &"to"))
                .map(|s| s.parse::<usize>().unwrap());
            let count = it.next().unwrap();
            let src = &mut stacks[it.next().unwrap() - 1];
            let split_idx = src.len() - count;
            let crates = src.split_off(split_idx);
            let dst = &mut stacks[it.next().unwrap() - 1];
            dst.extend(crates.iter().rev());
        });
        stacks
            .iter()
            .map(
                |stack| *stack.last().unwrap() as char, /*Char cast could waste memroy*/
            )
            .collect()
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
