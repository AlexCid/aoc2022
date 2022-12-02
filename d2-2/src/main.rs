use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Clone, Copy, Debug)]
enum RPS {
    Rock,
    Paper,
    Scissor,
}

impl RPS {
    pub fn outcome(self, other: RPS) -> u64 {
        match (self, other) {
            (RPS::Rock, RPS::Rock) => 3,
            (RPS::Rock, RPS::Paper) => 0,
            (RPS::Rock, RPS::Scissor) => 6,
            (RPS::Paper, RPS::Rock) => 6,
            (RPS::Paper, RPS::Paper) => 3,
            (RPS::Paper, RPS::Scissor) => 0,
            (RPS::Scissor, RPS::Rock) => 0,
            (RPS::Scissor, RPS::Paper) => 6,
            (RPS::Scissor, RPS::Scissor) => 3,
        }
    }

    pub fn score(self) -> u64 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissor => 3,
        }
    }
}

impl FromStr for RPS {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RPS::Rock),
            "B" => Ok(RPS::Paper),
            "C" => Ok(RPS::Scissor),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Win,
    Draw,
    Loose,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

impl Outcome {
    pub fn score(self) -> u64 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loose => 0,
        }
    }

    pub fn find_move(self, other: RPS) -> RPS {
        match (self, other) {
            (Outcome::Win, RPS::Rock) => RPS::Paper,
            (Outcome::Win, RPS::Paper) => RPS::Scissor,
            (Outcome::Win, RPS::Scissor) => RPS::Rock,
            (Outcome::Draw, RPS::Rock) => RPS::Rock,
            (Outcome::Draw, RPS::Paper) => RPS::Paper,
            (Outcome::Draw, RPS::Scissor) => RPS::Scissor,
            (Outcome::Loose, RPS::Rock) => RPS::Scissor,
            (Outcome::Loose, RPS::Paper) => RPS::Rock,
            (Outcome::Loose, RPS::Scissor) => RPS::Paper,
        }
    }
}

fn main() {
    println!(
        "{}",
        BufReader::new(File::open("input.txt").unwrap())
            .lines()
            .map(|l| {
                l.unwrap()
                    .split_once(' ')
                    .map(|(elf, outcome)| {
                        (
                            RPS::from_str(elf).unwrap(),
                            Outcome::from_str(outcome).unwrap(),
                        )
                    })
                    .map(|(elf, outcome)| (outcome.find_move(elf), outcome))
                    .map(|(me, outcome)| me.score() + outcome.score())
                    .unwrap()
            })
            .sum::<u64>()
    )
}
