// Advent of Code 2020
// Day 14

extern crate regex;
use regex::Regex;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::fs;

type Memory = HashMap<u64, u64>; 

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
        mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 0";

        let memory = interpret(input);

        assert_eq!(64, *memory.get(&8).unwrap());
        assert_eq!(101, *memory.get(&7).unwrap())
    }

    #[test]
    fn test_part2() {
        let input = "
        mask = 000000000000000000000000000000X1001X
        mem[42] = 100
        mask = 00000000000000000000000000000000X0XX
        mem[26] = 1";

        let memory = interpret_part2(input);
        
        assert_eq!(10, memory.len());

        assert_eq!(100, *memory.get(&58).unwrap());
        assert_eq!(100, *memory.get(&59).unwrap());

        assert_eq!(1, *memory.get(&16).unwrap());
        assert_eq!(1, *memory.get(&17).unwrap());
        assert_eq!(1, *memory.get(&18).unwrap());
        assert_eq!(1, *memory.get(&19).unwrap());
        assert_eq!(1, *memory.get(&24).unwrap());
        assert_eq!(1, *memory.get(&25).unwrap());
        assert_eq!(1, *memory.get(&26).unwrap());
        assert_eq!(1, *memory.get(&27).unwrap());
    }
}

#[cfg(test)]
mod test_bitmask {
    use super::*;

    #[test]
    fn test_apply1() {
        let mask = Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
        assert_eq!(73, mask.apply(11));
        assert_eq!(101, mask.apply(101));
    }

    #[test]
    fn test_apply2() {
        let mask = Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX").unwrap();
        assert_eq!(11, mask.apply(11));
        assert_eq!(123, mask.apply(123));
        assert_eq!(0, mask.apply(0));
    }

    #[test]
    fn test_apply3() {
        let mask = Mask::from_str("111111111111111111111111111111111111").unwrap();
        assert_eq!(68719476735, mask.apply(11));
        assert_eq!(68719476735, mask.apply(123));
        assert_eq!(68719476735, mask.apply(0));
    }

    #[test]
    fn test_apply4() {
        let mask = Mask::from_str("000000000000000000000000000000000000").unwrap();
        assert_eq!(0, mask.apply(11));
        assert_eq!(0, mask.apply(123));
        assert_eq!(0, mask.apply(0));
    }

    #[test]
    fn test_apply_default() {
        let mask = Mask::default();
        assert_eq!(11, mask.apply(11));
        assert_eq!(123, mask.apply(123));
        assert_eq!(0, mask.apply(0));
    }
}

#[cfg(test)]
mod test_bitmask_address {
    use super::*;

    #[test]
    fn test_apply1() {
        let mask = Mask::from_str("000000000000000000000000000000X1001X").unwrap();
        let addresses = mask.addresses(42);

        assert_eq!(4, addresses.len());

        assert_eq!(true, addresses.contains(&26));
        assert_eq!(true, addresses.contains(&27));
        assert_eq!(true, addresses.contains(&58));
        assert_eq!(true, addresses.contains(&59));
    }

    #[test]
    fn test_apply2() {
        let mask = Mask::from_str("00000000000000000000000000000000X0XX").unwrap();
        let addresses = mask.addresses(26);

        assert_eq!(8, addresses.len());

        assert_eq!(true, addresses.contains(&16));
        assert_eq!(true, addresses.contains(&17));
        assert_eq!(true, addresses.contains(&18));
        assert_eq!(true, addresses.contains(&19));

        assert_eq!(true, addresses.contains(&24));
        assert_eq!(true, addresses.contains(&25));
        assert_eq!(true, addresses.contains(&26));
        assert_eq!(true, addresses.contains(&27));
    }

    #[test]
    fn test_apply_default() {
        let mask = Mask::default_address();
        let addresses = mask.addresses(42);

        assert_eq!(1, addresses.len());

        assert_eq!(true, addresses.contains(&42));

        assert_eq!(false, addresses.contains(&26));
        assert_eq!(false, addresses.contains(&27));
        assert_eq!(false, addresses.contains(&58));
        assert_eq!(false, addresses.contains(&59));
    }
}

#[cfg(test)]
mod test_puzzles {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(2346881602152, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(3885232834169, part2());
    }
}

// A "mask" is actually a set of two bitmasks.
// One bitmask is the "set" mask; this is ORed with the input.
// The other is the "reset" mask; this is ANDed with the input.
struct Mask {
    set: u64,   // Holds a 1 in any bit to be set, 0 elsewhere.
    reset: u64  // Holds a 0 in any bit to be reset, 1 elsewhere.
}

impl Mask {
    fn default() -> Mask {
        return Mask {
            set: 0,
            reset: u64::MAX
        };
    }

    fn default_address() -> Mask {
        return Mask {
            set: 0,
            reset: 0
        };
    }

    fn from_str(s: &str) -> Result<Mask, String> {
        // Return error if the string length is not
        // exactly 36.
        if s.len() != 36 {
            return Err(String::from("Invalid mask length"))
        }

        for c in s.chars() {
            if c != 'X' && c != '1' && c != '0' {
                return Err(format!("Invalid character in mask: {}", c));
            }
        }

        // Build the masks.
        let mut set = 0;
        let mut reset = 0;

        // Iterate over each character in the mask.
        // For an X, put a 0 in the "set" mask and a 1 in the "reset"
        // mask.
        // For a 1, put a 1 in the "set" mask and a 1 in the "reset"
        // mask.
        // For a 0, put a 0 in the "set" mask and a 0 in the "reset"
        // mask.
        //
        // We represent the set/reset pairs as tuples, where the first
        // in each tuple is the "set" bit, and the second the "reset"
        // bit.
        let bits = s.chars().map(|c| {
            match c {
                'X' => (0, 1),
                '1' => (1, 1),
                '0' => (0, 0),

                // Will never occur; we checked earlier.
                // But we want to make the compiler happy.
                _ => panic!("Invalid char!")
            }
        });

        // Now concatenate the tuples into the two bitmasks.
        for bit in bits {
            set <<= 1;
            reset <<= 1;

            set |= bit.0;
            reset |= bit.1;
        }

        return Ok(Mask { set, reset });
    }

    // Apply this bitmask to a value.
    fn apply(&self, value: u64) -> u64 {
        // OR with set mask
        let value = value | self.set;

        // AND with reset mask
        return value & self.reset;
    }

    // Apply this bitmask to an address and return
    // a set of possible addresses.
    fn addresses(&self, value: u64) -> HashSet<u64> {
        // Apply overwrite bits first.
        // These are the bits where both set and reset
        // are 1.
        let overwrite = self.set & self.reset;

        let value = value | overwrite;

        // Apply floating bits.
        // This is a bit trickier.
        // First get all the possibilities of the floating bits.
        let floating = self.set ^ self.reset;
        let highest = msb(floating);

        let possibilities = if highest < 64 {
            find_possibilities(floating, highest)
        } else {
            HashSet::from_iter(vec![value].iter().map(|i| *i))
        };

        let mut addresses = HashSet::new();

        for p in possibilities {
            // Zero the floating bits and OR this possibility.
            addresses.insert((value & !floating) | p);
        }

        return addresses;
    }
}

// Given a bitmask, return all possible values when the bits
// covered by the bitmask are "floating" (i.e. can either be 0 or 1).
fn find_possibilities(bits: u64, highest: u64) -> HashSet<u64> {
    let mut possibilities = HashSet::new();

    if highest == 0 {
        possibilities.insert(0);
        possibilities.insert(1);
    } else if bits & 1 == 1 {
        let a = find_possibilities(bits >> 1, highest - 1);

        for p in a {
            let v = p << 1;

            possibilities.insert(v);

            let v = v | 1;

            possibilities.insert(v);
        }
    } else {
        let a = find_possibilities(bits >> 1, highest - 1);

        for p in a {
            let v = p << 1;

            possibilities.insert(v);
        }
    }

    return possibilities;
}

fn msb(bits: u64) -> u64 {
    let mut bits = bits;
    let mut msb = 64;

    for i in 0..64 {
        if bits & 1 == 1 {
            msb = i;
        }

        bits >>= 1;
    }

    return msb;
}

fn interpret(input: &str) -> Memory {
    // Regex to match memory stores.
    let re = Regex::new(r"mem\[(\d+)\]").unwrap();

    let mut memory = Memory::new();

    let mut mask = Mask::default();

    for line in input.trim().split('\n') {
        let line = line.trim();

        // Split on "="
        let sep_index = line.find('=')
            .expect(&format!("Invalid line: {}", line));

        let command = line.split_at(sep_index).0.trim();

        // split_at.1 includes the '='.
        let value = line.split_at(sep_index).1[1..].trim();

        // Mask or store?
        if command == "mask" {
            mask = Mask::from_str(value).unwrap();
        }
        else {
            // Attempt to match against regex.
            // If match, extract the address.
            let address = match re.captures(command) {
                Some(captures) => {
                    captures.get(1).unwrap()
                            .as_str()
                            .parse::<u64>().unwrap()
                },
                None => panic!("Invalid instruction: {}", command)
            };

            let value = value.parse().unwrap();

            memory.insert(address, mask.apply(value));
        }

    }

    return memory;
}

fn interpret_part2(input: &str) -> Memory {
    // Regex to match memory stores.
    let re = Regex::new(r"mem\[(\d+)\]").unwrap();

    let mut memory = Memory::new();

    let mut mask = Mask::default_address();

    for line in input.trim().split('\n') {
        let line = line.trim();

        // Split on "="
        let sep_index = line.find('=')
            .expect(&format!("Invalid line: {}", line));

        let command = line.split_at(sep_index).0.trim();

        // split_at.1 includes the '='.
        let value = line.split_at(sep_index).1[1..].trim();

        // Mask or store?
        if command == "mask" {
            mask = Mask::from_str(value).unwrap();
        }
        else {
            // Attempt to match against regex.
            // If match, extract the address.
            let address = match re.captures(command) {
                Some(captures) => {
                    captures.get(1).unwrap()
                            .as_str()
                            .parse::<u64>().unwrap()
                },
                None => panic!("Invalid instruction: {}", command)
            };

            let value = value.parse().unwrap();

            let addresses = mask.addresses(address);

            for a in &addresses {
                memory.insert(*a, value);
            }
        }

    }

    return memory;
}

fn part1() -> u64 {
    let input = fs::read_to_string("data/day14.txt")
        .expect("Could not read data/day14.txt");

    let memory = interpret(&input);

    return memory.values().sum();
}

fn part2() -> u64 {
    let input = fs::read_to_string("data/day14.txt")
        .expect("Could not read data/day14.txt");

    let memory = interpret_part2(&input);

    return memory.values().sum();
}

fn main() {
    let sum_part1 = part1();
    println!("Part 1: The sum of all values in memory is: {}", sum_part1);

    let sum_part2 = part2();
    println!("Part 2: The sum of all values in memory is: {}", sum_part2);
}
