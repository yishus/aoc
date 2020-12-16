use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let mut seen = HashSet::new();

    let values: Vec<usize> = contents
        .split('\n')
        .map(|s| s.parse().expect("parse error"))
        .collect();
    
    for v in values {
        let pair = 2020 - v;
        if seen.contains(&pair) {
            println!("{}", v * pair);
            break;
        }
        seen.insert(v);
    }
}