use std::{
    io::{self, Read},
    collections::HashSet
};

fn main() {
    let mut handle = io::stdin().lock();
    let mut input = String::new();
    handle.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let tokens = parse_tokens(lines.next().unwrap());
    let n: u32 = tokens[0].parse().unwrap();

    let mut next_group: u32 = 1;
    let mut set: HashSet<u32> = HashSet::new();
    set.reserve(n as usize);

    let mut max_waiting: usize = 0;

    for num in parse_tokens(lines.next().unwrap()) {
        let cur: u32 = num.parse().unwrap();

        if cur != next_group {
            set.insert(cur);
            max_waiting = max_waiting.max(set.len());
            continue;
        }
        next_group += 1;

        while set.contains(&next_group) {
            set.remove(&next_group);
            next_group += 1;
        }
    }

    println!("{}", max_waiting);
}

fn parse_tokens(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}