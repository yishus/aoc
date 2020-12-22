use std::collections::{HashMap, HashSet};
use std::fs;

pub fn a() {
    let contents = fs::read_to_string("input/day_6.txt")
        .expect("Something went wrong reading the file");

    let groups: Vec<&str> = contents
        .split("\n\n")
        .collect();
    
    let mut total = 0;
    
    for group in groups {
        let mut set = HashSet::new();

        let answers: Vec<&str> = group
            .split('\n')
            .collect();
        for answer in answers {
            for q in answer.chars() {
                set.insert(q);
            }
        }

        total += set.len();
    }

    println!("{}", total);
}

pub fn b() {
    let contents = fs::read_to_string("input/day_6.txt")
        .expect("Something went wrong reading the file");
    
    let groups: Vec<&str> = contents
        .split("\n\n")
        .collect();
    
    let mut total = 0;
    
    for group in groups {
        let mut count = 0;
        let mut map: HashMap<char, usize> = HashMap::new();
        let answers: Vec<&str> = group
            .split('\n')
            .collect();
            
        for answer in &answers {
            for q in answer.chars() {
                let value = map.entry(q).or_insert(0);
                *value += 1;
            }
        }

        for (_, c) in map {
            if c == answers.len() {
                count += 1;
            }
        }
        
        total += count;
    }

    println!("{}", total);
}