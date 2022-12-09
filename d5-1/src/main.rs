use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut lines = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap());

    let mut state: Option<Vec<Vec<char>>> = None;
    let mut len = 0;
    loop {
        let line = lines.next().unwrap();
        if state.is_none() {
            // Initialization with the right number of stacks
            len = (line.len() + 1) / 4;
            state = Some(vec![Vec::new(); len]);
        }
        if line.is_empty() {
            // finished parsing initial state
            break;
        } else if line.starts_with(" 1") {
            // stack numbering line: we don't care
            continue;
        } else {
            // Initial state description
            let line = line.as_bytes();
            let local_state = state.as_mut().unwrap();
            for i in 0..len {
                if line[1 + 4 * i] != b' ' {
                    local_state[i].insert(0, line[1 + 4 * i] as char)
                }
            }
        }
    }

    let mut state = state.unwrap();
    println!("{:?}", state);

    for line in lines {
        let chunks = line.split(' ').collect::<Vec<_>>();

        let nb: usize = chunks[1].parse().unwrap();
        let from: usize = chunks[3].parse().unwrap();
        let from = from - 1;
        let to: usize = chunks[5].parse().unwrap();
        let to = to - 1;

        for _ in 0..nb {
            let my_crate = state[from].pop().unwrap();
            state[to].push(my_crate);
        }
    }
    println!("{:?}", state);
    println!(
        "{}",
        state.iter().map(|l| l.last().unwrap()).collect::<String>(),
    );
}
