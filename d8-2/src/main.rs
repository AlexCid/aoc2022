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
    let mut best_score = 0;
    for i in 0..dim_x {
        for j in 0..dim_y {
            // to the right
            let up = (1..=i)
                .map(|di| trees[i - di][j] < trees[i][j])
                .enumerate()
                .find(|v| !v.1)
                .map(|(idx, _)| idx + 1)
                .unwrap_or(i);
            let down = (1..(dim_x - i))
                .map(|di| trees[i + di][j] < trees[i][j])
                .enumerate()
                .find(|v| !v.1)
                .map(|(idx, _)| idx + 1)
                .unwrap_or_else(|| dim_x.saturating_sub(i + 1));
            let right = (1..(dim_y - j))
                .map(|dj| trees[i][j + dj] < trees[i][j])
                .enumerate()
                .find(|v| !v.1)
                .map(|(idx, _)| idx + 1)
                .unwrap_or_else(|| dim_y.saturating_sub(j + 1));
            let left = (1..=j)
                .map(|dj| trees[i][j - dj] < trees[i][j])
                .enumerate()
                .find(|v| !v.1)
                .map(|(idx, _)| idx + 1)
                .unwrap_or(j);
            let res = left * right * down * up;

            if res > best_score {
                best_score = res;
            }
        }
    }

    println!("{}", best_score);
}
