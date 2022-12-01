use std::{
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
    iter::once,
};

/* Challenge: find the answer without using ; */
fn main() {
    println!(
        "{:?}",
        BufReader::new(File::open("input.txt").unwrap())
            .lines()
            .chain(once(Ok("\n".into())))
            .fold((BinaryHeap::new(), 0), |(totals, cur), l| {
                match l.unwrap().parse::<u64>() {
                    Ok(n) => (totals, cur + n),
                    Err(_) => (totals.into_iter().chain(once(cur)).collect(), 0),
                }
            })
            .0
            .into_sorted_vec() // that's why we use BinaryHeap
            .into_iter()
            .rev() // into_sorted_vec uses ascending, not descending order...
            .take(3)
            .sum::<u64>()
    )
}
