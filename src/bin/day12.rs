// Advent of Code 2020
// Day 12

use std::fs;

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = "
        F10
        N3
        F7
        R90
        F11";

        let mut state = State {
            x: 0,
            y: 0,
            heading: Direction::East
        };

        for i in input.trim().split('\n') {
            let action = Action::from_str(i.trim());
            state = action.apply(&state);
        }

        let d = manhattan_distance(&state,
            &State {
                x: 0,
                y: 0,
                heading: Direction::East
            });

        assert_eq!(25, d);
    }
}

#[cfg(test)]
mod test_action {
    use super::*;

    #[test]
    fn test_apply_n() {
        let a = Action::from_str("N3");
        let state = State {
            x: 5,
            y: 3,
            heading: Direction::East
        };

        let state = a.apply(&state);

        assert_eq!(5, state.x);
        assert_eq!(0, state.y);
        assert_eq!(Direction::East, state.heading);
    }

    #[test]
    fn test_apply_s() {
        let a = Action::from_str("S2");
        let state = State {
            x: 5,
            y: 3,
            heading: Direction::East
        };

        let state = a.apply(&state);

        assert_eq!(5, state.x);
        assert_eq!(5, state.y);
        assert_eq!(Direction::East, state.heading);
    }

    #[test]
    fn test_apply_w() {
        let a = Action::from_str("W10");
        let state = State {
            x: 5,
            y: 3,
            heading: Direction::East
        };

        let state = a.apply(&state);

        assert_eq!(-5, state.x);
        assert_eq!(3, state.y);
        assert_eq!(Direction::East, state.heading);
    }

    #[test]
    fn test_apply_e() {
        let a = Action::from_str("E1");
        let state = State {
            x: 5,
            y: 3,
            heading: Direction::East
        };

        let state = a.apply(&state);

        assert_eq!(6, state.x);
        assert_eq!(3, state.y);
        assert_eq!(Direction::East, state.heading);
    }

    #[test]
    fn test_apply_l90() {
        let a = Action::from_str("L90");
        let state = State {
            x: 5,
            y: 3,
            heading: Direction::East
        };

        let state = a.apply(&state);

        assert_eq!(5, state.x);
        assert_eq!(3, state.y);
        assert_eq!(Direction::North, state.heading);

        let state = a.apply(&state);

        assert_eq!(5, state.x);
        assert_eq!(3, state.y);
        assert_eq!(Direction::West, state.heading);
    }

    #[test]
    fn test_apply_l180() {
        let a = Action::from_str("L180");
        let state = State {
            x: 5,
            y: 3,
            heading: Direction::East
        };

        let state = a.apply(&state);

        assert_eq!(5, state.x);
        assert_eq!(3, state.y);
        assert_eq!(Direction::West, state.heading);

        let state = a.apply(&state);

        assert_eq!(5, state.x);
        assert_eq!(3, state.y);
        assert_eq!(Direction::East, state.heading);
    }

    #[test]
    fn test_apply_r90() {
        let a = Action::from_str("R90");
        let state = State {
            x: 5,
            y: 3,
            heading: Direction::East
        };

        let state = a.apply(&state);

        assert_eq!(5, state.x);
        assert_eq!(3, state.y);
        assert_eq!(Direction::South, state.heading);

        let state = a.apply(&state);

        assert_eq!(5, state.x);
        assert_eq!(3, state.y);
        assert_eq!(Direction::West, state.heading);
    }

    #[test]
    fn test_apply_r180() {
        let a = Action::from_str("R180");
        let state = State {
            x: 5,
            y: 3,
            heading: Direction::East
        };

        let state = a.apply(&state);

        assert_eq!(5, state.x);
        assert_eq!(3, state.y);
        assert_eq!(Direction::West, state.heading);

        let state = a.apply(&state);

        assert_eq!(5, state.x);
        assert_eq!(3, state.y);
        assert_eq!(Direction::East, state.heading);
    }

    #[test]
    fn test_apply_f() {
        let a = Action::from_str("F2");

        let state = State {
            x: 5,
            y: 3,
            heading: Direction::East
        };

        let state = a.apply(&state);

        assert_eq!(7, state.x);
        assert_eq!(3, state.y);
        assert_eq!(Direction::East, state.heading);

        let state = State {
            x: 5,
            y: 3,
            heading: Direction::South
        };

        let state = a.apply(&state);

        assert_eq!(5, state.x);
        assert_eq!(5, state.y);
        assert_eq!(Direction::South, state.heading);

        let state = State {
            x: 5,
            y: 3,
            heading: Direction::West
        };

        let state = a.apply(&state);

        assert_eq!(3, state.x);
        assert_eq!(3, state.y);
        assert_eq!(Direction::West, state.heading);

        let state = State {
            x: 5,
            y: 3,
            heading: Direction::North
        };

        let state = a.apply(&state);

        assert_eq!(5, state.x);
        assert_eq!(1, state.y);
        assert_eq!(Direction::North, state.heading);
    }
}

struct State {
    x: i32, // X-position; positive numbers east, negative west.
    y: i32, // Y-position; positive numbers south, negative north.
    heading: Direction
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West
}

enum Action {
    N(u32),
    S(u32),
    E(u32),
    W(u32),
    L(u32),
    R(u32),
    F(u32)
}

impl Action {
    fn from_str(s: &str) -> Action {
        let i = match s[1..].parse() {
            Ok(n) => n,
            Err(_) => panic!("Invalid value for action: {}", s)
        };

        match &s[..1] {
            "N" => return Action::N(i),
            "S" => return Action::S(i),
            "E" => return Action::E(i),
            "W" => return Action::W(i),
            "L" => {
                if i % 90 != 0 {
                    panic!("Invalid value for L: {}", i);
                }
                return Action::L(i)
            },
            "R" => {
                if i % 90 != 0 {
                    panic!("Invalid value for L: {}", i);
                }
                return Action::R(i)
            },
            "F" => return Action::F(i),
            _ => panic!("Invalid action: {}", &s[..1])
        }
    }

    fn apply(&self, state: &State) -> State {
        let mut x = state.x;
        let mut y = state.y;
        let mut heading = state.heading;

        match self {
            Action::N(i) => y -= *i as i32,
            Action::S(i) => y += *i as i32,
            Action::W(i) => x -= *i as i32,
            Action::E(i) => x += *i as i32,
            Action::R(i) => heading = right(heading, *i),
            Action::L(i) => heading = left(heading, *i),
            Action::F(i) => {
                match heading {
                    Direction::North => y -= *i as i32,
                    Direction::South => y += *i as i32,
                    Direction::West => x -= *i as i32,
                    Direction::East => x += *i as i32
                }
            }

        }

        return State {
            x, y, heading
        };
    }
}

fn left(heading: Direction, degrees: u32) -> Direction {
    let mut degrees = degrees;
    let mut heading = heading;
    
    while degrees > 0 {
        heading = match heading {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East
        };

        degrees -= 90;
    }

    return heading;
}

fn right(heading: Direction, degrees: u32) -> Direction {
    let mut degrees = degrees;
    let mut heading = heading;
    
    while degrees > 0 {
        heading = match heading {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East
        };

        degrees -= 90;
    }

    return heading;
}

fn manhattan_distance(a: &State, b: &State) -> i32 {
    let dx = (a.x - b.x).abs();
    let dy = (a.y - b.y).abs();

    return dx + dy;
}

fn part1() -> i32 {
    let input = fs::read_to_string("data/day12.txt")
        .expect("Could not read data/day12.txt");

    let mut state = State {
        x: 0,
        y: 0,
        heading: Direction::East
    };

    for i in input.trim().split('\n') {
        let action = Action::from_str(i.trim());
        state = action.apply(&state);
    }

    return manhattan_distance(&state,
        &State {
            x: 0,
            y: 0,
            heading: Direction::East
        });
}

fn main() {
    let d = part1();
    println!("Part 1: The manhattan distance is: {}", d);
}
