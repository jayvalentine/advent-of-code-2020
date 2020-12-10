// Advent of Code 2020
// Day 10

use std::collections::HashMap;
use std::iter::{FromIterator};

use std::fs;

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let mut adapters: [u32; 11] = [
            16,
            10,
            15,
            5,
            1,
            11,
            7,
            19,
            6,
            12,
            4,
        ];

        let distribution = get_distribution(&mut adapters);
        assert_eq!(7, *distribution.get(&1).unwrap());
        assert_eq!(0, *distribution.get(&2).unwrap());
        assert_eq!(5, *distribution.get(&3).unwrap());
    }

    #[test]
    fn test_part2_example1() {
        let mut adapters: [u32; 11] = [
            16,
            10,
            15,
            5,
            1,
            11,
            7,
            19,
            6,
            12,
            4,
        ];

        let arrangements = get_arrangements(&mut adapters);
        assert_eq!(8, arrangements);
    }

    #[test]
    fn test_part1_example2() {
        let mut adapters: [u32; 31] = [
            28,
            33,
            18,
            42,
            31,
            14,
            46,
            20,
            48,
            47,
            24,
            23,
            49,
            45,
            19,
            38,
            39,
            11,
            1,
            32,
            25,
            35,
            8,
            17,
            7,
            9,
            4,
            2,
            34,
            10,
            3,
        ];

        let distribution = get_distribution(&mut adapters);
        assert_eq!(22, *distribution.get(&1).unwrap());
        assert_eq!(0, *distribution.get(&2).unwrap());
        assert_eq!(10, *distribution.get(&3).unwrap());
    }

    #[test]
    fn test_part2_example2() {
        let mut adapters: [u32; 31] = [
            28,
            33,
            18,
            42,
            31,
            14,
            46,
            20,
            48,
            47,
            24,
            23,
            49,
            45,
            19,
            38,
            39,
            11,
            1,
            32,
            25,
            35,
            8,
            17,
            7,
            9,
            4,
            2,
            34,
            10,
            3,
        ];

        let arrangements = get_arrangements(&mut adapters);
        assert_eq!(19208, arrangements);
    }
}

#[cfg(test)]
mod test_puzzles {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1820, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(3454189699072, part2());
    }
}

fn get_arrangements(adapters: &mut [u32]) -> u64 {
    let mut adapters = Vec::from_iter(adapters.iter().map(|i| { *i }));
    adapters.push(0);
    adapters.sort();

    let mut known: HashMap<(u32, usize), u64> = HashMap::new();

    return get_arrangements_inner(&mut known, &mut adapters, 0);
}

fn get_arrangements_inner(known: &mut HashMap<(u32, usize), u64>, adapters: &mut Vec<u32>, start: usize) -> u64 {
    // Length of the list that we have left, and value of the first
    // item in that sub-list. These two values can be used to uniquely
    // identify that particular sublist, so that we can cache the number of
    // permutations for later use.
    let length = adapters.len() - start;
    let start_val = adapters[start];

    // Return the cached value if we have it.
    if known.contains_key(&(start_val, length)) {
        return *known.get(&(start_val, length)).unwrap();
    }

    let mut arrangements = 0;

    // For each element i in the list (including both the outlet and device),
    // for which pos(i) + 2 is within the bounds of the list,
    // if the element at pos(i) + 2 is within 3 jolts of i,
    // the element at pos(i) + 1 can be removed, and the chain
    // still works.
    //
    // Therefore we iterate over the list.
    // Each time we encounter such an element,
    // we take a branch - one where we remove it
    // and one where we don't, and count the ways
    // that sub-list can be arranged.
    for pos in start..adapters.len()-2 {
        let i = adapters[pos];

        if adapters[pos+2] - i <= 3 {
            // Remove the middle item from the vector.
            let removed = adapters.remove(pos+1);

            // How many ways is it possible to arrange the new list?
            arrangements += get_arrangements_inner(known, adapters, pos);

            adapters.insert(pos+1, removed);
        }
    }

    // Insert into the map of known items.
    known.insert((start_val, length), arrangements + 1);

    return arrangements + 1;
}

fn get_distribution(adapters: &mut [u32]) -> HashMap<u32, u32> {
    // Sort list of adapters.
    adapters.sort();

    // Distribution of differences between adapter jolts.
    let mut distribution: HashMap<u32, u32> = HashMap::new();
    distribution.insert(1, 0);
    distribution.insert(2, 0);
    
    // Our device is always +3 from the highest adapter value.
    // So there is always at least one occurance of a difference of 3.
    distribution.insert(3, 1);

    let mut current = 0;

    // Sort the list of adapters and iterate over them.
    for &a in adapters.iter() {
        let difference = a - current;

        if difference < 1 || difference > 3 {
            panic!("Could not find a valid adapter for value: {}", current);
        }

        *(distribution.get_mut(&difference).unwrap()) += 1;

        current = a;
    }

    return distribution;
}

// Given the puzzle input (data/day10.txt)
// return the number of 1-jolt differences multiplied
// by the number of 3-jolt differences.
fn part1() -> u32 {
    let input = fs::read_to_string("data/day10.txt")
        .expect("Could not read data/day10.txt");

    let mut input = Vec::from_iter(input.split('\n').map(|i| {
        i.trim().parse::<u32>()
            .expect(&format!("Could not parse u32 in input: {}", i))
    }).into_iter());

    let distribution = get_distribution(&mut input);

    let num_1 = *distribution.get(&1).unwrap();
    let num_3 = *distribution.get(&3).unwrap();

    return num_1 * num_3;
}

// Given the puzzle input (data/day10.txt)
// return the number of 1-jolt differences multiplied
// by the number of 3-jolt differences.
fn part2() -> u64 {
    let input = fs::read_to_string("data/day10.txt")
        .expect("Could not read data/day10.txt");

    let mut input = Vec::from_iter(input.split('\n').map(|i| {
        i.trim().parse::<u32>()
            .expect(&format!("Could not parse u32 in input: {}", i))
    }).into_iter());

    return get_arrangements(&mut input);
}

fn main() {
    let answer_part1 = part1();
    println!("The answer to part 1 is: {}", answer_part1);

    let answer_part2 = part2();
    println!("The answer to part 2 is: {}", answer_part2);
}