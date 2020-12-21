use std::fs;

pub fn a() {
    let contents = fs::read_to_string("input/day_3.txt")
        .expect("Something went wrong reading the file");

    let rows: Vec<&str> = contents
        .split('\n')
        .collect();

    let width = rows[0].len();
    let height = rows.len();
    let mut i = 0;
    let mut j = 0;
    let mut count = 0;

    while j < height {
        if &rows[j][i..i+1] == "#" {
            count += 1;
        }
        i = (i + 3) % width;
        j += 1;
    }

    println!("{}", count);
}

pub fn b() {
    let contents = fs::read_to_string("input/day_3.txt")
        .expect("Something went wrong reading the file");

    let rows: Vec<&str> = contents
        .split('\n')
        .collect();
    
    let slopes: [[usize; 2]; 5] = [
        [1, 1],
        [3, 1],
        [5, 1],
        [7, 1],
        [1, 2]
    ];
    let mut multiply = 1;
    for s in &slopes {
        let count = count_tress(&rows, s[0], s[1]);
        multiply *= count;
    }
    println!("{}", multiply);
}

fn count_tress(rows: &Vec<&str>, move_x: usize, move_y: usize) -> usize {
    let width = rows[0].len();
    let height = rows.len();
    let mut i = 0;
    let mut j = 0;
    let mut count = 0;

    while j < height {
        if &rows[j][i..i+1] == "#" {
            count += 1;
        }
        i = (i + move_x) % width;
        j += move_y;
    }

    return count;
}