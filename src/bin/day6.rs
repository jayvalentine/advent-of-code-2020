// Advent of Code 2020
// Day 6

use std::collections::HashSet;

use std::fs::File;
use std::io::{prelude::*, BufReader};

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn test_group_one_person() {
        let group = "abc";
        let answers: HashSet<char> = group_answers(group);

        assert_eq!(true, answers.contains(&'a'));
        assert_eq!(true, answers.contains(&'b'));
        assert_eq!(true, answers.contains(&'c'));

        assert_eq!(false, answers.contains(&'d'));
        assert_eq!(false, answers.contains(&'e'));
        assert_eq!(false, answers.contains(&'f'));

        assert_eq!(false, answers.contains(&'z'));
    }

    #[test]
    fn test_group_three_people_different_answers() {
        let group = "a\nb\nc";
        let answers: HashSet<char> = group_answers(group);

        assert_eq!(true, answers.contains(&'a'));
        assert_eq!(true, answers.contains(&'b'));
        assert_eq!(true, answers.contains(&'c'));

        assert_eq!(false, answers.contains(&'d'));
        assert_eq!(false, answers.contains(&'e'));
        assert_eq!(false, answers.contains(&'f'));

        assert_eq!(false, answers.contains(&'z'));
    }

    #[test]
    fn test_group_two_people_with_overlap() {
        let group = "ab\nac";
        let answers: HashSet<char> = group_answers(group);

        assert_eq!(true, answers.contains(&'a'));
        assert_eq!(true, answers.contains(&'b'));
        assert_eq!(true, answers.contains(&'c'));

        assert_eq!(false, answers.contains(&'d'));
        assert_eq!(false, answers.contains(&'e'));
        assert_eq!(false, answers.contains(&'f'));

        assert_eq!(false, answers.contains(&'z'));
    }

    #[test]
    fn test_group_four_people_all_same() {
        let group = "a\na\na\na";
        let answers: HashSet<char> = group_answers(group);

        assert_eq!(true, answers.contains(&'a'));
        assert_eq!(false, answers.contains(&'b'));
        assert_eq!(false, answers.contains(&'c'));
    }

    #[test]
    fn test_group_one_person_one_answer() {
        let group = "b";
        let answers: HashSet<char> = group_answers(group);

        assert_eq!(true, answers.contains(&'b'));
        assert_eq!(false, answers.contains(&'a'));
        assert_eq!(false, answers.contains(&'c'));
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
        assert_eq!(6633, answer);
    }
}

fn group_answers(group: &str) -> HashSet<char> {
    let mut s: HashSet<char> = HashSet::new();

    for l in group.trim().lines() {
        for c in l.chars() {
            if ('a'..='z').contains(&c) {
                s.insert(c);
            } else {
                panic!(format!("Invalid character {} in group:\n{}", c, group));
            }
        }
    }

    return s;
}

fn part1() -> usize {
    // Read test data in, iterate over each line.
    let f = File::open("data/day6.txt").expect("Could not open data/day6.txt");
    let reader = BufReader::new(f);

    let mut current_group = String::new();
    let mut answers_total = 0;

    for line in reader.lines() {
        let line = line.expect("Invalid line in data/day6.txt");

        // If we've hit a blank line, that's the end of the current
        // group. Check it if we've collected something.
        //
        // Otherwise this isn't a blank line, and is a continuation
        // of the current group.
        if line.trim().is_empty() && !current_group.is_empty() {
            let answers = group_answers(&current_group);
            answers_total += answers.len();

            current_group.clear();
        } else {
            current_group.push('\n');
            current_group.push_str(&line);
        }
    }

    return answers_total;
}

fn main() {
    let answer_part1 = part1();
    println!("Part 1: The sum of all positive answers is: {}", answer_part1);
}