use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let trees = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|v| (v as u8) - b'0')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (dim_x, dim_y) = (trees[0].len(), trees.len());
    let mut visible = vec![vec![false; dim_x]; dim_y];

    // First, the lines
    for i in 0..dim_x {
        let mut cur_max = 0;
        for j in 0..dim_y {
            if j == 0 || trees[i][j] > cur_max {
                visible[i][j] = true;
                cur_max = trees[i][j]
            }
        }
    }

    for i in 0..dim_x {
        let mut cur_max = 0;
        for j in (0..dim_y).rev() {
            if j == dim_y - 1 || trees[i][j] > cur_max {
                visible[i][j] = true;
                cur_max = trees[i][j];
            }
        }
    }

    // Then, columns

    for j in 0..dim_y {
        let mut cur_max = 0;
        for i in 0..dim_x {
            if i == 0 || trees[i][j] > cur_max {
                visible[i][j] = true;
                cur_max = trees[i][j];
            }
        }
    }

    for j in 0..dim_y {
        let mut cur_max = 0;
        for i in (0..dim_x).rev() {
            if i == dim_x - 1 || trees[i][j] > cur_max {
                visible[i][j] = true;
                cur_max = trees[i][j];
            }
        }
    }

    //println!("{:?}", visible);
    println!(
        "{:?}",
        visible
            .into_iter()
            .map(|v| v.into_iter().filter(|x| *x).count())
            .sum::<usize>()
    )
}
