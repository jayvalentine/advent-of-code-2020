// Advent of Code 2020
// Day 1

use std::collections::HashMap;

use std::fs::File;
use std::io::{prelude::*, BufReader};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let n = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(514579, find_match(&n));
    }
}

// Given an array of numbers, find the two numbers that sum to 2020
// and return their product.
fn find_match(n: &[u32]) -> u32 {
    let mut map: HashMap<u32, bool> = HashMap::new();

    // Put each number into the hashmap.
    for i in n {
        map.insert(*i, true);
    }

    // Now iterate over each key in the hashmap.
    // For each key, check to see if it's opposite
    // (i.e. the value with which it sums to 2020)
    // exists.
    //
    // If so, multiply them and return.
    for (k, _v) in &map {
        let left = *k;
        let right = 2020 - left;

        if map.contains_key(&right) {
            return left * right;
        }
    }

    return 0;
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
}
