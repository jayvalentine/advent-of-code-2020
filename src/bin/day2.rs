// Advent of Code 2020
// Day 2

use std::fs::File;
use std::io::{prelude::*, BufReader};

#[cfg(test)]
mod tests_parse_rule {
    use super::*;

    // Test that we can parse a rule.
    #[test]
    fn test_parse_rule() {
        let r = parse_rule("1-3 a")
            .expect("Expected valid rule to be parsed successfully");
        
        assert_eq!(1, r.a);
        assert_eq!(3, r.b);
        assert_eq!('a', r.character);
    }

    // Ensure we raise an appropriate error with an invalid rule.
    #[test]
    fn test_parse_rule_nonsense() {
        let err = parse_rule("NotARule").err().expect("No error returned.");
        assert_eq!("Invalid rule.", err);
    }

    // Ensure we raise an appropriate error with just a range.
    #[test]
    fn test_parse_rule_nochar() {
        let err = parse_rule("1-3").err().expect("No error returned.");
        assert_eq!("Invalid rule.", err);
    }

    // Ensure we raise an error if the range contains a non-integer.
    #[test]
    fn test_parse_rule_invalid_range_most() {
        let err = parse_rule("1-bob a").err().expect("No error returned.");
        assert_eq!("Non-integer in range: bob", err);
    }

    // Ensure we raise an error if the range contains a non-integer.
    #[test]
    fn test_parse_rule_invalid_range_least() {
        let err = parse_rule("alice-7 a").err().expect("No error returned.");
        assert_eq!("Non-integer in range: alice", err);
    }

    // Ensure we raise an error if the first half of the rule is not a range.
    #[test]
    fn test_parse_rule_notarange() {
        let err = parse_rule("fff a").err().expect("No error returned.");
        assert_eq!("First component of rule is not a range.", err);
    }

    // Ensure we raise an error if the second half of the rule is not a single character.
    #[test]
    fn test_parse_rule_multiple_chars() {
        let err = parse_rule("1-3 abc").err().expect("No error returned.");
        assert_eq!("Second component of rule must be a single character.", err);
    }

    // Ensure we raise an error if the rule contains a space but no second half,
    // i.e. the last character is a space.
    #[test]
    fn test_parse_rule_space_at_end() {
        let err = parse_rule("1-3 ").err().expect("No error returned.");
        assert_eq!("Second component of rule must be a single character.", err);
    }
}

#[cfg(test)]
mod tests_password_parsing {
    use super::*;

    // Ensure we raise an appropriate error if the string cannot be parsed
    // into a rule and a password.
    #[test]
    fn test_cannot_parse() {
        let err = parse_password("1-3 a blah").err().expect("No error returned.");
        assert_eq!("Line must contain a rule and password, separated by ':'.", err);
    }

    // Ensure a valid password-rule pair can be parsed.
    #[test]
    fn test_valid() {
        let password_and_rule = parse_password("1-3 a: blah").expect("No error returned.");

        assert_eq!(1, password_and_rule.0.a);
        assert_eq!(3, password_and_rule.0.b);
        assert_eq!('a', password_and_rule.0.character);

        assert_eq!("blah", password_and_rule.1);
    }
}

#[cfg(test)]
mod tests_password_valid {
    use super::*;

    // Test that we mark a password as invalid
    // when there are too few of the required character.
    #[test]
    fn test_too_few() {
        let r = Rule { a: 2, b: 4, character: 'a' };

        let f = |r: &Rule, p: &str| {
            let count = p.matches(r.character).count();

            if count >= r.a && count <= r.b {
                return true;
            }

            return false;
        };

        assert_eq!(false, password_valid(&r, f, "abc"));
    }

    // Test that we mark a password as invalid
    // when there are too many of the required character.
    #[test]
    fn test_too_many() {
        let r = Rule { a: 2, b: 4, character: 'a' };

        let f = |r: &Rule, p: &str| {
            let count = p.matches(r.character).count();

            if count >= r.a && count <= r.b {
                return true;
            }

            return false;
        };

        assert_eq!(false, password_valid(&r, f, "abracadabra"));
    }

    // Test that we mark a password as valid
    // when there are exactly the least number
    // of the required character.
    #[test]
    fn test_exactly_least() {
        let r = Rule { a: 2, b: 4, character: 'd' };

        let f = |r: &Rule, p: &str| {
            let count = p.matches(r.character).count();

            if count >= r.a && count <= r.b {
                return true;
            }

            return false;
        };

        assert_eq!(true, password_valid(&r, f, "dado"));
    }

    // Test that we mark a password as valid
    // when there are exactly the most number
    // of the required character.
    #[test]
    fn test_exactly_most() {
        let r = Rule { a: 2, b: 4, character: 'd' };

        let f = |r: &Rule, p: &str| {
            let count = p.matches(r.character).count();

            if count >= r.a && count <= r.b {
                return true;
            }

            return false;
        };

        assert_eq!(true, password_valid(&r, f, "dadodd"));
    }

    // Test that we mark a password as valid
    // when there are between least and most
    // of the required character.
    #[test]
    fn test_somewhere_inbetween() {
        let r = Rule { a: 2, b: 4, character: 'e' };

        let f = |r: &Rule, p: &str| {
            let count = p.matches(r.character).count();

            if count >= r.a && count <= r.b {
                return true;
            }

            return false;
        };

        assert_eq!(true, password_valid(&r, f, "eee"));
    }

    // Test that we can apply different functions using
    // rules to a password.
    #[test]
    fn test_different_functions() {
        let r = Rule { a: 2, b: 4, character: 'e' };

        let f1 = |r1: &Rule, p1: &str| {
            let count = p1.matches(r1.character).count();

            if count >= r1.a && count <= r1.b {
                return true;
            }

            return false;
        };

        // Valid by first rule but not second.
        assert_eq!(true, password_valid(&r, f1, "ebee"));

        let f2 = |r2: &Rule, p2: &str| {
            let at_a = if r2.a < p2.len() - 1 {
                p2.chars().nth(r2.a - 1).unwrap() == r2.character
            } else {
                false
            };

            let at_b = if r2.b < p2.len() - 1 {
                p2.chars().nth(r2.b - 1).unwrap() == r2.character
            } else {
                false
            };

            if at_a && !at_b {
                return true;
            }

            if at_b && !at_a {
                return true;
            }

            return false;
        };

        // Valid by second rule but not first.
        assert_eq!(true, password_valid(&r, f2, "bedcc"));
    }

    // One of the examples for part 2.
    #[test]
    fn test_example_part2_a() {
        let r = Rule { a: 1, b: 3, character: 'a' };

        let f = |r: &Rule, p: &str| {
            let at_a = if r.a < p.len() - 1 {
                p.chars().nth(r.a - 1).unwrap() == r.character
            } else {
                false
            };

            let at_b = if r.b < p.len() - 1 {
                p.chars().nth(r.b - 1).unwrap() == r.character
            } else {
                false
            };

            if at_a && !at_b {
                return true;
            }

            if at_b && !at_a {
                return true;
            }

            return false;
        };

        assert_eq!(true, password_valid(&r, f, "abcde"));
    }

    // Another of the examples for part 2.
    #[test]
    fn test_example_part2_b() {
        let r = Rule { a: 1, b: 3, character: 'b' };

        let f = |r: &Rule, p: &str| {
            let at_a = if r.a < p.len() - 1 {
                p.chars().nth(r.a - 1).unwrap() == r.character
            } else {
                false
            };

            let at_b = if r.b < p.len() - 1 {
                p.chars().nth(r.b - 1).unwrap() == r.character
            } else {
                false
            };

            if at_a && !at_b {
                return true;
            }

            if at_b && !at_a {
                return true;
            }

            return false;
        };

        assert_eq!(false, password_valid(&r, f, "cdefg"));
    }

    // Another of the examples for part 2.
    #[test]
    fn test_example_part2_c() {
        let r = Rule { a: 2, b: 9, character: 'c' };

        let f = |r: &Rule, p: &str| {
            let at_a = if r.a <= p.len() {
                p.chars().nth(r.a - 1).unwrap() == r.character
            } else {
                false
            };

            let at_b = if r.b <= p.len() {
                p.chars().nth(r.b - 1).unwrap() == r.character
            } else {
                false
            };

            if at_a && !at_b {
                return true;
            }

            if at_b && !at_a {
                return true;
            }

            return false;
        };

        assert_eq!(false, password_valid(&r, f, "ccccccccc"));
    }
}

// Represents a password rule.
//
// A password rule has the following syntax:
// <least>-<most> <char>
//
// Where:
// - <least> is the least number of times <char> can occur
// - <most> is the most number of times <char> can occur
struct Rule {
    a: usize,
    b: usize,
    character: char,
}

fn parse_rule(rule: &str) -> Result<Rule, String> {
    // We expect the rule to have a range and a char, separated by space.
    let space_index = match rule.find(' ') {
        Some(i) => i,
        None => return Err(String::from("Invalid rule."))
    };

    let split = rule.split_at(space_index);

    // Handle the first part of the rule - the range.
    let range_sep_index = match split.0.find('-') {
        Some(i) => i,
        None => return Err(String::from("First component of rule is not a range."))
    };

    let split_range = split.0.split_at(range_sep_index);

    let a = match split_range.0.parse::<usize>() {
        Ok(n) => n,
        Err(_) => return Err(String::from(format!("Non-integer in range: {}", split_range.0)))
    };

    // Split includes the separator in the second half of the string.
    let b = match split_range.1[1..].parse::<usize>() {
        Ok(n) => n,
        Err(_) => return Err(String::from(format!("Non-integer in range: {}", &split_range.1[1..])))
    };

    // Now get the character for the rule.
    // This is the bit after the space-separator.
    let character_string = &split.1[1..];

    if character_string.len() != 1 {
        return Err(String::from("Second component of rule must be a single character."));
    }

    // We can be sure that there is exactly one character in the string,
    // so this is perfectly safe.
    let character = character_string.chars().nth(0).unwrap();

    return Ok(Rule { a, b, character: character });
}

// Parses a rule-password pair.
fn parse_password(s: &str) -> Result<(Rule, String), String> {
    let split_index = match s.find(':') {
        Some(i) => i,
        None => return Err(String::from("Line must contain a rule and password, separated by ':'."))
    };

    let split = s.split_at(split_index);
    let rule_string = split.0;
    
    let rule = match parse_rule(rule_string) {
        Ok(r) => r,
        Err(e) => return Err(e)
    };

    let password = split.1[1..].trim();

    return Ok((rule, String::from(password)));
}

// Tests a particular password using the function provided
// against a given rule, returning whether the password matches or not.
fn password_valid<T>(r: &Rule, f: T, p: &str) -> bool
    where T: Fn(&Rule, &str) -> bool {
    return f(r, p);
}

fn main() {
    // Read test data into vector.
    let mut v: Vec<String> = Vec::new();
    
    // Read test data in, iterate over each line.
    let f = File::open("data/day2.txt").expect("Could not open data/day2.txt");
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.expect("Invalid line in data/day2.txt");

        v.push(line);
    }

    // Count number of valid passwords.
    let mut valid_passwords = 0;

    let f = |r: &Rule, p: &str| {
        let count = p.matches(r.character).count();

        if count >= r.a && count <= r.b {
            return true;
        }

        return false;
    };

    for line in &v {
        let (r, p) = match parse_password(&line) {
            Ok((r, p)) => (r, p),
            Err(_) => panic!(format!("Invalid password: {}", line))
        };

        if password_valid(&r, f, &p) {
            valid_passwords += 1;
        }
    }

    println!("Part 1: Number of valid passwords is: {}", valid_passwords);

    let f = |r: &Rule, p: &str| {
        let at_a = if r.a <= p.len() {
            p.chars().nth(r.a - 1).unwrap() == r.character
        } else {
            false
        };

        let at_b = if r.b <= p.len() {
            p.chars().nth(r.b - 1).unwrap() == r.character
        } else {
            false
        };

        if at_a && !at_b {
            return true;
        }

        if at_b && !at_a {
            return true;
        }

        false
    };

    valid_passwords = 0;

    for line in &v {
        let (r, p) = match parse_password(&line) {
            Ok((r, p)) => (r, p),
            Err(_) => panic!(format!("Invalid password: {}", line))
        };

        if password_valid(&r, f, &p) {
            valid_passwords += 1;
        }
    }

    println!("Part 2: Number of valid passwords is: {}", valid_passwords);
}
