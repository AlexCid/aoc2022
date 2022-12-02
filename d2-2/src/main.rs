use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!(
        "{:?}",
        BufReader::new(File::open("input.txt").unwrap())
            .lines()
            .map(|l| {
                l.unwrap()
                    .split_once(' ')
                    .map(|(elf, s)| {
                        (
                            // Rock = 0, Paper = 1, Scissor = 2
                            ((elf.chars().next().unwrap() as u64) - ('A' as u64)) as u64,
                            // Draw = 0, Win = 1, Loose = 2
                            ((((s.chars().next().unwrap() as i8) - ('Y' as i8)) + 3) % 3) as u64,
                        )
                    })
                    .map(|(elf, outcome)| ((elf + outcome) % 3, outcome))
                    .map(|(me, outcome)| (me + 1) + ((outcome + 1) % 3) * 3)
                    .unwrap()
            })
            .sum::<u64>()
    )
}
