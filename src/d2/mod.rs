use std::cmp::Ordering;
use std::str::FromStr;
use Action::*;

#[derive(PartialEq)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&str> for Action {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Action::Rock),
            "B" | "Y" => Ok(Action::Paper),
            "C" | "Z" => Ok(Action::Scissors),
            _ => Err("Invalid token to parse action"),
        }
    }
}

impl Action {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        match (self, other) {
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Equal,
            (Rock, Paper) => Less,
            (Rock, Scissors) => Greater,
            (Paper, Rock) => Greater,
            (Paper, Scissors) => Less,
            (Scissors, Rock) => Less,
            (Scissors, Paper) => Greater,
        }
    }

    fn score(&self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

struct Match {
    opponent: Action,
    own: Action,
}

impl FromStr for Match {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opponent, own) = s.split_once(" ").ok_or("Couldn't split actions")?;
        let opponent: Action = opponent.try_into()?;
        let own: Action = own.try_into()?;
        Ok(Match { opponent, own })
    }
}

impl Match {
    fn get_score(&self) -> i32 {
        let outcome = match self.own.cmp(&self.opponent) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };
        outcome + self.own.score()
    }
}

mod part1 {
    use super::*;
    pub fn solution(input: &'static str) -> i32 {
        input
            .lines()
            .map(Match::from_str)
            .map(Result::unwrap)
            .map(|m| m.get_score())
            .sum::<i32>()
    }

    #[test]
    fn sample() {
        let sample_input = "A Y
B X
C Z";
        assert_eq!(solution(sample_input), 15);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(include_str!("input")), 13675);
    }
}
pub use part1::solution;
