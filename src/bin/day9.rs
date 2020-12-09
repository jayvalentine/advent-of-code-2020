// Advent of Code 2020
// Day 9

use std::collections::HashSet;
use std::iter::{FromIterator, Sum};

use std::fs;

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = "
        35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";

        let input = input.trim().split('\n').map(|i| {
            i.trim().parse().unwrap()
        });
    
        let input: Vec<u64> = Vec::from_iter(input);

        let invalid = first_invalid(&input, 5);
        assert_eq!(127, invalid);
    }

    #[test]
    fn test_example_part2() {
        let input = "
        35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";

        let input = input.trim().split('\n').map(|i| {
            i.trim().parse().unwrap()
        });
    
        let input: Vec<u64> = Vec::from_iter(input);

        let (min, max) = find_weakness(&input, 127);
        assert_eq!(15, min);
        assert_eq!(47, max);
    }
}

#[cfg(test)]
mod test_is_valid {
    use super::*;

    #[test]
    fn positive() {
        let values = [1, 2, 3, 4, 5, 6];
        assert_eq!(true, is_valid(&values, 5, 5));
    }

    #[test]
    fn positive_after_preamble() {
        let values = [1, 2, 3, 4, 5, 6, 10];
        assert_eq!(true, is_valid(&values, 6, 5));
    }

    #[test]
    fn positive_from_example() {
        let values = [35, 20, 15, 25, 47, 40];
        assert_eq!(true, is_valid(&values, 5, 5));
    }

    #[test]
    fn negative() {
        let values = [1, 2, 3, 4, 5, 2];
        assert_eq!(false, is_valid(&values, 5, 5));
    }

    #[test]
    fn negative_after_preamble() {
        let values = [1, 2, 3, 4, 5, 6, 4];
        assert_eq!(false, is_valid(&values, 6, 5));
    }
}

#[cfg(test)]
mod test_puzzle {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(85848519, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(13414198, part2());
    }
}

fn is_valid(values: &[u64], i: usize, preamble_count: usize) -> bool {
    // Make sure that i is in range:
    // - is >= the preamble count
    // - is < the length of values
    if (i < preamble_count) || (i >= values.len()) {
        panic!("Index out of range: {}", i);
    }

    let value = *values.get(i).unwrap();

    let preamble_start = i - preamble_count;

    // Set of items in preamble.
    let mut preamble: HashSet<u64> = HashSet::new();
    for n in values[preamble_start..i].iter() {
        preamble.insert(*n);
    }

    // For each item in the set, subtract it from the input.
    // If the result is also in the set, this is valid!
    for &p in &preamble {
        // Skip if this element is equal to or larger than our value,
        // as it can't possibly be part of a sum.
        if p >= value {
            continue;
        }

        // Subtract from value, check to see if other half
        // is in the set.
        let difference = value - p;

        // If the difference is equal to this element,
        // then it isn't valid.
        if difference == p {
            continue;
        }
        
        if preamble.contains(&difference) {
            return true;
        }
    }

    return false;
}

fn find_weakness(input: &[u64], invalid: u64) -> (u64, u64) {
    // Vector to hold our contiguous set that sum to invalid.
    let mut s: Vec<u64> = Vec::new();

    for (pos, &i) in input.iter().enumerate() {
        // Skip if >= invalid, because it can't then
        // be part of a set of at least 2 numbers
        // summing to invalid.
        if i >= invalid {
            continue;
        }

        s.push(i);

        // Can this be part of a set?
        for &j in input[pos+1..].iter() {
            // Add to set.
            s.push(j);

            // If sum(set) is now == invalid, return the min and max.
            // Otherwise, if sum(set) > invalid, clear it and break.
            // Otherwise, continue.
            let sum = s.iter().sum::<u64>();

            if sum == invalid {
                return (*s.iter().min().unwrap(), *s.iter().max().unwrap());
            } else if sum > invalid {
                s.clear();
                break;
            }
        }
    }

    panic!("Could not find contiguous set!");
}

fn first_invalid(input: &[u64], preamble_count: usize) -> u64 {
    for i in preamble_count..input.len() {
        if !is_valid(&input, i, preamble_count) {
            return input[i];
        }
    }

    panic!("Could not find invalid value!");
}

fn part1() -> u64 {
    let input = fs::read_to_string("data/day9.txt")
        .expect("Could not read data/day9.txt");

    let input = input.trim().split('\n').map(|i| {
        i.trim().parse().unwrap()
    });

    let input: Vec<u64> = Vec::from_iter(input);

    return first_invalid(&input, 25);
}

fn part2() -> u64 {
    let input = fs::read_to_string("data/day9.txt")
        .expect("Could not read data/day9.txt");

    let input = input.trim().split('\n').map(|i| {
        i.trim().parse().unwrap()
    });

    let input: Vec<u64> = Vec::from_iter(input);

    let invalid = first_invalid(&input, 25);

    let (min, max) = find_weakness(&input, invalid);
    return min + max;
}

fn main() {
    let invalid_part1 = part1();
    println!("Part 1: The first invalid number in the list is: {}", invalid_part1);

    let weakness_part2 = part2();
    println!("Part 2: The weakness is: {}", weakness_part2);
}
