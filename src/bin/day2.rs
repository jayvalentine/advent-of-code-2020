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
        
        assert_eq!(1, r.least);
        assert_eq!(3, r.most);
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

        assert_eq!(1, password_and_rule.0.least);
        assert_eq!(3, password_and_rule.0.most);
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
        let r = Rule { least: 2, most: 4, character: 'a' };
        assert_eq!(false, password_valid(&r, "abc"));
    }

    // Test that we mark a password as invalid
    // when there are too many of the required character.
    #[test]
    fn test_too_many() {
        let r = Rule { least: 2, most: 4, character: 'a' };
        assert_eq!(false, password_valid(&r, "abracadabra"));
    }

    // Test that we mark a password as valid
    // when there are exactly the least number
    // of the required character.
    #[test]
    fn test_exactly_least() {
        let r = Rule { least: 2, most: 4, character: 'd' };
        assert_eq!(true, password_valid(&r, "dado"));
    }

    // Test that we mark a password as valid
    // when there are exactly the most number
    // of the required character.
    #[test]
    fn test_exactly_most() {
        let r = Rule { least: 2, most: 4, character: 'd' };
        assert_eq!(true, password_valid(&r, "dadodd"));
    }

    // Test that we mark a password as valid
    // when there are between least and most
    // of the required character.
    #[test]
    fn test_somewhere_inbetween() {
        let r = Rule { least: 2, most: 4, character: 'e' };
        assert_eq!(true, password_valid(&r, "eee"));
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
    least: usize,
    most: usize,
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

    let least = match split_range.0.parse::<usize>() {
        Ok(n) => n,
        Err(_) => return Err(String::from(format!("Non-integer in range: {}", split_range.0)))
    };

    // Split includes the separator in the second half of the string.
    let most = match split_range.1[1..].parse::<usize>() {
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

    return Ok(Rule { least, most, character: character });
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

// Tests a particular password against the given rule for validity.
fn password_valid(r: &Rule, p: &str) -> bool {
    let count = p.matches(r.character).count();

    if count >= r.least && count <= r.most {
        return true;
    }

    return false;
}

fn main() {
    // Read test data in, iterate over each line.
    let f = File::open("data/day2.txt").expect("Could not open data/day2.txt");
    let reader = BufReader::new(f);

    // Count number of valid passwords.
    let mut valid_passwords = 0;

    for line in reader.lines() {
        let line = line.expect("Invalid line in data/day2.txt");

        let (r, p) = match parse_password(&line) {
            Ok((r, p)) => (r, p),
            Err(_) => panic!(format!("Invalid password: {}", line))
        };

        if password_valid(&r, &p) {
            valid_passwords += 1;
        }
    }

    println!("Part 1: Number of valid passwords is: {}", valid_passwords);
}
