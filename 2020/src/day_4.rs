use std::collections::{HashMap, HashSet};
use std::fs;
use itertools::Itertools;

pub fn a() {
    let contents = fs::read_to_string("input/day_4.txt")
        .expect("Something went wrong reading the file");

    let passports: Vec<&str> = contents
        .split("\n\n")
        .collect();

    let mut count = 0;
    for p in passports {
        let fields: HashSet<&str> = p
            .split_whitespace()
            .map(|s| s.split(':').collect::<Vec<&str>>()[0])
            .collect();
        if validate(fields) {
            count += 1;
        }
    }

    println!("{}", count);
}

pub fn b() {
    let contents = fs::read_to_string("input/day_4.txt")
        .expect("Something went wrong reading the file");

    let passports: Vec<&str> = contents
        .split("\n\n")
        .collect();

    let mut count = 0;
    for p in passports {
        let fields: HashMap<&str, &str> = p
            .split_whitespace()
            .flat_map(|l| l.split(":"))
            .tuples()
            .collect();
        if validate_hashmap(fields) {
            count += 1;
        }
    }

    println!("{}", count);
}

fn validate(p_fields: HashSet<&str>) -> bool {
    let required_fields: HashSet<&'static str> =
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().cloned().collect();
    if required_fields.difference(&p_fields).collect::<HashSet<_>>().len() == 0 {
        return true;
    }

    return false;
}

fn validate_hashmap(fields: HashMap<&str, &str>) -> bool {
    let keys: HashSet<&str> = fields.keys().cloned().collect();
    if !validate(keys) {
        return false;
    }

    for (field_name, value) in &fields {
        match field_name {
            &"byr" => {
                let byr: usize = match value.parse() {
                    Ok(i) => i,
                    Err(_) => return false
                };
                if byr < 1920 || byr > 2002 {
                    return false;
                }
            },
            &"iyr" => {
                let iyr: usize = match value.parse() {
                    Ok(i) => i,
                    Err(_) => return false
                };
                if iyr < 2010 || iyr > 2020 {
                    return false;
                }
            },
            &"eyr" => {
                let eyr: usize = match value.parse() {
                    Ok(i) => i,
                    Err(_) => return false
                };
                if eyr < 2020 || eyr > 2030 {
                    return false;
                }
            },
            &"hgt" => {
                let hgt_cm = match value.strip_suffix("cm") {
                    Some(h) => h.parse::<usize>().unwrap_or(0),
                    None => 0
                };
                let hgt_in = match value.strip_suffix("in") {
                    Some(h) => h.parse::<usize>().unwrap_or(0),
                    None => 0
                };
                let validate_cm = hgt_cm >= 150 && hgt_cm <= 193;
                let validate_in = hgt_in >= 59 && hgt_in <= 76;
                if !validate_cm && !validate_in {
                    return false;
                }
            },
            &"hcl" => {
                let hcl = match value.strip_prefix("#") {
                    Some(hcl) => hcl,
                    None => return false
                };
                if hcl.chars().count() != 6 || !hcl.chars().all(char::is_alphanumeric) {
                    return false;
                }
            },
            &"ecl" => {
                let colors: HashSet<&'static str> =
                    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                        .iter().cloned().collect();

                if !colors.contains(value) {
                    return false;
                }
            },
            &"pid" => {
                if value.chars().count() != 9 || !value.chars().all(char::is_numeric) {
                    return false;
                }
            },
            _ => ()
        }
    }

    return true;
}