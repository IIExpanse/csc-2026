use std::{
    io::{self, BufRead},
};

fn main() {

}

fn parse_tokens(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}