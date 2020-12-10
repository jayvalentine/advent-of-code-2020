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

fn main() {
    let answer_part1 = part1();
    println!("The answer to part 1 is: {}", answer_part1);
}