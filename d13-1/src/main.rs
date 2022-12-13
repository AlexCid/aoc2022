use std::{cmp::Ordering, str::FromStr};

enum Packet {
    Integer(u64),
    List(Vec<Packet>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::List(l0), Self::Integer(r0)) => l0 == &vec![Packet::Integer(*r0)],
            (Self::Integer(l0), Self::List(r0)) => &vec![Packet::Integer(*l0)] == r0,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => a.partial_cmp(b),
            (Packet::Integer(a), Packet::List(b)) => vec![Packet::Integer(*a)].partial_cmp(b),
            (Packet::List(a), Packet::Integer(b)) => a.partial_cmp(&vec![Packet::Integer(*b)]),
            (Packet::List(a), Packet::List(b)) => a.partial_cmp(b),
        }
    }
}

impl Eq for Packet {}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn split_outer(s: &str) -> Vec<&str> {
    let mut stack_height: u32 = 0;
    let mut last_idx: usize = 0;
    let mut res = Vec::new();
    for (idx, c) in s.char_indices() {
        if stack_height == 0 && c == ',' {
            res.push(&s[last_idx..idx]);
            last_idx = idx + 1;
        } else if c == '[' {
            stack_height += 1;
        } else if c == ']' {
            stack_height -= 1;
        }
    }
    res.push(&s[last_idx..]);
    res
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //dbg!(s);
        if let Some(s) = s.strip_prefix('[') {
            // Find the index of the closing ]
            let mut stack_height = 1;
            let mut it = s.char_indices();
            let end_idx = loop {
                if let Some((idx, c)) = it.next() {
                    if c == '[' {
                        stack_height += 1;
                    } else if c == ']' {
                        stack_height -= 1;
                    }

                    if stack_height == 0 {
                        break idx;
                    }
                } else {
                    return Err("Wrongly formed input".to_string());
                }
            };

            Ok(Packet::List(
                split_outer(&s[..end_idx])
                    .into_iter()
                    .filter_map(|s| Packet::from_str(s).ok())
                    .collect(),
            ))
        } else {
            // Should be an integer
            Ok(Packet::Integer(s.parse().map_err(|_| "empty".to_string())?))
        }
    }
}

fn main() {
    println!(
        "{:?}",
        include_str!("../input.txt")
            .split("\n\n")
            .map(|s| {
                let (a, b) = s.split_once('\n').unwrap();
                Packet::from_str(a).cmp(&Packet::from_str(b))
            })
            .enumerate()
            .filter_map(|(idx, o)| if o == Ordering::Less {
                Some(idx + 1)
            } else {
                None
            })
            .sum::<usize>() //.collect::<Vec<_>>()
    );
}
