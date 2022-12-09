use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!(
        "{}",
        BufReader::new(File::open("input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|mut l| (l.split_off(l.len() / 2), l))
            .map(|(l, r)| {
                (
                    l.chars().collect::<HashSet<_>>(),
                    r.chars().collect::<HashSet<_>>(),
                )
            })
            .map(|(l, r)| *l.intersection(&r).next().unwrap())
            .map(|c| match c {
                'a'..='z' => c as u32 - 'a' as u32 + 1,
                'A'..='Z' => c as u32 - 'A' as u32 + 27,
                _ => unreachable!("should not happen"),
            })
            .sum::<u32>()
    )
}
