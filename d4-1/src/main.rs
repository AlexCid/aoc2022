use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!(
        "{:?}",
        BufReader::new(File::open("input.txt").unwrap())
            .lines()
            .map(|l| l
                .unwrap()
                .split([',', '-'])
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>())
            .filter(|vals| (vals[0] >= vals[2] && vals[1] <= vals[3])
                || (vals[0] <= vals[2] && vals[1] >= vals[3]))
            .count() //    .collect::<Vec<_>>()
    )
}
