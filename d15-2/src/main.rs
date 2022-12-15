use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use scan_fmt::scan_fmt;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
const MIN: i64 = 0;
//const MAX: i64 = 20;
const MAX: i64 = 4000000;
struct LinesSet(Vec<(i64, i64)>);

impl LinesSet {
    fn new(d: impl Iterator<Item = (i64, i64)>) -> Self {
        Self(d.filter(|(s, e)| *s <= MAX && *e >= MIN).collect())
    }
    fn coalesce(&mut self) -> Option<i64> {
        self.0.sort_unstable();
        let (_, mut end) = self.0[0];
        for (s, e) in self.0[1..].iter() {
            if *s <= end + 1 {
                if *e > end {
                    end = *e;
                }
            } else {
                return Some(end + 1);
            }
        }
        None
    }
}

fn main() {
    let state = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| {
            scan_fmt!(
                &l.unwrap(),
                "Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}",
                i64,
                i64,
                i64,
                i64
            )
            .unwrap()
        })
        .collect::<Vec<_>>();

    (MIN..MAX).into_par_iter().for_each(|y| {
        let mut ls = LinesSet::new(state.iter().filter_map(|(s_x, s_y, b_x, b_y)| {
            let distance = (s_x - b_x).abs() + (s_y - b_y).abs();
            let length = distance - (s_y - y).abs();
            if length >= 0 {
                Some((s_x - length, s_x + length))
            } else {
                None
            }
            //let beacon_at_target = if *b_y == y { Some(b_x) } else { None };
        }));
        if let Some(x) = ls.coalesce() {
            println!("{}", y + x * 4000000);
        }
    });
}
