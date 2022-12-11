use std::fmt::Debug;

//      __
// w  c(..)o   (
//  \__(-)    __)
//      /\   (
//     /(_)___)
//     w /|
//      | \
//     m  m
struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    divisibility_test: u64,
    if_true_idx: usize,
    if_false_idx: usize,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            //.field("operation", &self.operation)
            .field("divisibility_test", &self.divisibility_test)
            .field("if_true_idx", &self.if_true_idx)
            .field("if_false_idx", &self.if_false_idx)
            .finish()
    }
}

fn main() {
    let mut monkeys = include_str!("../input.txt")
        .split("\n\n")
        .map(|desc| {
            let mut desc = desc.split('\n').skip(1);
            let items = desc
                .next()
                .unwrap()
                .split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            let (operator, value) = desc
                .next()
                .unwrap()
                .split_once(" = old ")
                .unwrap()
                .1
                .split_once(' ')
                .unwrap();
            let operation: Box<dyn Fn(u64) -> u64> = match (operator, value.parse::<u64>().ok()) {
                ("*", Some(i)) => Box::new(move |v| v * i),
                ("*", None) => Box::new(|v| v * v),
                ("+", Some(i)) => Box::new(move |v| v + i),
                ("+", None) => Box::new(|v| v + v),
                _ => unreachable!("Should not happen"),
            };
            let divisibility_test = desc
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let if_true_idx = desc
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let if_false_idx = desc
                .next()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            Monkey {
                items,
                operation,
                divisibility_test,
                if_true_idx,
                if_false_idx,
            }
        })
        .collect::<Vec<_>>();

    let all_divisibility = monkeys.iter().map(|m| m.divisibility_test).product::<u64>();

    let mut inspected = vec![0; monkeys.len()];
    for _ in 0..10000 {
        //println!("{:?}", monkeys);
        for i in 0..monkeys.len() {
            // Monkey gets item
            while let Some(worry_level) = monkeys[i].items.pop() {
                // Count item as inspected
                inspected[i] += 1;
                // Change worry_level
                //dbg!(worry_level);
                let new_worry_level = (monkeys[i].operation)(worry_level) % all_divisibility;
                //dbg!(new_worry_level);
                let idx = if new_worry_level % monkeys[i].divisibility_test == 0 {
                    monkeys[i].if_true_idx
                } else {
                    monkeys[i].if_false_idx
                };
                //dbg!(idx);
                monkeys[idx].items.push(new_worry_level);
            }
        }
    }
    inspected.sort_unstable();
    println!("{}", inspected.into_iter().rev().take(2).product::<u64>());
}
