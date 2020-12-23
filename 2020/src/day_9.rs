use std::collections::HashSet;
use std::fs;

pub fn a() {
    let contents = fs::read_to_string("input/day_9.txt")
        .expect("Something went wrong reading the file");

    let numbers: Vec<usize> = contents
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect();

    let preamble_len = 25;

    let mut set: HashSet<usize> = numbers[..preamble_len].iter().cloned().collect();

    for n in preamble_len..numbers.len() {
        if !valid(&set, numbers[n]) {
            println!("{}", numbers[n]);
            break;
        }
        set.insert(numbers[n]);
        set.remove(&numbers[n - preamble_len]);
    }
}

fn valid(set: &HashSet<usize>, target: usize) -> bool {
    for num in set {
        if target > *num && set.get(&(target - num)) != None {
            return true;
        }
    }

    return false;
}

pub fn b() {
    let contents = fs::read_to_string("input/day_9.txt")
        .expect("Something went wrong reading the file");

    let numbers: Vec<usize> = contents
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect();
    
    let target = 1038347917;
    let mut i = 0;
    let mut j = 0;
    let mut sum = numbers[0];
    loop {
        if sum == target {
            break;
        } else if sum > target {
            sum -= numbers[i];
            i += 1;
        } else {
            j += 1;
            sum += numbers[j];
        }

    }

    let mut min = 1038347917;
    let mut max = 0;
    for x in i..j+1 {
        if numbers[x] < min {
            min = numbers[x];
        }
        if numbers[x] > max {
            max = numbers[x];
        }
    }

    println!("{}", min + max);
}