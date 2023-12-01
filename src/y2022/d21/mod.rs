use itertools::Itertools;
use std::collections::HashMap;

pub const SAMPLE: &str = include_str!("sample");
pub const INPUT: &str = include_str!("input");

struct SystemOfEquations<'a>(HashMap<&'a str, Node<'a>>);

fn parse(s: &str) -> SystemOfEquations {
    SystemOfEquations(
        s.lines()
            .map(|l| {
                let (k, l) = l.split_once(": ").unwrap();
                let mut it = l.split_whitespace();
                let lhs = it.next().unwrap();
                (
                    k,
                    if let Some(op) = it.next() {
                        let rhs = it.next().unwrap();
                        match op {
                            "+" => Node::Operation(lhs, Op::Add, rhs),
                            "-" => Node::Operation(lhs, Op::Sub, rhs),
                            "*" => Node::Operation(lhs, Op::Mul, rhs),
                            "/" => Node::Operation(lhs, Op::Div, rhs),
                            _ => panic!("Invalid op `{op}`"),
                        }
                    } else {
                        Node::Leaf(lhs.parse().unwrap())
                    },
                )
            })
            .collect(),
    )
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn inverse(self) -> Op {
        match self {
            Op::Add => Op::Sub,
            Op::Sub => Op::Add,
            Op::Mul => Op::Div,
            Op::Div => Op::Mul,
        }
    }
}

#[derive(Debug, Clone)]
enum Node<'a> {
    Leaf(i64),
    Operation(&'a str, Op, &'a str),
}

impl Node<'_> {
    fn as_leaf(&self) -> Option<i64> {
        match self {
            Node::Leaf(v) => Some(*v),
            Node::Operation(_, _, _) => None,
        }
    }
}

impl<'a> SystemOfEquations<'a> {
    fn traverse(&mut self, k: &'a str) {
        let Some(node) = self.0.remove(k) else { return };
        let node = match node {
            Node::Operation(lhs, op, rhs) => {
                self.traverse(lhs);
                self.traverse(rhs);
                match (self.get_leaf(lhs), self.get_leaf(rhs)) {
                    (Some(l), Some(r)) => Node::Leaf(match op {
                        Op::Add => l + r,
                        Op::Sub => l - r,
                        Op::Mul => l * r,
                        Op::Div => l / r,
                    }),
                    _ => Node::Operation(lhs, op, rhs),
                }
            }
            leaf => leaf,
        };
        self.0.insert(k, node);
    }

    fn get_leaf(&self, k: &str) -> Option<i64> {
        self.0.get(k).and_then(Node::as_leaf)
    }

    fn reversed(self) -> SystemOfEquations<'a> {
        SystemOfEquations(
            self.0
                .into_iter()
                // Put the leafes last, so they overwrite Operations when collecting
                // Bools will be sorted false first, so Operations are first
                .sorted_unstable_by_key(|(_k, v)| matches!(v, Node::Leaf(_)))
                .flat_map(|(res, v)| match v {
                    Node::Operation(lhs, op, rhs) => match op {
                        Op::Add | Op::Mul => vec![
                            (lhs, Node::Operation(res, op.inverse(), rhs)),
                            (rhs, Node::Operation(res, op.inverse(), lhs)),
                        ],
                        Op::Sub | Op::Div => vec![
                            (rhs, Node::Operation(lhs, op, res)),
                            (lhs, Node::Operation(res, op.inverse(), rhs)),
                        ],
                    },
                    Node::Leaf(v) => vec![(res, Node::Leaf(v))],
                })
                .collect(),
        )
    }
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> i64 {
        let mut soe = parse(s);
        soe.traverse("root");
        soe.get_leaf("root").unwrap()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 152);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 309248622142100);
    }
}

pub mod part2 {
    use super::*;
    pub fn solution(s: &str) -> i64 {
        let mut soe = parse(s);
        soe.0.remove("humn").unwrap();
        soe.traverse("root");

        // In the input is written `root = lhs + rhs`
        // Either `lhs` or `rhs` has to be a leaf. To fullfill the given requrement
        // `lhs == rhs` we set the value of root to twice the leaf (rhs or lhs)
        let root_value = match soe.0.get("root").unwrap() {
            Node::Operation(l, Op::Add, r) => 2 * soe.get_leaf(l).or(soe.get_leaf(r)).unwrap(),
            _ => panic!(),
        };
        let mut soe = soe.reversed();
        soe.0.insert("root", Node::Leaf(root_value));
        soe.traverse("humn");
        soe.get_leaf("humn").unwrap()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 301);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 3757272361782);
    }
}
