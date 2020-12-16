use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents
        .split('\n')
        .collect();

    let mut count = 0;
    for line in lines {
        if password_valid(line) {
            count += 1;
        }
    }

    println!("{}", count);
}

fn password_valid(str: &str) -> bool {
    let parts: Vec<&str> = str.split(": ").collect();
    let condition_parts: Vec<&str> = parts[0].split(' ').collect();
    let range: Vec<usize> = condition_parts[0]
        .split('-')
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let hm = count_password_letters(parts[1]);
    match hm.get(&condition_parts[1]) {
        Some(count) => {
            if count >= &range[0] && count <= &range[1] {
                return true;
            } else {
                return false;
            }
        },
        None => { return false; }
    }
}

fn count_password_letters(password: &str) -> HashMap<&str, usize> {
    let mut hm = HashMap::new();
    let chars: Vec<&str> = password.split("").collect();
    for c in chars { 
        let count = hm.entry(c).or_insert(0);
        *count += 1;
    }

    return hm;
}