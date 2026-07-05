use std::{
    io::{self, Read}
};

fn main() {
    let mut handle = io::stdin().lock();
    let mut input = String::new();
    handle.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let tokens = parse_tokens(lines.next().unwrap());
    let n: i32 = tokens[0].parse().unwrap();

    let mut queries: Vec<[u32; 3]> = Vec::new();
    queries.reserve(n as usize * 2);

    for _ in 0..n {
         let tokens = parse_tokens(lines.next().unwrap());

         let a = tokens[0].parse().unwrap();
         let f = tokens[1].parse().unwrap();
         let s = tokens[2].parse().unwrap();

         queries.push([a, s, 0]);
         queries.push([f, s, 1]);
    }
    queries.sort_by(|arr1, arr2| {
        if arr1[0] != arr2[0] {
            return arr1[0].cmp(&arr2[0]);
        }
        arr2[2].cmp(&arr1[2])
    });

    let mut sum: u64 = 0;
    let mut max_sum: u64 = 0;

    for query in queries {
        if query[2] == 0 {
            sum += query[1] as u64;
            max_sum = max_sum.max(sum);

        } else {
            sum -= query[1] as u64
        }
    }

    println!("{}", max_sum);
}

fn parse_tokens(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}