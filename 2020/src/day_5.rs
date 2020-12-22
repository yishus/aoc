use std::fs;

pub fn a() {
    let contents = fs::read_to_string("input/day_5.txt")
        .expect("Something went wrong reading the file");
    
    let lines: Vec<&str> = contents
        .split('\n')
        .collect();

    let mut max = 0;
    let mut min = 1023;

    for line in lines {
        let seat_id = seat_from_binary(line);
        if seat_id > max {
            max = seat_id;
        }
        if seat_id < min {
            min = seat_id;
        }
    }

    println!("{} {}", max, min);
}

pub fn b() {
    let contents = fs::read_to_string("input/day_5.txt")
        .expect("Something went wrong reading the file");
    
    let lines: Vec<&str> = contents
        .split('\n')
        .collect();

    // Min and max values from part a
    let mut array: [bool; 856-81] = [false; 856-81];
    for line in lines {
        let seat_id = seat_from_binary(line);
        array[seat_id - 81] = true;
    }

    for (seat, taken) in array.iter().enumerate() {
        if !taken {
            println!("{}", seat + 81);
        }
    }
}

fn seat_from_binary(row: &str) -> usize {
    let chars = row.chars();
    let mut value = 0;
    for c in chars {
        if c == 'F' || c == 'L' {
            value = value << 1;
        }

        if c == 'B' || c == 'R' {
            value = value << 1;
            value = value | 1; 
        }
    }
    return value;
}