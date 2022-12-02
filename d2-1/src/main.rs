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
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissor),
            _ => Err(()),
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
                    .map(|(elf, me)| {
                        RPS::from_str(me)
                            .unwrap()
                            .outcome(RPS::from_str(elf).unwrap())
                            + RPS::from_str(me).unwrap().score()
                    })
                    .unwrap()
            })
            .sum::<u64>()
    )
}
