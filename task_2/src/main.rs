use std::{
    io::{self, Read},
    collections::HashMap
};

fn main() {
    let mut handle = io::stdin().lock();
    let mut input = String::new();
    handle.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let mut tokens = parse_tokens(lines.next().unwrap());
    let m: u32 = tokens[0].parse().unwrap();

    let mut map: HashMap<u32, u32> = HashMap::new();
    map.reserve(5 * 1_000_000);

    for _ in 0..m {
        tokens = parse_tokens(lines.next().unwrap());
        let l: u32 = tokens[0].parse().unwrap();

        for _ in 0..l {
            tokens = parse_tokens(lines.next().unwrap());

            map.insert(tokens[0].parse().unwrap(), tokens[1].parse().unwrap());
        }
    }

    let mut ans = String::new();

    let mut sorted_keys: Vec<&u32> = map.keys().collect();
    sorted_keys.sort();

    for num in sorted_keys {
        ans.push_str(&(num.to_string().to_owned() + " " + &map.get(num).unwrap().to_string()));
        ans.push_str("\n");
    }

    println!("{}", ans);
}

fn parse_tokens(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}