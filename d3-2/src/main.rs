use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!(
        "{:?}",
        BufReader::new(File::open("input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|chunks| chunks
                .iter()
                .map(|c| c.chars().collect::<HashSet<char>>())
                .fold(
                    HashSet::from_iter(('a'..='z').chain('A'..='Z')),
                    |acc, el| { acc.intersection(&el).copied().collect::<HashSet<char>>() }
                ))
            .map(|h| match h.iter().next().unwrap() {
                v @ 'a'..='z' => *v as u32 - 'a' as u32 + 1,
                v @ 'A'..='Z' => *v as u32 - 'A' as u32 + 27,
                _ => unreachable!("skj"),
            })
            .sum::<u32>() //.collect::<Vec<_>>()
    )
}
