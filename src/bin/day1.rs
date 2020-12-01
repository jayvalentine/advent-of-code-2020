// Advent of Code 2020
// Day 1

use std::collections::HashSet;

use std::fs::File;
use std::io::{prelude::*, BufReader};

#[cfg(test)]
mod tests {
    use super::*;

    // First example - find product of two numbers in list
    // which sum to 2020.
    #[test]
    fn example() {
        let n = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(514579, find_match(&n, 2, 2020));
    }

    // Second example - find product of three numbers in list
    // which sum to 2020.
    #[test]
    fn example_three() {
        let n = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(241861950, find_match(&n, 3, 2020));
    }
}

// Given an array of numbers <n>, find <group_size> numbers
// that sum to <sum> and return their product.
fn find_match(n: &[u32], group_size: u32, sum: u32) -> u32 {
    let mut map: HashSet<u32> = HashSet::new();

    // Put each number into the hashmap.
    for i in n {
        if map.contains(i) {
            panic!("Warning: duplicate: {}", i);
        }

        map.insert(*i);
    }
    
    return find_match_inner(&map, group_size, sum).expect("Combination not found");
}

fn find_match_inner(s: &HashSet<u32>, group_size: u32, sum: u32) -> Option<u32> {
    // Now iterate over each key in the hashmap.
    // For each key, check to see if it's opposite
    // (i.e. the value with which it sums to 2020)
    // exists.
    //
    // If so, multiply them and return.
    for k in s {
        // If this value is > sum then it isn't a candidate.
        if *k > sum {
            continue;
        }

        let left = *k;
        let right = sum - left;

        // Base case - if we have to find two numbers
        // that sum, we've just done it.
        //
        // Recursive case - we need to find (<group_size>-1)
        // numbers that sum to <right>.
        if group_size == 2 && s.contains(&right) {
            return Some(left * right);
        }
        else if group_size > 2 {
            // Create a new set without the element we're checking.
            let mut s_new = s.clone();
            s_new.remove(&left);

            let result = find_match_inner(&s_new, group_size - 1, right);
            match result {
                Some(val) => return Some(left * val),
                None => continue
            }
        }
    }

    return None;
}

fn main() {
    // Vector to hold test data.
    let mut v: Vec<u32> = Vec::new();

    // Read test data in, iterate over each line.
    let f = File::open("data/day1.txt").expect("Could not open data/day1.txt");
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.expect("Invalid line in data/day1.txt");

        let n: u32 = line.trim().parse().expect("Non-number in data/day1.txt");
        v.push(n);
    }

    println!("Data size: {}", v.len());

    // Find the match in the data.
    let product = find_match(&v, 2, 2020);
    println!("Part 1: The answer is: {}", product);

    let product_three = find_match(&v, 3, 2020);
    println!("Part 2: The answer is: {}", product_three);

}
