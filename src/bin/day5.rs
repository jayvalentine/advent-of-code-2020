// Advent of Code 2020
// Day 5

use std::collections::HashSet;

use std::fs::File;
use std::io::{prelude::*, BufReader};

#[cfg(test)]
mod test_examples_part1 {
    use super::*;

    #[test]
    fn test_example1() {
        let (row, column) = search_seat("BFFFBBFRRR");
        assert_eq!(70, row);
        assert_eq!(7, column);
    }

    #[test]
    fn test_example2() {
        let (row, column) = search_seat("FFFBBBFRRR");
        assert_eq!(14, row);
        assert_eq!(7, column);
    }

    #[test]
    fn test_example3() {
        let (row, column) = search_seat("BBFFBBFRLL");
        assert_eq!(102, row);
        assert_eq!(4, column);
    }
}

#[cfg(test)]
mod test_search_row {
    use super::*;

    #[test]
    fn test_first() {
        let rows: Vec<u32> = (0..128).collect();
        let row = search_row(&rows, "FFFFFFF");
        assert_eq!(0, row);
    }

    #[test]
    fn test_last() {
        let rows: Vec<u32> = (0..128).collect();
        let row = search_row(&rows, "BBBBBBB");
        assert_eq!(127, row);
    }

    #[test]
    fn test_example() {
        let rows: Vec<u32> = (0..128).collect();
        let row = search_row(&rows, "FBFBBFF");
        assert_eq!(44, row);
    }
}

#[cfg(test)]
mod test_search_column {
    use super::*;

    #[test]
    fn test_left() {
        let columns: Vec<u32> = (0..8).collect();
        let column = search_column(&columns, "LLL");
        assert_eq!(0, column);
    }

    #[test]
    fn test_right() {
        let columns: Vec<u32> = (0..8).collect();
        let column = search_column(&columns, "RRR");
        assert_eq!(7, column);
    }

    #[test]
    fn test_example() {
        let columns: Vec<u32> = (0..8).collect();
        let column = search_column(&columns, "RLR");
        assert_eq!(5, column);
    }
}

fn search_column(v: &[u32], pattern: &str) -> u32 {
    return search(v, pattern, 'L', 'R');
}

fn search_row(v: &[u32], pattern: &str) -> u32 {
    return search(v, pattern, 'F', 'B');
}

fn search(v: &[u32], pattern: &str, first: char, last: char) -> u32 {
    let direction = pattern.chars().next().unwrap();

    let search_range = if direction == first {
        &v[..v.len()/2]
    } else if direction == last {
        &v[v.len()/2..]
    } else {
        panic!("Invalid character in search pattern")
    };

    if search_range.len() == 1 {
        return *search_range.get(0).unwrap();
    } else {
        return search(search_range, &pattern[1..], first, last);
    }
}

fn search_seat(s: &str) -> (u32, u32) {
    let rows: Vec<u32> = (0..128).collect();
    let row = search_row(&rows, &s[..7]);

    let columns: Vec<u32> = (0..8).collect();
    let column = search_column(&columns, &s[7..]);
    (row, column)
}

fn main() {
    // Read test data in, iterate over each line.
    let f = File::open("data/day5.txt").expect("Could not open data/day5.txt");
    let reader = BufReader::new(f);

    let mut highest_id: u32 = 0;

    // Hashset to store IDs to search for part 2.
    let mut ids: HashSet<u32> = HashSet::new();

    for line in reader.lines() {
        let line = line.expect("Invalid line in data/day5.txt");

        let (row, column) = search_seat(line.trim());
        let id = row * 8 + column;
        if id > highest_id { highest_id = id; }

        ids.insert(id);
    }

    println!("Part 1: The highest boarding pass ID is: {}", highest_id);

    // Part 2:
    // We need to find the ID of a missing seat that is not at the front
    // or back of the plane. We know that ID+1 and ID-1 exist, so we can search
    // the set for a seat with ID X where X+1 doesn't exist but X+2 does.

    let mut possible: Vec<u32> = Vec::new();

    for id in &ids {
        let id = *id;
        if ids.contains(&(id+1)) {
            continue;
        }

        if ids.contains(&(id+2)) {
            possible.push(id+1);
        }
    }

    if possible.len() != 1 {
        panic!("Missing seat ID is ambiguous");
    }

    println!("Part 2: Missing seat ID is: {}", possible.get(0).unwrap());
}
