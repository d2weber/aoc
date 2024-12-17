use itertools::Itertools;

pub const INPUT: &str = include_str!("input.txt");

pub const SAMPLE: &str = include_str!("sample.txt");
pub const SAMPLE_PART2: &str = include_str!("sample_part2.txt");

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> String {
        let (mut state, program) = parse(s);
        run(&mut state, program)
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), "4,3,2,6,4,5,3,2,4");
    }
}

#[cfg(test)]
pub mod part2 {
    use super::*;

    #[test]
    fn actual_proof() {
        let (mut state, program) = parse(INPUT);
        state.reg_a = 164540892147389;
        assert_eq!(run(&mut state, program.clone()), program.0.iter().join(","));
    }

    // I used code similar to the demo test below to find solutions for parts
    // of my input program and manually concatenated the found solutions for reg_a
    //
    //                     reg_a                       |                 program
    //  ------------------------------------------------------------------------------------------------
    //                                              110 [2]
    //                                             1110 [2, 4]
    //                                        101001100 [2, 4, 1]
    //                                  111100110110101 [2, 4, 1, 1]
    //                            100000111100110110101 [2, 4, 1, 1, 7]
    //                           1011010110111010111101 [2, 4, 1, 1, 7, 5]
    //                          11011010110111010111101 [2, 4, 1, 1, 7, 5, 1]
    //                        1011011010110111010111101 [2, 4, 1, 1, 7, 5, 1, 5]
    //                        1011011010110111010111101 [2, 4, 1, 1, 7, 5, 1, 5, 4]
    //                  1011001011011010110111010111101 [2, 4, 1, 1, 7, 5, 1, 5, 4, 5]
    //               1000101001011011010110111010111101 [2, 4, 1, 1, 7, 5, 1, 5, 4, 5, 0]
    // 100101011010011000101001011011010                               [5, 1, 5, 4, 5, 0, 3, 5, 5, 3, 0]
    // 100101011010011000101001011011                                     [1, 5, 4, 5, 0, 3, 5, 5, 3, 0]
    // 100101011010011000101001011                                           [5, 4, 5, 0, 3, 5, 5, 3, 0]
    // 100101011010011000101001                                                 [4, 5, 0, 3, 5, 5, 3, 0]
    // 100101011010011000101                                                       [5, 0, 3, 5, 5, 3, 0]

    // Solution:
    // 100101011010011000101001011011010110111010111101 == 164540892147389

    #[test]
    #[ignore = "only demo"]
    fn find() {
        let ns = [2, 4, 1, 1, 7, 5, 1, 5, 4, 5, 0, 3, 5, 5, 3, 0];
        for n in (1..ns.len() - 1).rev() {
            let s = &ns[n..ns.len()];
            let r = find_reg_a(Default::default(), parse(INPUT).1, s);
            println!("{:?} {r:b}", s);
        }
        assert!(false);
    }

    fn find_reg_a(init_state: State, program: Program, expected: &[u8]) -> u64 {
        'outer: for reg_a in 0.. {
            let mut state = State {
                reg_a,
                ..init_state
            };
            if reg_a % 10_000_000_000 == 0 {
                dbg!(reg_a);
            }
            let mut expected_sequence = expected.iter().peekable();
            while let Some([instr, oper]) = program.0.get(state.ip..state.ip + 2) {
                if let Some(output) = state.op(*instr, *oper) {
                    if output == *expected_sequence.next().unwrap() {
                        if expected_sequence.peek().is_none() {
                            return reg_a;
                        }
                    } else {
                        continue 'outer;
                    }
                }
            }
        }
        unreachable!()
    }
}

fn run(state: &mut State, program: Program) -> String {
    let mut out = Vec::new();
    while let Some([instr, oper]) = program.0.get(state.ip..state.ip + 2) {
        if let Some(v) = state.op(*instr, *oper) {
            out.push(v);
        }
    }
    out.into_iter().join(",")
}

#[derive(Debug, Default)]
struct State {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    ip: usize,
}

impl State {
    fn op(&mut self, instr: u8, operand: u8) -> Option<u8> {
        if instr == 3 && self.reg_a != 0 {
            self.ip = operand as usize;
        } else {
            self.ip += 2;
        }
        match instr {
            0 => self.reg_a /= 2_u64.pow(self.combo(operand).try_into().unwrap()),
            1 => self.reg_b ^= operand as u64,
            2 => self.reg_b = self.combo(operand) % 8,
            3 => (), // already handled above
            4 => self.reg_b ^= self.reg_c,
            5 => return Some((self.combo(operand) % 8) as u8),
            6 => self.reg_b = self.reg_a / 2_u64.pow(self.combo(operand).try_into().unwrap()),
            7 => self.reg_c = self.reg_a / 2_u64.pow(self.combo(operand).try_into().unwrap()),
            _ => panic!("Invalid instruction `{instr}`"),
        }
        None
    }

    fn combo(&self, operand: u8) -> u64 {
        match operand {
            o @ 0..=3 => o as u64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Invalid operand `{operand}`"),
        }
    }
}

#[derive(Clone, Debug)]
struct Program(Vec<u8>);

fn parse(s: &str) -> (State, Program) {
    let mut lines = s.lines();
    (
        State {
            reg_a: lines
                .next()
                .unwrap()
                .strip_prefix("Register A: ")
                .unwrap()
                .parse()
                .unwrap(),
            reg_b: lines
                .next()
                .unwrap()
                .strip_prefix("Register B: ")
                .unwrap()
                .parse()
                .unwrap(),
            reg_c: lines
                .next()
                .unwrap()
                .strip_prefix("Register C: ")
                .unwrap()
                .parse()
                .unwrap(),
            ..Default::default()
        },
        Program(
            lines
                .nth(1)
                .unwrap()
                .strip_prefix("Program: ")
                .unwrap()
                .split(',')
                .map(|s| {
                    assert!(s.as_bytes().len() == 1);
                    s.as_bytes()[0]
                        .checked_sub(48)
                        .expect("Op code should be a number")
                })
                .collect(),
        ),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two() {
        let mut state = State {
            reg_c: 9,
            ..Default::default()
        };
        state.op(2, 6);
        assert_eq!(state.reg_b, 1);
    }

    #[test]
    fn five() {
        let mut state = State {
            reg_a: 10,
            ..Default::default()
        };
        assert_eq!(state.op(5, 0), Some(0));
        assert_eq!(state.op(5, 1), Some(1));
        assert_eq!(state.op(5, 4), Some(2));
    }

    #[test]
    fn zero_five_three() {
        let mut state = State {
            reg_a: 2024,
            ..Default::default()
        };
        assert_eq!(
            run(&mut state, Program(vec![0, 1, 5, 4, 3, 0])),
            "4,2,5,6,7,7,7,7,3,1,0"
        );
        assert_eq!(state.reg_a, 0);
    }

    #[test]
    fn one() {
        let mut state = State {
            reg_b: 29,
            ..Default::default()
        };
        state.op(1, 7);
        assert_eq!(state.reg_b, 26);
    }

    #[test]
    fn four() {
        let mut state = State {
            reg_b: 2024,
            reg_c: 43690,
            ..Default::default()
        };
        state.op(4, 0);
        assert_eq!(state.reg_b, 44354);
    }
}
