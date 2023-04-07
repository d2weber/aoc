use Action::*;
use Outcome::*;

#[derive(Clone, PartialEq)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}

impl TryFrom<&str> for Outcome {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(Lose),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err("Invalid token to parse outcome"),
        }
    }
}

impl TryFrom<&str> for Action {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err("Invalid token to parse action"),
        }
    }
}

impl Action {
    fn score(&self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

struct Match {
    own: Action,
    other: Action,
    outcome: Outcome,
}

const GAME_LOGIC: [Match; 6] = [
    Match {
        own: Rock,
        other: Paper,
        outcome: Lose,
    },
    Match {
        own: Rock,
        other: Scissors,
        outcome: Win,
    },
    Match {
        own: Paper,
        other: Rock,
        outcome: Win,
    },
    Match {
        own: Paper,
        other: Scissors,
        outcome: Lose,
    },
    Match {
        own: Scissors,
        other: Rock,
        outcome: Lose,
    },
    Match {
        own: Scissors,
        other: Paper,
        outcome: Win,
    },
];
impl Match {
    fn new(other: Action, own: Action) -> Self {
        if own == other {
            Match {
                own,
                other,
                outcome: Draw,
            }
        } else {
            GAME_LOGIC
                .into_iter()
                .find(|m| m.own == own && m.other == other)
                .unwrap()
        }
    }
    fn from_other_action(other: Action, outcome: Outcome) -> Self {
        if matches!(outcome, Draw) {
            Match {
                own: other.clone(),
                other,
                outcome,
            }
        } else {
            GAME_LOGIC
                .into_iter()
                .find(|m| m.other == other && m.outcome == outcome)
                .unwrap()
        }
    }

    fn get_score(&self) -> i32 {
        self.outcome.score() + self.own.score()
    }
}

pub const SAMPLE: &str = "A Y
B X
C Z";

pub const INPUT: &str = include_str!("input");

pub mod part2 {
    use super::*;

    pub fn solution(s: &str) -> i32 {
        s.lines()
            .map(|line| {
                let (other, outcome) = line.split_once(' ').unwrap();
                let other: Action = other.try_into().unwrap();
                let outcome: Outcome = outcome.try_into().unwrap();
                Match::from_other_action(other, outcome).get_score()
            })
            .sum()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 12);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 14184);
    }
}

pub mod part1 {
    use super::*;

    pub fn solution(input: &'static str) -> i32 {
        input
            .lines()
            .map(|line| {
                let (other, own) = line.split_once(' ').unwrap();
                let other: Action = other.try_into().unwrap();
                let own: Action = own.try_into().unwrap();
                Match::new(other, own).get_score()
            })
            .sum()
    }

    #[test]
    fn sample() {
        assert_eq!(solution(SAMPLE), 15);
    }
    #[test]
    fn actual() {
        assert_eq!(solution(INPUT), 13675);
    }
}
