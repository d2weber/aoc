use std::collections::HashMap;

pub const SAMPLE: &str = include_str!("sample");
pub const INPUT: &str = include_str!("input");

type Equations<'a> = HashMap<&'a str, Node<'a>>;

fn parse(s: &str) -> Equations {
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
                        "+" => Node::Other(lhs, Op::Add, rhs),
                        "-" => Node::Other(lhs, Op::Sub, rhs),
                        "*" => Node::Other(lhs, Op::Mul, rhs),
                        "/" => Node::Other(lhs, Op::Div, rhs),
                        _ => panic!("Invalid op `{op}`"),
                    }
                } else {
                    Node::Value(lhs.parse().unwrap())
                },
            )
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
enum Node<'a> {
    Value(i64),
    Other(&'a str, Op, &'a str),
}

fn traverse(mf: &Equations, recorder: &mut impl FnMut(&str, i64), k: &str) -> Option<i64> {
    let r = match mf.get(k)? {
        Node::Value(v) => *v,
        Node::Other(lhs, op, rhs) => {
            let l = traverse(mf, recorder, lhs);
            let r = traverse(mf, recorder, rhs)?;
            let l = l?;
            match op {
                Op::Add => l + r,
                Op::Sub => l - r,
                Op::Mul => l * r,
                Op::Div => l / r,
            }
        }
    };
    recorder(k, r);
    Some(r)
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> i64 {
        let m = parse(s);
        traverse(&m, &mut |_, _| {}, "root").unwrap()
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
        let mut m = parse(s);
        m.remove("humn").unwrap();
        let mut mem = HashMap::new();
        assert!(traverse(
            &m,
            &mut |k, v| {
                mem.insert(k.to_owned(), v);
            },
            "root",
        )
        .is_none());
        let root = match m.get("root").unwrap() {
            Node::Other(l, _, r) => 2 * mem.get(*l).unwrap_or(mem.get(*r).unwrap()),
            Node::Value(_) => panic!("Unexpected result"),
        };
        let mut m: Equations = m
            .into_iter()
            .flat_map(|(res, v)| match v {
                Node::Other(lhs, Op::Add, rhs) => vec![
                    (lhs, Node::Other(res, Op::Sub, rhs)),
                    (rhs, Node::Other(res, Op::Sub, lhs)),
                ],
                Node::Other(lhs, Op::Sub, rhs) => vec![
                    (lhs, Node::Other(res, Op::Add, rhs)),
                    (rhs, Node::Other(lhs, Op::Sub, res)),
                ],
                Node::Other(lhs, Op::Mul, rhs) => vec![
                    (lhs, Node::Other(res, Op::Div, rhs)),
                    (rhs, Node::Other(res, Op::Div, lhs)),
                ],
                Node::Other(lhs, Op::Div, rhs) => vec![
                    (lhs, Node::Other(res, Op::Mul, rhs)),
                    (rhs, Node::Other(lhs, Op::Div, res)),
                ],
                v @ Node::Value(_) => vec![(res, v)],
            })
            .collect();
        mem.into_iter().for_each(|(k, v)| {
            *m.get_mut(k.as_str()).unwrap() = Node::Value(v);
        });
        m.insert("root", Node::Value(root));
        traverse(&m, &mut |_, _| {}, "humn").unwrap()
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
