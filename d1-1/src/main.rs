use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::once,
};

/* Challenge: find the answer without using ; */
fn main() {
    println!(
        "{}",
        BufReader::new(File::open("input.txt").unwrap())
            .lines()
            .chain(once(Ok("\n".into())))
            .fold((0, 0), |(max, cur), l| match l.unwrap().parse::<u64>() {
                Ok(n) => (max, cur + n),
                Err(_) => {
                    if max < cur {
                        (cur, 0)
                    } else {
                        (max, 0)
                    }
                }
            })
            .0
    )
}
