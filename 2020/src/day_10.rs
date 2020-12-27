use std::fs;

pub fn a() {
    let contents = fs::read_to_string("input/day_10.txt")
        .expect("Something went wrong reading the file");

    let mut numbers: Vec<usize> = contents
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect();

    numbers.sort();

    let mut one = 0;
    let mut three = 1;
    let mut iter = numbers.iter().peekable();
    while let Some(&elem) = iter.next() {
        if let Some(&next) = iter.peek() {
            if next - elem == 1 {
                one += 1;
            } else if next - elem == 3 {
                three += 1;
            }
        }
    }
    if numbers[0] == 1 {
        one += 1;
    } else if numbers[0] == 3 {
        three += 1;
    }

    println!("{}", three * one);
}

pub fn b() {
    let contents = fs::read_to_string("input/day_10.txt")
        .expect("Something went wrong reading the file");

    let mut numbers: Vec<usize> = contents
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect();

    numbers.sort();

    let mut paths: Vec<usize> = vec![0; numbers.len()];
    
    for (i, n) in numbers.iter().enumerate() {
        let mut count = 0;
        if i <= 2 && n <= &3 {
            count += 1;
        }

        if i >= 1 && n - numbers[i - 1] <= 3 {
            count += paths[i - 1];
        }

        if i >= 2 && n- numbers[i - 2] <= 3 {
            count += paths[i - 2];
        }

        if i >= 3 && n- numbers[i - 3] <= 3 {
            count += paths[i - 3];
        }

        paths[i] = count;
    }

    println!("{}", paths.last().unwrap());
}