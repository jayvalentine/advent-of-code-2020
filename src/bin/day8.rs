// Advent of Code 2020
// Day 8

use std::collections::HashSet;
use std::fs;

#[cfg(test)]
mod test_parse_instructon {
    use super::*;

    #[test]
    fn nop() {
        assert_eq!(Instruction::Nop(0), Instruction::parse("nop +0"));
    }

    #[test]
    fn acc() {
        assert_eq!(Instruction::Acc(1), Instruction::parse("acc +1"));
        assert_eq!(Instruction::Acc(3), Instruction::parse("acc +3"));
        assert_eq!(Instruction::Acc(-99), Instruction::parse("acc -99"));
        assert_eq!(Instruction::Acc(0), Instruction::parse("acc +0"));
    }

    #[test]
    fn jmp() {
        assert_eq!(Instruction::Jmp(4), Instruction::parse("jmp +4"));
        assert_eq!(Instruction::Jmp(-3), Instruction::parse("jmp -3"));
        assert_eq!(Instruction::Jmp(0), Instruction::parse("jmp +0"));
    }
}

#[cfg(test)]
mod test_interpret {
    use super::*;

    #[test]
    fn nop1() {
        let state = ProgramState { instruction: 0, accumulator: 0 };
        let state = execute(&Instruction::Nop(0), &state);
        assert_eq!(1, state.instruction);
        assert_eq!(0, state.accumulator);
    }

    #[test]
    fn nop2() {
        let state = ProgramState { instruction: 4, accumulator: 5 };
        let state = execute(&Instruction::Nop(0), &state);
        assert_eq!(5, state.instruction);
        assert_eq!(5, state.accumulator);
    }

    #[test]
    fn acc1() {
        let state = ProgramState { instruction: 4, accumulator: 7 };
        let state = execute(&Instruction::Acc(5), &state);
        assert_eq!(5, state.instruction);
        assert_eq!(12, state.accumulator);
    }

    #[test]
    fn acc2() {
        let state = ProgramState { instruction: 6, accumulator: 7 };
        let state = execute(&Instruction::Acc(-3), &state);
        assert_eq!(7, state.instruction);
        assert_eq!(4, state.accumulator);
    }

    #[test]
    fn acc3() {
        let state = ProgramState { instruction: 5, accumulator: 7 };
        let state = execute(&Instruction::Acc(-9), &state);
        assert_eq!(6, state.instruction);
        assert_eq!(-2, state.accumulator);
    }

    #[test]
    fn jmp1() {
        let state = ProgramState { instruction: 4, accumulator: 7 };
        let state = execute(&Instruction::Jmp(2), &state);
        assert_eq!(6, state.instruction);
        assert_eq!(7, state.accumulator);
    }

    #[test]
    fn jmp2() {
        let state = ProgramState { instruction: 4, accumulator: 4 };
        let state = execute(&Instruction::Jmp(-3), &state);
        assert_eq!(1, state.instruction);
        assert_eq!(4, state.accumulator);
    }
}

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn test_example_part1() {
        let program = vec![
            Instruction::Nop(0),
            Instruction::Acc(1),
            Instruction::Jmp(4),
            Instruction::Acc(3),
            Instruction::Jmp(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Jmp(-4),
            Instruction::Acc(6),
        ];

        let state = find_loop(&program).0;

        assert_eq!(1, state.instruction);
        assert_eq!(5, state.accumulator);
    }

    #[test]
    fn test_example_part2_noloop() {
        let program = vec![
            Instruction::Nop(0),
            Instruction::Acc(1),
            Instruction::Jmp(4),
            Instruction::Acc(3),
            Instruction::Jmp(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Nop(-4),
            Instruction::Acc(6),
        ];

        let state = find_loop(&program).0;

        assert_eq!(program.len(), state.instruction as usize);
        assert_eq!(8, state.accumulator);
    }

    #[test]
    fn test_example_part2_break_loop() {
        let program = vec![
            Instruction::Nop(0),
            Instruction::Acc(1),
            Instruction::Jmp(4),
            Instruction::Acc(3),
            Instruction::Jmp(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Jmp(-4),
            Instruction::Acc(6),
        ];

        let new_program = break_loop(&program);

        // New program must be same size as old one.
        assert_eq!(program.len(), new_program.len());

        // Second to last instruction should be changed
        // to a nop.
        let expected_program = vec![
            Instruction::Nop(0),
            Instruction::Acc(1),
            Instruction::Jmp(4),
            Instruction::Acc(3),
            Instruction::Jmp(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Nop(-4),
            Instruction::Acc(6),
        ];

        assert_eq!(expected_program, new_program);
    }
}

#[cfg(test)]
mod test_puzzles {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1782, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(797, part2());
    }
}

struct ProgramState {
    instruction: i32,
    accumulator: i32
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32)
}

impl Instruction {
    fn parse(s: &str) -> Instruction {
        let s = s.trim();
        let mut s_split = s.trim().split(' ');
        
        let opcode = s_split.next().expect(&format!("No opcode in instruction: '{}'", s));

        let operand = s_split.next().expect(&format!("No operand in instruction: '{}'", s));
        let operand: i32 = operand.parse().expect(&format!("Operand is not a valid integer: '{}'", s));

        return match opcode {
            "nop" => Instruction::Nop(operand),
            "acc" => Instruction::Acc(operand),
            "jmp" => Instruction::Jmp(operand),
            _ => panic!("Unknown opcode")
        }
    }
}

fn execute(i: &Instruction, s: &ProgramState) -> ProgramState {
    let instruction = s.instruction;
    let accumulator = s.accumulator;

    return match i {
        Instruction::Nop(_) => ProgramState { instruction: instruction + 1, accumulator },
        Instruction::Acc(n) => ProgramState { instruction: instruction + 1, accumulator: accumulator + n },
        Instruction::Jmp(n) => ProgramState { instruction: instruction + n, accumulator }
    };
}

// Returns the program state and set of visited instructions
// either on termination or on the second visit to any given instruction.
fn find_loop(p: &Vec<Instruction>) -> (ProgramState, HashSet<i32>) {
    let mut state = ProgramState { instruction: 0, accumulator: 0 };

    let mut visited: HashSet<i32> = HashSet::new();

    loop {
        let index = state.instruction as usize;

        let instruction = p.get(index).expect("Execution out of bounds");

        state = execute(instruction, &state);

        if (state.instruction as usize) == p.len() {
            return (state, visited);
        }

        if visited.contains(&state.instruction) {
            return (state, visited);
        }

        visited.insert(state.instruction);
    }
}

// Given a program with an infinite loop, returns an altered
// version of that program with one instruction changed,
// which does not contain a loop.
fn break_loop(p: &Vec<Instruction>) -> Vec<Instruction> {
    // Get the instructions visited by the original program.
    let visited = find_loop(p).1;

    // For each nop or jmp in the visited instructions,
    // change it and see if the program completes.
    for i in visited {
        let new_instruction = match p.get(i as usize).unwrap() {
            Instruction::Nop(n) => Instruction::Jmp(*n),
            Instruction::Jmp(n) => Instruction::Nop(*n),
            Instruction::Acc(_) => continue // Skip adds
        };

        // Clone the program.
        let mut new_p = p.to_vec();

        // Insert the new instruction.
        new_p[i as usize] = new_instruction;

        // Run, see if it loops.
        let state = find_loop(&new_p).0;
        let index = state.instruction as usize;
        if index == new_p.len() {
            return new_p;
        }
    }

    panic!("Could not break loop");
}

fn part1() -> i32 {
    let s = fs::read_to_string("data/day8.txt").expect("Could not read data/day8.txt");

    let mut p: Vec<Instruction> = Vec::new();

    for i in s.trim().split('\n') {
        p.push(Instruction::parse(i.trim()));
    }

    let state = find_loop(&p).0;

    return state.accumulator;
}

fn part2() -> i32 {
    let s = fs::read_to_string("data/day8.txt").expect("Could not read data/day8.txt");

    let mut p: Vec<Instruction> = Vec::new();

    for i in s.trim().split('\n') {
        p.push(Instruction::parse(i.trim()));
    }

    let fixed_p = break_loop(&p);
    let state = find_loop(&fixed_p).0;

    return state.accumulator;
}

fn main() {
    let acc_part1 = part1();
    println!("The value of the accumulator at the loop-point is: {}", acc_part1);

    let acc_part2 = part2();
    println!("The value of the accumulator at the end of the fixed program is: {}", acc_part2);
}
