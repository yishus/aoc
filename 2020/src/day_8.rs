use std::collections::HashSet;
use std::fs;

pub fn a() {
    let contents = fs::read_to_string("input/day_8.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents
        .split('\n')
        .collect();

    let (acc, _) = run(&lines, None);

    println!("{}", acc);
}

pub fn b() {
    let contents = fs::read_to_string("input/day_8.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents
        .split('\n')
        .collect();

    let len = lines.len();
    let mut seen: HashSet<usize> = HashSet::new();
    let mut pointer = 0;
    while pointer < len && seen.get(&pointer) == None {
        seen.insert(pointer);
        let parts: Vec<&str> = lines[pointer as usize].split(' ').collect();
        let instruction = parts[0];
        let current_pointer = pointer;
        
        let (acc, terminated) = match instruction {
            "nop" => {
                pointer += 1;
                run(&lines, Some(current_pointer))
            },
            "jmp" => {
                pointer = pointer_update(pointer, parts[1]);
                run(&lines, Some(current_pointer))
            },
            "acc" => {
                pointer += 1;
                (0, false)
            },
            _ => (0, false)
        };

        if terminated {
            println!("acc: {}", acc);
            break;
        }
    }
}

fn run(lines: &Vec<&str>, update_line: Option<usize>) -> (isize, bool) {
    let len = lines.len();
    let mut seen: HashSet<usize> = HashSet::new();
    let mut pointer = 0;
    let mut acc = 0;
    while pointer < len && seen.get(&pointer) == None {
        seen.insert(pointer);
        let parts: Vec<&str> = lines[pointer as usize].split(' ').collect();
        let instruction = parts[0];
        let value: isize = parts[1].parse().unwrap();

        match instruction {
            "nop" => {
                if update_line != None && update_line.unwrap() == pointer {
                    pointer = pointer_update(pointer, parts[1]);    
                } else {
                    pointer += 1;
                }
            },
            "acc" => {
                acc += value;
                pointer += 1;
            },
            "jmp" => {
                if update_line != None && update_line.unwrap() == pointer {
                    pointer += 1;
                } else {
                    pointer = pointer_update(pointer, parts[1]);    
                }
            },
            _ => ()
        }
    }

    return (acc, pointer == len);
}

fn pointer_update(current: usize, update: &str) -> usize {
    if &update[..1] == "-" {
        let subtract_value: usize = update[1..].parse().unwrap();
        return current - subtract_value;
    } else {
        let add_value: usize = update.parse().unwrap();
        return current + add_value;
    }
}