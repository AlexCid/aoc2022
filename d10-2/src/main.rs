use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!(
        "{}",
        BufReader::new(File::open("input2.txt").unwrap())
            .lines()
            .fold((&mut vec![1], 1), |(v, reg), cmd| {
                match cmd.unwrap().split_once(' ') {
                    None => {
                        v.push(reg);
                        (v, reg)
                    }
                    Some((_, val)) => {
                        v.push(reg);
                        v.push(reg + val.parse::<i64>().unwrap());
                        (v, reg + val.parse::<i64>().unwrap())
                    }
                }
            })
            .0
            .iter()
            .take(240)
            .enumerate()
            .map(|(idx, reg)| {
                let y = idx % 40;
                if (reg - y as i64).abs() <= 1 {
                    '#'
                } else {
                    '.'
                }
            })
            .collect::<Vec<_>>()
            .as_slice()
            .chunks(40)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
