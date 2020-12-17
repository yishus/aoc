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
    let positions: Vec<usize> = condition_parts[0]
        .split('-')
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let at_first_position = &parts[1][positions[0] - 1..positions[0]] == condition_parts[1];
    let at_second_position = &parts[1][positions[1] - 1..positions[1]] == condition_parts[1];
    return (at_first_position || at_second_position)  && !(at_first_position && at_second_position);
}