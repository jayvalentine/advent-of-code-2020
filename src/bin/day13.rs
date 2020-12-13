// Advent of Code 2020
// Day 13

use std::collections::HashMap;
use std::fs;

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "
        939
        7,13,x,x,59,x,31,19";

        let (arrival, buses) = parse_buses(input);

        assert_eq!(939, arrival);

        assert_eq!(59, earliest_bus(arrival, buses).0);
    }

    #[test]
    fn example_part2() {
        let input = "
        939
        7,13,x,x,59,x,31,19";

        let (arrival, buses) = parse_buses(input);

        assert_eq!(939, arrival);

        assert_eq!(1068781, earliest_timestamp(&buses));
    }
}

#[cfg(test)]
mod test_parse {
    use super::*;

    #[test]
    fn parse_with_none_missing() {
        let input = "
        100
        7,11,13";

        let (arrival, buses) = parse_buses(input);

        assert_eq!(100, arrival);

        assert_eq!(true, buses.contains(&7));
        assert_eq!(true, buses.contains(&11));
        assert_eq!(true, buses.contains(&13));

        assert_eq!(false, buses.contains(&5));
    }

    #[test]
    fn parse_with_missing() {
        let input = "
        523
        7,52,x,6";

        let (arrival, buses) = parse_buses(input);

        assert_eq!(523, arrival);

        assert_eq!(true, buses.contains(&7));
        assert_eq!(true, buses.contains(&52));
        assert_eq!(true, buses.contains(&6));

        assert_eq!(false, buses.contains(&11));
    }
}

#[cfg(test)]
mod test_puzzles {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(2238, part1());
    }
}

fn parse_buses(input: &str) -> (u32, Vec<u32>) {
    let mut input = input.trim().split('\n');
    let arrival = input.next().expect("No arrival in input").trim();
    let arrival = arrival.parse().expect("Arrival not a valid integer");

    let mut buses = Vec::new();
    for b in input.next().expect("No bus schedule in input")
        .trim().split(',') {
        if b == "x" {
            buses.push(0);
        } else {
            let b = b.parse()
                .expect(&format!("Bus ID is not a valid integer: {}", b));

            buses.push(b);
        }
    }

    return (arrival, buses);
}

fn earliest_bus(arrival: u32, buses: Vec<u32>) -> (u32, u32) {
    let mut closest = u32::MAX;
    let mut id = 0;
    for bus in buses {
        if bus == 0 { continue; }

        let departure = next_departure(arrival, bus);
        if departure < closest {
            closest = departure;
            id = bus;
        }
    }

    return (id, closest);
}

fn earliest_timestamp(buses: &[u32]) -> u64 {
    // Only check multiples of the first entry in
    // the schedule.
    let first = buses[0] as u64;
    let mut t = 0;

    let mut timings = HashMap::new();

    for (i, bus) in buses[1..].iter().enumerate() {
        if *bus == 0 {
            continue;
        }

        timings.insert(*bus as u64, (i as u64) + 1);
    }

    loop {
        if satisfies(t, &timings) {
            return t;
        }

        t += first;
    }
}

fn satisfies(t: u64, buses: &HashMap<u64, u64>) -> bool {
    for (bus, i) in buses {
        if *bus == 0 {
            continue;
        }

        let this_t = t + i;

        if this_t % bus != 0 {
            return false;
        }
    }

    return true;
}

fn next_departure(arrival: u32, id: u32) -> u32 {
    let mut departure = id;
    while departure < arrival {
        departure += id;
    }

    return departure;
}

fn part1() -> u32 {
    let input = fs::read_to_string("data/day13.txt")
        .expect("Could not read data/day13.txt");

    let (arrival, buses) = parse_buses(&input);

    let next = earliest_bus(arrival, buses);
    return (next.1 - arrival) * next.0;
}

fn part2() -> u64 {
    let input = fs::read_to_string("data/day13.txt")
        .expect("Could not read data/day13.txt");

    let (arrival, buses) = parse_buses(&input);

    return earliest_timestamp(&buses);
}

fn main() {
    let result = part1();
    println!("Part 1: The answer is: {}", result);

    let result = part2();
    println!("Part 2: The answer is: {}", result);
}

