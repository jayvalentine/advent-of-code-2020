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

        let state = find_loop(program);

        assert_eq!(1, state.instruction);
        assert_eq!(5, state.accumulator);
    }
}

#[cfg(test)]
mod test_puzzles {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1782, part1());
    }
}

struct ProgramState {
    instruction: i32,
    accumulator: i32
}

#[derive(Eq, PartialEq, Debug)]
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

fn find_loop(p: Vec<Instruction>) -> ProgramState {
    let mut state = ProgramState { instruction: 0, accumulator: 0 };

    let mut visited: HashSet<i32> = HashSet::new();

    loop {
        let index = state.instruction as usize;
        let instruction = p.get(index).expect("Execution out of bounds");

        state = execute(instruction, &state);

        if visited.contains(&state.instruction) {
            return state;
        }

        visited.insert(state.instruction);
    }
}

fn part1() -> i32 {
    let s = fs::read_to_string("data/day8.txt").expect("Could not read data/day8.txt");

    let mut p: Vec<Instruction> = Vec::new();

    for i in s.trim().split('\n') {
        p.push(Instruction::parse(i.trim()));
    }

    let state = find_loop(p);

    return state.accumulator;
}

fn main() {
    let acc_part1 = part1();
    println!("The value of the accumulator at the loop-point is: {}", acc_part1);
}
