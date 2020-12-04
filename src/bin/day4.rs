// Advent of Code 2020
// Day 4

use std::collections::HashMap;

use std::fs::File;
use std::io::{prelude::*, BufReader};

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn test_example1() {
        let passport = "
        ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm";

        assert_eq!(true, passport_valid(passport));
    }

    #[test]
    fn test_example2() {
        let passport = "
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929";

        assert_eq!(false, passport_valid(passport));
    }

    #[test]
    fn test_example3() {
        let passport = "
        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm";

        assert_eq!(true, passport_valid(passport));
    }

    #[test]
    fn test_example4() {
        let passport = "
        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in";

        assert_eq!(false, passport_valid(passport));
    }
}

fn passport_valid(p: &str) -> bool {
    let p = p.trim();

    let mut fields: HashMap<String, String> = HashMap::new();

    // Fields in password are split by whitespace.
    for field in p.split(char::is_whitespace) {
        if field.trim().is_empty() {
            continue;
        }

        let split_index = field.find(":").expect(&format!("Invalid password field: {}", field));
        let split = field.split_at(split_index);

        if fields.contains_key(split.0) {
            panic!(format!("Duplicate key {} in passport:\n{}", split.0, p));
        }
        fields.insert(String::from(split.0), String::from(&split.1[1..]));
    }

    // All fields except "cid" are required.
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    // Check each of the required fields.
    // Password is invalid if it is missing any one of them.
    for f in required_fields {
        if !fields.contains_key(f) {
            return false;
        }
    }

    return true;
}

fn main() {
    // Read test data into vector.
    let mut v: Vec<String> = Vec::new();
    
    // Read test data in, iterate over each line.
    let f = File::open("data/day4.txt").expect("Could not open data/day4.txt");
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.expect("Invalid line in data/day4.txt");

        v.push(line);
    }

    let mut current_passport = String::new();
    let mut valid_passports = 0;

    for line in v {
        // If we've hit a blank line, that's the end of the current
        // passport. Check it if we've collected something.
        //
        // Otherwise this isn't a blank line, and is a continuation
        // of the current passport.
        if line.trim().is_empty() && !current_passport.is_empty() {
            if passport_valid(&current_passport) {
                valid_passports += 1;
            }

            current_passport.clear();
        } else {
            current_passport.push(' ');
            current_passport.push_str(&line);
        }
    }

    println!("Part 1: The number of valid passports is: {}", valid_passports);
}
