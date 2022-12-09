use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};
const MARKER_SIZE: usize = 14;

fn main() {
    println!(
        "{}",
        BufReader::new(File::open("input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .last()
            .unwrap()
            .as_bytes()
            .windows(MARKER_SIZE)
            .enumerate()
            .find_map(|(idx, w)| {
                if HashSet::<u8>::from_iter(w.iter().copied()).len() == MARKER_SIZE {
                    Some(idx + MARKER_SIZE)
                } else {
                    None
                }
            })
            .unwrap()
    )
}
