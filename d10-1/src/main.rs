use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!(
        "{:?}",
        BufReader::new(File::open("input2.txt").unwrap())
            .lines()
            //.chain(once(Ok("nop".to_string())))
            .fold((0, 1, 0), |(cycle, reg, total), cmd| {
                match cmd.unwrap().split_once(' ') {
                    None => (
                        cycle + 1,
                        reg,
                        if cycle % 40 == 19 {
                            println!("{reg}*{}", cycle + 1);

                            total + (cycle + 1) * reg
                        } else {
                            total
                        },
                    ),
                    Some((_, value)) => (
                        cycle + 2,
                        reg + value.parse::<i64>().unwrap(),
                        if cycle % 40 == 19 {
                            println!("{reg}*{}", cycle + 1);
                            total + (cycle + 1) * reg
                        } else if (cycle + 1) % 40 == 19 {
                            println!("{reg}*{}", cycle + 2);
                            total + (cycle + 2) * (reg)
                        } else {
                            total
                        },
                    ),
                }
            })
    );
}
