use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use scan_fmt::scan_fmt;
const TARGET_Y: i64 = 2000000;
//const TARGET_Y: i64 = 10;

fn main() {
    let (beacons, segments): (Vec<_>, Vec<_>) = BufReader::new(File::open("input.txt").unwrap())
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
        .map(|(s_x, s_y, b_x, b_y)| {
            let distance = (s_x - b_x).abs() + (s_y - b_y).abs();
            let length = distance - (s_y - TARGET_Y).abs();
            let segment = if length >= 0 {
                Some((s_x - length, s_x + length))
            } else {
                None
            };
            let beacon_at_target = if b_y == TARGET_Y { Some(b_x) } else { None };
            (beacon_at_target, segment)
        })
        .unzip();

    let mut positions: HashSet<i64> = HashSet::new();

    segments.iter().filter_map(|x| *x).for_each(|(start, end)| {
        for i in start..=end {
            positions.insert(i);
        }
    });

    beacons.iter().filter_map(|x| *x).for_each(|x| {
        positions.remove(&x);
    });

    let mut positions = positions.into_iter().collect::<Vec<_>>();
    positions.sort();
    println!("{:?}", positions.len());
}
