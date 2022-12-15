use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};
#[derive(Copy, Clone, PartialEq, Eq)]
enum CellState {
    Rock,
    Sand,
}

fn print_cave(config: &HashMap<(usize, usize), CellState>) {
    for y in 0..10 {
        for x in 494..=503 {
            let state = config.get(&(x, y));
            if state == None {
                print!(".");
            } else if state == Some(&CellState::Rock) {
                print!("#");
            } else {
                print!("o");
            }
        }
        println!();
    }
}
fn main() {
    let mut config = HashMap::new();
    let mut deepest = 0_usize;
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .for_each(|l| {
            l.unwrap()
                .split(" -> ")
                .map(|s| s.split_once(',').unwrap())
                .map(|(l, r)| (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()))
                .collect::<Vec<_>>()
                .windows(2)
                .for_each(|w| {
                    let start = w[0];
                    let end = w[1];
                    if start.0 == end.0 {
                        for y in std::cmp::min(start.1, end.1)..=std::cmp::max(start.1, end.1) {
                            config.insert((start.0, y), CellState::Rock);
                        }
                    } else if start.1 == end.1 {
                        for x in std::cmp::min(start.0, end.0)..=std::cmp::max(start.0, end.0) {
                            config.insert((x, start.1), CellState::Rock);
                        }
                    } else {
                        panic!("Sould not happen");
                    }
                    if std::cmp::max(start.1, end.1) > deepest {
                        deepest = std::cmp::max(start.1, end.1);
                    }
                })
        });
    //println!("{}", deepest);
    let mut nb_grains = 0;
    // Outer loop: one for each grain of sand
    loop {
        let mut pos = (500, 0);
        //print_cave(&config);
        if config.get(&pos).is_some() {
            break;
        }
        // Inner loop: one for each movement of a grain of sand
        loop {
            if pos.1 == deepest + 1 {
                config.insert(pos, CellState::Sand);
                nb_grains += 1;
                break;
            }
            let down = (pos.0, pos.1 + 1);
            let down_left = (pos.0 - 1, pos.1 + 1);
            let down_right = (pos.0 + 1, pos.1 + 1);
            if config.get(&down).is_none() {
                pos = down;
            } else if config.get(&down_left).is_none() {
                pos = down_left;
            } else if config.get(&down_right).is_none() {
                pos = down_right
            } else {
                // Cannot fall further down
                config.insert(pos, CellState::Sand);
                nb_grains += 1;
                break;
            }
        }
    }
    println!("{nb_grains}");
}
