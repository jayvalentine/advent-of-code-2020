// Advent of Code 2020
// Day 15

use std::collections::HashMap;

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn part1() {
        let input = vec![0, 3, 6];
        assert_eq!(0, spoken(&input, 1));
        assert_eq!(3, spoken(&input, 2));
        assert_eq!(6, spoken(&input, 3));

        assert_eq!(436, spoken(&input, 2020));
    }
}

mod test_spoken {
    use super::*;

    #[test]
    fn starting() {
        let input = vec![4, 6, 2];
        assert_eq!(4, spoken(&input, 1));
        assert_eq!(6, spoken(&input, 2));
        assert_eq!(2, spoken(&input, 3));
    }

    #[test]
    fn fourth() {
        let input = vec![0, 3, 6];
        assert_eq!(0, spoken(&input, 4));
    }

    #[test]
    fn fifth() {
        let input = vec![0, 3, 6];
        assert_eq!(3, spoken(&input, 5));
    }

    #[test]
    fn sixth() {
        let input = vec![0, 3, 6];
        assert_eq!(3, spoken(&input, 6));
    }
}

#[cfg(test)]
mod test_puzzles {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(240, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(505, part2());
    }
}

fn spoken(starting: &[u32], i: usize) -> u32 {
    assert!(i != 0);

    let mut said = HashMap::new();
    let mut said_twice: HashMap<u32, (usize, usize)> = HashMap::new();
    let mut spoken: u32 = *starting.last().unwrap();

    for turn in 1..=i {
        let previous = spoken;

        spoken = if turn <= starting.len() {
            starting[turn-1]
        } else if said_twice.contains_key(&previous) {
            (said_twice[&previous].0 - said_twice[&previous].1) as u32
        } else {
            0
        };

        if said.contains_key(&spoken) {
            said_twice.insert(spoken, (turn, said[&spoken]));
        }

        said.insert(spoken, turn);
    }

    return spoken;
}

fn part1() -> u32 {
    let input = vec![14, 8, 16, 0, 1, 17];
    return spoken(&input, 2020);
}

fn part2() -> u32 {
    let input = vec![14, 8, 16, 0, 1, 17];
    return spoken(&input, 30000000);
}

fn main() {
    println!("Part 1: The 2020th number spoken is: {}", part1());
    println!("Part 1: The 30000000th number spoken is: {}", part2());
}