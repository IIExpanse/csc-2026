use std::{
    io::{self, BufRead}
};

fn main() {
    let mut handle = io::stdin().lock();
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    let tokens = parse_tokens(&input);

    println!("{}", "");
}

fn parse_tokens(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}