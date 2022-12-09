use std::{
    cmp::Ordering,
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn add(a: i64, order: Ordering) -> i64 {
    match order {
        Ordering::Less => a - 1,
        Ordering::Equal => a,
        Ordering::Greater => a + 1,
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Default)]
struct Coords {
    x: i64,
    y: i64,
}

impl Coords {
    fn touching(self, other: Self) -> bool {
        (other.x - self.x).abs() <= 1 && (other.y - self.y).abs() <= 1
    }
}

fn main() {
    let mut visited = BTreeSet::<Coords>::new();
    let mut head = Coords::default();
    let mut tail = Coords::default();
    visited.insert(tail);

    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| {
            l.unwrap()
                .split_once(' ')
                .map(|(l, r)| (l.chars().next().unwrap(), r.parse::<u64>().unwrap()))
                .unwrap()
        })
        .for_each(|(direction, nbr)| {
            for _ in 0..nbr {
                let (dx, dy) = match direction {
                    'U' => (1, 0),
                    'D' => (-1, 0),
                    'R' => (0, 1),
                    'L' => (0, -1),
                    _ => unreachable!("should not happen"),
                };
                head.x += dx;
                head.y += dy;

                if !head.touching(tail) {
                    tail.x = add(tail.x, head.x.cmp(&tail.x));
                    tail.y = add(tail.y, head.y.cmp(&tail.y));
                }
                visited.insert(tail);
                /*println!(
                    "head: {}, {} | tail: {}, {}",
                    head.x, head.y, tail.x, tail.y,
                );*/
            }
        });

    println!("{}", visited.len())
}
