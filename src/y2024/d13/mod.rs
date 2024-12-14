pub const INPUT: &str = include_str!("input.txt");

pub const SAMPLE: &str = include_str!("sample.txt");

#[derive(Debug)]
struct Machine {
    x_a: i64,
    y_a: i64,
    x_b: i64,
    y_b: i64,
    x_targ: i64,
    y_targ: i64,
}

/// Divides `q` by `d` if `q` is exactly divisible by `d`
fn if_even_checked_div(q: i64, d: i64) -> Option<Option<i64>> {
    (q % d == 0).then_some(q.checked_div(d))
}

fn parse(s: &str) -> impl Iterator<Item = Machine> + '_ {
    s.split("\n\n").map(|s| {
        let mut lines = s.lines();
        let (x_a, y_a) = lines
            .next()
            .unwrap()
            .strip_prefix("Button A: X")
            .unwrap()
            .split_once(", Y")
            .unwrap();
        let (x_b, y_b) = lines
            .next()
            .unwrap()
            .strip_prefix("Button B: X")
            .unwrap()
            .split_once(", Y")
            .unwrap();
        let (x_targ, y_targ) = lines
            .next()
            .unwrap()
            .strip_prefix("Prize: X=")
            .unwrap()
            .split_once(", Y=")
            .unwrap();
        Machine {
            x_a: x_a.parse().unwrap(),
            y_a: y_a.parse().unwrap(),
            x_b: x_b.parse().unwrap(),
            y_b: y_b.parse().unwrap(),
            x_targ: x_targ.parse().unwrap(),
            y_targ: y_targ.parse().unwrap(),
        }
    })
}

impl Machine {
    fn price(&self) -> Option<i64> {
        // In the more common case there can only be one solution
        // and the costs don't really matter:
        //
        // (1) n_a * x_a + n_b * x_b = x_targ
        // (2) n_a * y_a + n_b * y_b = y_targ
        //
        // Resolve (1) according to n_b
        // (3) n_b = ( x_targ - n_a * x_a ) / x_b
        //
        // Insert (3) into (2)
        // (4) n_a * y_a + ( x_targ - n_a * x_a ) / x_b * y_b = y_targ
        // Resolve according to n_a
        // (5) n_a * y_a + x_targ * y_b / x_b - n_a * x_a * y_b / x_b = y_targ
        // (6) n_a * ( y_a - x_a * y_b / x_b ) + x_targ * y_b / x_b = y_targ
        // (7) n_a = ( y_targ - x_targ * y_b / x_b ) / ( y_a - x_a * y_b / x_b )
        // (8) n_a = ( y_targ * x_b - x_targ * y_b ) / ( y_a * x_b - x_a * y_b )

        let Machine {
            x_a,
            y_a,
            x_b,
            y_b,
            x_targ,
            y_targ,
        } = *self;

        // Calculate (8)
        let Some(n_a) = if_even_checked_div(y_targ * x_b - x_targ * y_b, y_a * x_b - x_a * y_b)?
        else {
            // If the divider gets zero, it gets more tricky
            // It means the vectors a and b are parallel
            // and there can be more than one solution
            unimplemented!("{self:?}")
        };

        // Calculate (3)
        let n_b = if_even_checked_div(x_targ - n_a * x_a, x_b)?.unwrap();

        assert!(n_a * x_a + n_b * x_b == x_targ);
        assert!(n_a * y_a + n_b * y_b == y_targ);

        Some(3 * n_a + n_b)
    }
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> i64 {
        parse(s).filter_map(|m| m.price()).sum()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 480);
    }

    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 29201);
    }
}

pub mod part2 {
    use super::*;

    pub fn solution(s: &str) -> i64 {
        parse(s)
            .filter_map(|mut m| {
                m.x_targ += 10000000000000;
                m.y_targ += 10000000000000;
                m.price()
            })
            .sum()
    }

    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 104140871044942);
    }
}
