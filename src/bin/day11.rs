// Advent of Code 2020
// Day 11

use std::collections::HashMap;

use std::fs;

type State = HashMap<(usize, usize), Seat>;
type Rule = fn(&(usize, usize), Seat, &State) -> Seat;

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = "
        L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";

        let input = parse_input(input);

        let seats = stable_occupied_seats(input, rule_part1, false);
        assert_eq!(37, seats);
    }
}

#[cfg(test)]
mod test_puzzles {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(2251, part1(false));
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Seat {
    None,
    Unoccupied,
    Occupied
}

fn parse_input(input: &str) -> HashMap<(usize, usize), Seat> {
    let mut seats = HashMap::new();

    let mut y = 0;
    for row in input.trim().split('\n') {
        let mut x = 0;
        for col in row.trim().chars() {
            let s = match col {
                'L' => Seat::Unoccupied,
                '#' => Seat::Occupied,
                '.' => Seat::None,
                _ => panic!("Invalid character: {}", col)
            };

            // Don't add floor tiles to map.
            if s != Seat::None {
                seats.insert((x, y), s);
            }

            x += 1;
        }

        y += 1;
    }

    return seats;
}

fn stable_occupied_seats(input: State, rule: Rule, print_states: bool) -> usize {
    let mut current_state = input;

    loop {
        // If we're in interactive mode, print the current state.
        if print_states {
            print!("\x1b[2J\x1b[1;1H");

            // Print just a portion of the state.
            for y in 0..40 {
                for x in 0..160 {
                    let c = match current_state.get(&(x, y)) {
                        Some(s) => {
                            match *s {
                                Seat::Occupied => '#',
                                Seat::Unoccupied => 'L',
                                Seat::None => '.'
                            }
                        },
                        None => '.'
                    };

                    print!("{}", c);
                }

                print!("\n");
            }
        }

        let new_state = next_generation(&current_state, rule);

        if new_state == current_state {
            return new_state
                .values()
                .filter(|&v| *v == Seat::Occupied)
                .count();
        }

        current_state = new_state;
    }
}

fn rule_part1(point: &(usize, usize), seat: Seat, state: &HashMap<(usize, usize), Seat>) -> Seat {
    let occupied = occupied(point, state);

    // Apply rules based on the current value and the
    // number of occupied neighbours.
    return match seat {
        Seat::Occupied => {
            if occupied >= 4 {
                Seat::Unoccupied
            } else {
                seat
            }
        },
        Seat::Unoccupied => {
            if occupied == 0 {
                Seat::Occupied
            } else {
                seat
            }
        }
        _ => seat
    };
}

fn next_generation(state: &State, rule: Rule) -> State {
    let mut next = HashMap::new();

    for (&point, &seat) in state {
        next.insert(point, rule(&point, seat, state));
    }

    return next;
}

fn occupied(point: &(usize, usize), state: &State) -> usize {
    let x = point.0;
    let y = point.1;

    let min_x = if x == 0 { x } else { x - 1 };
    let min_y = if y == 0 { y } else { y - 1 };

    let mut count = 0;

    for p_x in min_x..=x+1 {
        for p_y in min_y..=y+1 {
            if p_x == x && p_y == y { continue; }

            if !state.contains_key(&(p_x, p_y)) {
                continue;
            }

            if *state.get(&(p_x, p_y)).unwrap() == Seat::Occupied {
                count += 1;
            }
        }
    }

    return count;
}

fn part1(print_state: bool) -> usize {
    let input = fs::read_to_string("data/day11.txt")
        .expect("Could not read data/day11.txt");

    let input = parse_input(&input);

    return stable_occupied_seats(input, rule_part1, print_state);
}

fn main() {
    let occupied_seats = part1(true);
    println!("Part 1: The number of occupied seats is: {}", occupied_seats);
}
