use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let mut seen = HashSet::new();
    
    let values: Vec<isize> = contents
        .split('\n')
        .map(|s| s.parse().expect("parse error"))
        .collect();

    for (i, v) in values.iter().enumerate() {
        if i != 0 && i < values.len() - 1 {
            for j in i+1..values.len() {
                let r = 2020 - v - values[j];
                if seen.contains(&r) {
                    println!("{}", r * v * values[j]);
                    break;
                }
            }
        }
        seen.insert(v);
    }
}