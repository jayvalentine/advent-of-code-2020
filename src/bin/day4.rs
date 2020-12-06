// Advent of Code 2020
// Day 4

use std::collections::HashMap;

use aoc::file::*;

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn test_example1() {
        let passport = "
        ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm";

        let (valid, _) = passport_valid(passport);

        assert_eq!(true, valid);
    }

    #[test]
    fn test_example2() {
        let passport = "
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929";

        let (valid, _) = passport_valid(passport);

        assert_eq!(false, valid);
    }

    #[test]
    fn test_example3() {
        let passport = "
        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm";

        let (valid, _) = passport_valid(passport);

        assert_eq!(true, valid);
    }

    #[test]
    fn test_example4() {
        let passport = "
        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in";

        let (valid, _) = passport_valid(passport);

        assert_eq!(false, valid);
    }
}

#[cfg(test)]
mod test_fields {
    use super::*;

    #[test]
    fn test_byr_valid() {
        assert_eq!(true, byr_valid("1920"));
        assert_eq!(false, byr_valid("1919"));

        assert_eq!(true, byr_valid("2002"));
        assert_eq!(false, byr_valid("2003"));

        assert_eq!(true, byr_valid("1969"));

        assert_eq!(false, byr_valid("foo"));
    }

    #[test]
    fn test_iyr_valid() {
        assert_eq!(true, iyr_valid("2010"));
        assert_eq!(false, iyr_valid("2009"));

        assert_eq!(true, iyr_valid("2020"));
        assert_eq!(false, iyr_valid("2021"));

        assert_eq!(true, iyr_valid("2016"));

        assert_eq!(false, iyr_valid("foo"));
    }

    #[test]
    fn test_eyr_valid() {
        assert_eq!(true, eyr_valid("2020"));
        assert_eq!(false, eyr_valid("2019"));

        assert_eq!(true, eyr_valid("2030"));
        assert_eq!(false, eyr_valid("2031"));

        assert_eq!(true, eyr_valid("2025"));

        assert_eq!(false, eyr_valid("foo"));
    }

    #[test]
    fn test_hgt_valid() {
        assert_eq!(true, hgt_valid("190cm"));
        assert_eq!(true, hgt_valid("60in"));

        // Upper limit
        assert_eq!(true, hgt_valid("193cm"));
        assert_eq!(true, hgt_valid("76in"));

        // Lower limit
        assert_eq!(true, hgt_valid("150cm"));
        assert_eq!(true, hgt_valid("59in"));

        // Out of range (just)
        assert_eq!(false, hgt_valid("194cm"));
        assert_eq!(false, hgt_valid("77in"));
        assert_eq!(false, hgt_valid("149cm"));
        assert_eq!(false, hgt_valid("58cm"));

        // Nonsense
        assert_eq!(false, hgt_valid("bar"));

        // Number; no unit
        assert_eq!(false, hgt_valid("85"));
    }

    #[test]
    fn test_hcl_valid() {
        assert_eq!(true, hcl_valid("#123456"));

        assert_eq!(false, hcl_valid("#12345"));
        assert_eq!(false, hcl_valid("#1234567"));

        assert_eq!(true, hcl_valid("#abcdef"));
        assert_eq!(true, hcl_valid("#1a3c5f"));

        assert_eq!(false, hcl_valid("123456"));
        assert_eq!(false, hcl_valid("1234567"));

        assert_eq!(false, hcl_valid("#123abz"));
    }

    #[test]
    fn test_ecl_valid() {
        assert_eq!(true, ecl_valid("amb"));
        assert_eq!(true, ecl_valid("blu"));
        assert_eq!(true, ecl_valid("brn"));
        assert_eq!(true, ecl_valid("gry"));
        assert_eq!(true, ecl_valid("grn"));
        assert_eq!(true, ecl_valid("hzl"));
        assert_eq!(true, ecl_valid("oth"));

        assert_eq!(false, ecl_valid("huh"));
        assert_eq!(false, ecl_valid("am"));
        assert_eq!(false, ecl_valid("grne"));
        assert_eq!(false, ecl_valid("grey"));
    }

    #[test]
    fn test_pid_valid() {
        assert_eq!(true, pid_valid("123456789"));
        assert_eq!(true, pid_valid("000045678"));
        assert_eq!(true, pid_valid("000000000"));

        assert_eq!(false, pid_valid("0123456789"));
        assert_eq!(false, pid_valid("12345678"));
        assert_eq!(false, pid_valid("123A56789"));
        assert_eq!(false, pid_valid("!23456789"));
    }
}

// Regression testing previous puzzle answers to make sure
// we don't break anything.
#[cfg(test)]
mod test_puzzle_answers {
    use super::*;

    #[test]
    fn test_part1() {
        let answer = part1();
        assert_eq!(216, answer);
    }

    #[test]
    fn test_part2() {
        let answer = part2();
        assert_eq!(150, answer);
    }
}

fn pid_valid(s: &str) -> bool {
    if s.len() != 9 {
        return false;
    }

    for c in s.chars() {
        match c {
            '0'..='9' => continue,
            _ => return false
        };
    }

    return true;
}

fn ecl_valid(s: &str) -> bool {
    let valid = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    return valid.contains(&s);
}

fn hcl_valid(s: &str) -> bool {
    // Expect exactly 7 characters.
    if s.len() != 7 {
        return false;
    }
    
    // Invalid unless starts with '#'
    if !s.starts_with("#") {
        return false;
    }

    // Iterate over the last 6 characters.
    for c in s[1..].chars() {
        match c {
            'a'..='f' => continue,
            '0'..='9' => continue,
            _ => return false
        };
    }

    return true;
}

fn hgt_valid(s: &str) -> bool {
    let unit = &s[s.len()-2..];
    
    let value = &s[..s.len()-2];
    let value: u32 = match value.parse() {
        Ok(n) => n,
        Err(_) => return false
    };

    if unit == "cm" {
        return (value >= 150) && (value <= 193);
    } else if unit == "in" {
        return (value >= 59) && (value <= 76);
    } else {
        return false;
    }
}

fn byr_valid(s: &str) -> bool {
    return yr_valid(s, 1920, 2002);
}

fn iyr_valid(s: &str) -> bool {
    return yr_valid(s, 2010, 2020);
}

fn eyr_valid(s: &str) -> bool {
    return yr_valid(s, 2020, 2030);
}

fn yr_valid(s: &str, min: u32, max: u32) -> bool {
    let year: u32 = match s.parse() {
        Ok(y) => y,
        Err(_) => return false
    };

    return (year >= min) && (year <= max);
}

fn passport_valid_check_fields(p: &str) -> bool {
    let (valid, passport) = passport_valid(p);
    if !valid {
        return false;
    }

    // We know the passport has all required fields
    // so it is safe to expect them here.
    let byr = passport.get("byr").unwrap();
    let iyr = passport.get("iyr").unwrap();
    let eyr = passport.get("eyr").unwrap();
    let hgt = passport.get("hgt").unwrap();
    let hcl = passport.get("hcl").unwrap();
    let ecl = passport.get("ecl").unwrap();
    let pid = passport.get("pid").unwrap();

    return byr_valid(byr) &&
        iyr_valid(iyr) &&
        eyr_valid(eyr) &&
        hgt_valid(hgt) &&
        hcl_valid(hcl) &&
        ecl_valid(ecl) &&
        pid_valid(pid);
}

fn passport_valid(p: &str) -> (bool, HashMap<String, String>) {
    let p = p.trim();

    let mut fields: HashMap<String, String> = HashMap::new();

    // Fields in password are split by whitespace.
    for field in p.split(char::is_whitespace) {
        if field.trim().is_empty() {
            continue;
        }

        let split_index = field.find(':').expect(&format!("Invalid password field: {}", field));
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
            return (false, fields);
        }
    }

    return (true, fields);
}

fn part1() -> u32 {
    // Read test data in, iterate over each line.
    let mut f = GroupedFileReader::open("data/day4.txt").expect("Could not open data/day4.txt");

    let mut valid_passports = 0;

    while let FileReadResult::Success(group) = f.next_group() {
        let (valid, _) = passport_valid(&group);
        if valid {
            valid_passports += 1;
        }
    }

    valid_passports
}

fn part2() -> u32 {
    // Read test data in, iterate over each line.
    let mut f = GroupedFileReader::open("data/day4.txt").expect("Could not open data/day4.txt");

    let mut valid_passports = 0;

    while let FileReadResult::Success(group) = f.next_group() {
        let valid = passport_valid_check_fields(&group);
        if valid {
            valid_passports += 1;
        }
    }

    valid_passports
}

fn main() {
    let valid_passports = part1();
    println!("Part 1: The number of valid passports is: {}", valid_passports);

    let valid_passports = part2();
    println!("Part 2: The number of valid passports (checking fields) is: {}", valid_passports);
}
