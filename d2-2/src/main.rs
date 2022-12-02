use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn outcome(me: u64, other: u64) -> u64 {
    // ROCK = 0, PAPER = 1, SCISSOR = 2
    if me == other {
        3
    } else if (me + 1) % 3 == other {
        0
    } else {
        6
    }
}

pub fn outcome_from_str(s: &str) -> u64 {
    // Draw = 0, Win = 1, Loose = 2
    ((((s.chars().next().unwrap() as i8) - ('Y' as i8)) + 3) % 3) as u64
}

fn main() {
    println!(
        "{:?}",
        BufReader::new(File::open("input.txt").unwrap())
            .lines()
            .map(|l| {
                l.unwrap()
                    .split_once(' ')
                    .map(|(elf, outcome)| {
                        (
                            ((elf.chars().next().unwrap() as u64) - ('A' as u64)) as u64,
                            outcome_from_str(outcome),
                        )
                    })
                    .map(|(elf, outcome)| ((elf + outcome) % 3, outcome))
                    .map(|(me, outcome)| (me + 1) + ((outcome + 1) % 3) * 3)
                    .unwrap()
            })
            .sum::<u64>()
    )
}
