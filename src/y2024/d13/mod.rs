pub const INPUT: &str = include_str!("input.txt");

pub const SAMPLE: &str = include_str!("sample.txt");

#[derive(Debug)]
struct Machine {
    x_a: f64,
    y_a: f64,
    x_b: f64,
    y_b: f64,
    x_targ: f64,
    y_targ: f64,
}

/// Convert to a positive integer if within tolerance
fn to_int(v: f64) -> Option<u64> {
    if v > 0.0 && (v.round() - v).abs() < 1e-3
    /* threshold had to be decreased for part 2 */
    {
        Some(v.round() as u64)
    } else {
        None
    }
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
    fn price(&self) -> u64 {
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
        // (8) n_a = ( y_targ / y_b - x_targ / x_b ) / ( y_a / y_b - x_a / x_b )

        let Machine {
            x_a,
            y_a,
            x_b,
            y_b,
            x_targ,
            y_targ,
        } = *self;

        // Calculate (8)
        let divisor = y_a / y_b - x_a / x_b;
        if divisor.abs() < 0.0001 {
            // If the divider gets zero, it gets more tricky
            // It means the vectors a and b are parallel
            // and there can be more than one solution
            unimplemented!("{self:?}");
        }
        let n_a = (y_targ / y_b - x_targ / x_b) / divisor;

        // Calculate (3)
        let n_b = (x_targ - n_a * x_a) / x_b;

        if let (Some(n_a), Some(n_b)) = (to_int(n_a), to_int(n_b)) {
            assert!(n_a * x_a as u64 + n_b * x_b as u64 == x_targ as u64);
            assert!(n_a * y_a as u64 + n_b * y_b as u64 == y_targ as u64);
            3 * n_a + n_b
        } else {
            0
        }
    }
}

pub mod part1 {
    use super::*;

    pub fn solution(s: &str) -> u64 {
        parse(s).map(|m| m.price()).sum()
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

    pub fn solution(s: &str) -> u64 {
        parse(s)
            .map(|mut m| {
                m.x_targ += 10000000000000.0;
                m.y_targ += 10000000000000.0;
                m.price()
            })
            .sum()
    }

    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 104140871044942);
    }
}
