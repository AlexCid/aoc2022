use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn possible_moves(cur_x: usize, cur_y: usize, heightmap: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let (dim_x, dim_y) = (heightmap.len(), heightmap[0].len());
    let mut res = Vec::new();
    if cur_x > 0 && heightmap[cur_x][cur_y] >= heightmap[cur_x - 1][cur_y] - 1 {
        res.push((cur_x - 1, cur_y));
    }
    if cur_x < dim_x - 1 && heightmap[cur_x][cur_y] >= heightmap[cur_x + 1][cur_y] - 1 {
        res.push((cur_x + 1, cur_y));
    }
    if cur_y > 0 && heightmap[cur_x][cur_y] >= heightmap[cur_x][cur_y - 1] - 1 {
        res.push((cur_x, cur_y - 1));
    }
    if cur_y < dim_y - 1 && heightmap[cur_x][cur_y] >= heightmap[cur_x][cur_y + 1] - 1 {
        res.push((cur_x, cur_y + 1));
    }
    //dbg!(res.clone());
    res
}

fn main() {
    let mut possible_starts = Vec::new();
    let mut end = (0, 0);
    let heightmap = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .enumerate()
        .map(|(idx_line, l)| {
            l.unwrap()
                .char_indices()
                .map(|(idx_col, c)| match c {
                    'b'..='z' => c as u8,
                    'a' | 'S' => {
                        possible_starts.push((idx_line, idx_col));
                        b'a'
                    }
                    'E' => {
                        end = (idx_line, idx_col);
                        b'z'
                    }
                    _ => unreachable!("Should not happen"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (dim_x, dim_y) = (heightmap.len(), heightmap[0].len());
    let mut nb_moves = vec![vec![usize::MAX; dim_y]; dim_x];

    let res = possible_starts
        .into_iter()
        .map(|start| {
            nb_moves[start.0][start.1] = 0;
            let mut to_explore = vec![start];

            while !to_explore.is_empty() {
                let mut new_to_explore = Vec::new();

                for (cur_x, cur_y) in to_explore.drain(..) {
                    for (new_x, new_y) in possible_moves(cur_x, cur_y, &heightmap).into_iter() {
                        if nb_moves[new_x][new_y] > nb_moves[cur_x][cur_y] + 1 {
                            nb_moves[new_x][new_y] = nb_moves[cur_x][cur_y] + 1;
                            new_to_explore.push((new_x, new_y));
                        }
                    }
                }
                to_explore.append(&mut new_to_explore);
            }
            /*for i in 0..dim_x {
                for j in 0..dim_y {
                    if nb_moves[i][j] == usize::MAX {
                        print!("X");
                    } else {
                        print!("{}", nb_moves[i][j]);
                    }
                }
                println!();
            }*/
            nb_moves[end.0][end.1]
        })
        .min()
        .unwrap();
    println!("{res}");
}
