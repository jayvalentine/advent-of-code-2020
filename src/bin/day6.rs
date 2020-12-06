// Advent of Code 2020
// Day 6

use std::collections::HashSet;

use aoc::file::*;

#[cfg(test)]
mod test_examples_part1 {
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

#[cfg(test)]
mod test_examples_part2 {
    use super::*;

    #[test]
    fn test_group_one_person() {
        let group = "abc";
        let answers: HashSet<char> = group_answers_intersection(group);

        assert_eq!(3, answers.len());

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
        let answers: HashSet<char> = group_answers_intersection(group);

        assert_eq!(0, answers.len());

        assert_eq!(false, answers.contains(&'a'));
        assert_eq!(false, answers.contains(&'b'));
        assert_eq!(false, answers.contains(&'c'));

        assert_eq!(false, answers.contains(&'d'));
        assert_eq!(false, answers.contains(&'e'));
        assert_eq!(false, answers.contains(&'f'));

        assert_eq!(false, answers.contains(&'z'));
    }

    #[test]
    fn test_group_two_people_with_overlap() {
        let group = "ab\nac";
        let answers: HashSet<char> = group_answers_intersection(group);

        assert_eq!(1, answers.len());

        assert_eq!(true, answers.contains(&'a'));

        assert_eq!(false, answers.contains(&'b'));
        assert_eq!(false, answers.contains(&'c'));

        assert_eq!(false, answers.contains(&'d'));
        assert_eq!(false, answers.contains(&'e'));
        assert_eq!(false, answers.contains(&'f'));

        assert_eq!(false, answers.contains(&'z'));
    }

    #[test]
    fn test_group_four_people_all_same() {
        let group = "a\na\na\na";
        let answers: HashSet<char> = group_answers_intersection(group);

        assert_eq!(1, answers.len());

        assert_eq!(true, answers.contains(&'a'));
        assert_eq!(false, answers.contains(&'b'));
        assert_eq!(false, answers.contains(&'c'));
    }

    #[test]
    fn test_group_one_person_one_answer() {
        let group = "b";
        let answers: HashSet<char> = group_answers_intersection(group);

        assert_eq!(1, answers.len());

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

    #[test]
    fn test_part2() {
        let answer = part2();
        assert_eq!(3202, answer);
    }
}

fn group_answers(group: &str) -> HashSet<char> {
    let mut s: HashSet<char> = HashSet::new();

    for l in group.trim().lines() {
        let l = l.trim();

        for c in l.chars() {
            if ('a'..='z').contains(&c) {
                s.insert(c);
            } else {
                panic!(format!("Invalid character {} in group:\n{}", c, group));
            }
        }
    }

    s
}

fn group_answers_intersection(group: &str) -> HashSet<char> {
    // Start with the set of all possible answers.
    // We will exclude from this set as we go.
    let mut answers: HashSet<char> = HashSet::from(('a'..='z').collect());

    for l in group.trim().lines() {
        let mut s: HashSet<char> = HashSet::new();
        for c in l.chars() {
            if ('a'..='z').contains(&c) {
                s.insert(c);
            } else {
                panic!(format!("Invalid character {} in group:\n{}", c, group));
            }
        }

        answers = answers.intersection(&s).map(|c| { *c }).collect::<HashSet<char>>();
    }

    answers
}

fn part1() -> usize {
    let mut f = GroupedFileReader::open("data/day6.txt").expect("Could not open puzzle data.");

    let mut answers_total = 0;

    while let FileReadResult::Success(group) = f.next_group() {
        let answers = group_answers(&group);
        answers_total += answers.len();
    }

    answers_total
}

fn part2() -> usize {
    let mut f = GroupedFileReader::open("data/day6.txt").expect("Could not open puzzle data.");

    let mut answers_total = 0;

    while let FileReadResult::Success(group) = f.next_group() {
        let answers = group_answers_intersection(&group);
        answers_total += answers.len();
    }

    answers_total
}

fn main() {
    let answer_part1 = part1();
    println!("Part 1: The sum of all positive answers is: {}", answer_part1);

    let answer_part2 = part2();
    println!("Part 2: The sum of the size of the intersection of each group's answers is: {}", answer_part2);
}