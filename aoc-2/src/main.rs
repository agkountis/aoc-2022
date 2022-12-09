use std::error::Error;
use std::fs::File;
use std::io::Read;
use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[repr(i32)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<i32> for Hand {
    fn from(input: i32) -> Self {
        match input {
            1 => Hand::Rock,
            2 => Hand::Paper,
            3 => Hand::Scissors,
            _ => panic!("Invalid conversion. Accepted i32 values -> [1, 2, 3]"),
        }
    }
}

impl std::ops::Add for Hand {
    type Output = i32;

    fn add(self, rhs: Self) -> Self::Output {
        self as i32 + rhs as i32
    }
}

impl std::ops::Add<Hand> for i32 {
    type Output = i32;

    fn add(self, rhs: Hand) -> Self::Output {
        self + rhs as i32
    }
}

impl std::ops::Add<i32> for Hand {
    type Output = Hand;

    fn add(self, rhs: i32) -> Self::Output {
        let mut res = self as i32 + rhs;
        if res > 3 {
            res = 1
        }

        Hand::from(res)
    }
}

impl std::ops::Sub<i32> for Hand {
    type Output = Hand;

    fn sub(self, rhs: i32) -> Self::Output {
        let mut res = self as i32 - rhs;
        if res < 1 {
            res = 3
        }

        Hand::from(res)
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        match s {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            _ => panic!("Unrecognized hand letter: {}", s)
        }
    }
}

impl Hand {
    pub fn vs(&self, other: Hand) -> Outcome {
        if *self == other {
            return Outcome::Tie(*self)
        }

        let rock_victory = *self + other == 4 && *self < other;
        let paper_victory = *self + other == 3 && *self > other;
        let scissors_victory = *self + other == 5 && *self > other;

        if rock_victory || paper_victory || scissors_victory {
            return Outcome::Victory(*self)
        }

        Outcome::Defeat(*self)
    }
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Victory(Hand),
    Defeat(Hand),
    Tie(Hand),
}

#[derive(Debug, Copy, Clone)]
enum ExpectedOutcome {
    Victory,
    Defeat,
    Tie,
}

impl From<&str> for ExpectedOutcome {
    fn from(s: &str) -> Self {
        match s {
            "Z" => ExpectedOutcome::Victory,
            "X" => ExpectedOutcome::Defeat,
            "Y" => ExpectedOutcome::Tie,
            _ => panic!("Unrecognized expected outcome letter.")
        }
    }
}

impl From<(Hand, ExpectedOutcome)> for Hand {
    fn from(input: (Hand, ExpectedOutcome)) -> Self {
        let (op_hand, exp_outcome) = input;
        match exp_outcome {
            ExpectedOutcome::Victory => op_hand + 1,
            ExpectedOutcome::Defeat => op_hand - 1,
            ExpectedOutcome::Tie => op_hand,
        }
    }
}

impl From<Outcome> for i32 {
    fn from(outcome: Outcome) -> Self {
        match outcome {
            Outcome::Victory(hand) => 6 + hand,
            Outcome::Defeat(hand) => hand as i32,
            Outcome::Tie(hand) => 3 + hand,
        }
    }
}

impl std::ops::Add<Outcome> for i32 {
    type Output = i32;

    fn add(self, rhs: Outcome) -> Self::Output {
        self + i32::from(rhs)
    }
}

pub fn part_one(content: &str) {
    let score = content.lines()
        .map(|s| {
            let (op_hand, pl_hand) = s.split_whitespace()
                .map(Hand::from)
                .tuples()
                .collect_vec()
                .first()
                .copied()
                .unwrap();

            let outcome = pl_hand.vs(op_hand);
            println!("Player<{:?}> vs Opponent<{:?}> -> Outcome<{:?}>", pl_hand, op_hand, outcome);
            outcome
        })
        .fold(0, |acc, outcome| acc + outcome);

    println!("Final player score: {:?}", score);
}

pub fn part_two(content: &str) {
    let score = content.lines()
        .map(|s| {
            let (op_hand, pl_hand, exp_outcome) = s.split_whitespace()
                .tuples()
                .map(|(hand, outcome)| {
                    let op_hand = Hand::from(hand);
                    let expected_outcome = ExpectedOutcome::from(outcome);
                    let pl_hand = Hand::from((op_hand, expected_outcome));
                    (op_hand, pl_hand, expected_outcome)
                })
                .collect_vec()
                .first()
                .copied()
                .unwrap();

            let outcome = pl_hand.vs(op_hand);
            println!("ExpectedOutcome: {:?} -----> Opponent<{:?}> vs Player<{:?}> -> Outcome<{:?}>", exp_outcome, op_hand, pl_hand, outcome);
            outcome
        })
        .fold(0, |acc, outcome| acc + outcome);

    println!("Final player score: {:?}", score)
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("aoc-2/data/strategy_guide.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    part_one(&content);
    part_two(&content);

    Ok(())
}
