use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt")
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