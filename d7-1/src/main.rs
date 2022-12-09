use std::{
    cell::Cell,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum MyDirEntry {
    File(usize),
    Directory(Cell<Option<usize>>, HashMap<String, MyDirEntry>),
}

impl MyDirEntry {
    fn traverse_mut(&mut self, path: &[String]) -> &mut Self {
        let mut cur = self;
        for el in path {
            match cur {
                MyDirEntry::File(_) => panic!("Oops"),
                MyDirEntry::Directory(_, hm) => cur = hm.get_mut(el).unwrap(),
            }
        }
        cur
    }

    fn traverse(&self, path: &[String]) -> &Self {
        let mut cur = self;
        for el in path {
            match cur {
                MyDirEntry::File(_) => panic!("Oops"),
                MyDirEntry::Directory(_, hm) => cur = hm.get(el).unwrap(),
            }
        }
        cur
    }

    fn as_dir_mut(&mut self) -> Option<(&Cell<Option<usize>>, &mut HashMap<String, MyDirEntry>)> {
        match self {
            MyDirEntry::File(_) => None,
            MyDirEntry::Directory(c, hm) => Some((c, hm)),
        }
    }
    fn as_dir(&self) -> Option<(&Cell<Option<usize>>, &HashMap<String, MyDirEntry>)> {
        match self {
            MyDirEntry::File(_) => None,
            MyDirEntry::Directory(c, hm) => Some((c, hm)),
        }
    }

    fn compute_sizes(&self) -> usize {
        match self {
            MyDirEntry::File(size) => *size,
            MyDirEntry::Directory(c, hm) => {
                let total_size = c.take();
                if let Some(total_size) = total_size {
                    c.set(Some(total_size));
                    total_size
                } else {
                    let total_size = hm.values().map(|entry| entry.compute_sizes()).sum();
                    c.set(Some(total_size));
                    total_size
                }
            }
        }
    }

    fn walk<'s, T>(&self, state: &'s mut T, f: fn(&mut T, &Self)) -> &'s mut T {
        f(state, self);
        match self {
            MyDirEntry::Directory(_, hm) => {
                for val in hm.values() {
                    val.walk(state, f);
                }
            }
            MyDirEntry::File(_) => {}
        }
        state
    }
}

fn main() {
    let mut dir_entry = MyDirEntry::Directory(Cell::new(None), HashMap::new());
    let mut curr_path = Vec::<String>::new();
    let mut lines = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .skip(1)
        .peekable();
    while let Some(line) = lines.next() {
        //println!("@/{}, state: \n{:?}", curr_path.join("/"), dir_entry);
        if line.starts_with("$ ") {
            // A command
            if let Some(line) = line.strip_prefix("$ cd ") {
                // change directory
                if line.starts_with("..") {
                    curr_path.pop().expect("Should not be at root dir");
                } else {
                    curr_path.push(line.to_string());
                }
            } else {
                // Should be an ls
                assert_eq!(line, "$ ls");
                while let Some(line) = lines.peek() {
                    if line.starts_with("$ ") {
                        break;
                    }
                    let line = lines.next().unwrap();
                    // Either new dir, or file with size
                    let (_, hm) = dir_entry.traverse_mut(&curr_path).as_dir_mut().unwrap();

                    if let Some(dirname) = line.strip_prefix("dir ") {
                        // new directory
                        hm.entry(dirname.to_string()).or_insert_with(|| {
                            MyDirEntry::Directory(Cell::default(), HashMap::default())
                        });
                    } else {
                        // Should be a file
                        let (size, name) = line.split_once(' ').unwrap();
                        let size: usize = size.parse().unwrap();
                        hm.insert(name.to_string(), MyDirEntry::File(size));
                    }
                }
            }
        }
    }

    dir_entry.compute_sizes();
    println!(
        "{:?}",
        dir_entry
            .walk(&mut Vec::<usize>::new(), |state, el| state.extend(
                el.as_dir().map(|(c, _)| c.clone().take().unwrap()) //.unwrap_or_default()
            ))
            .into_iter()
            .filter(|v| **v < 100000)
            .map(|v| *v)
            .sum::<usize>() //.collect::<Vec<_>>()
    );
}
