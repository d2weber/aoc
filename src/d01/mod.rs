pub const INPUT: &str = include_str!("input");

pub fn groups(s: &str) -> impl Iterator<Item = i32> + '_ {
    s.split("\n\n").map(|group| {
        group
            .lines()
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .sum()
    })
}

pub mod part1 {
    use super::*;
    pub fn solution(s: &str) -> i32 {
        groups(s).max().unwrap()
    }

    #[test]
    fn part1() {
        assert_eq!(solution(INPUT), 67622);
    }
}

pub mod part2 {
    use super::*;
    pub fn solution(s: &str) -> i32 {
        let mut top_3 = [i32::MIN; 3];

        groups(s).for_each(|v| {
            let mut curr: i32 = v;
            for r in top_3.iter_mut() {
                if curr > *r {
                    std::mem::swap(&mut curr, &mut *r);
                }
            }
        });
        top_3
            .iter()
            .inspect(|v| assert_ne!(**v, i32::MIN))
            .sum::<i32>()
    }

    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 201491);
    }
}
