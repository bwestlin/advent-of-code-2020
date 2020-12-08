extern crate utils;

use std::env;
use std::collections::HashSet;
use std::str::FromStr;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<Instruction>;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum OpCode {
    Nop, Acc, Jmp
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    opcode: OpCode,
    arg: i32
}
#[derive(Debug)]
struct Console <'a> {
    instructions: &'a Vec<Instruction>,
    pc: usize,
    accumulator: i32
}

impl<'a> Console<'a> {
    fn from_program(instructions: &'a Vec<Instruction>) -> Self {
        Console { instructions, pc: 0, accumulator: 0 }
    }

    fn step(&mut self) {
        let Instruction { opcode, arg } = self.instructions[self.pc];

        match opcode {
            OpCode::Nop => {
                self.pc += 1;
            },
            OpCode::Acc => {
                self.accumulator += arg;
                self.pc += 1;
            },
            OpCode::Jmp => {
                self.pc += arg as usize;
            }
        }
    }

    fn run_until_loop_or_end(&mut self) {
        let mut visited = HashSet::new();

        loop {
            if visited.contains(&self.pc) || self.pc >= self.instructions.len() {
                break;
            }

            visited.insert(self.pc);
            self.step();
        }
    }
}

fn part1(input: &Input) -> i32 {
    let mut console = Console::from_program(input);
    console.run_until_loop_or_end();
    console.accumulator
}

fn part2(input: &Input) -> i32 {
    fn swap_opcode(opcode: OpCode) -> Option<OpCode> {
        match opcode {
            OpCode::Jmp => Some(OpCode::Nop),
            OpCode::Nop => Some(OpCode::Jmp),
            _ => None
        }
    }

    let mut program = input.clone();

    for i in 0..input.len() {
        let old_instruction = program[i];

        let Instruction { opcode, arg } = old_instruction;

        if let Some(opcode) = swap_opcode(opcode) {
            program[i] = Instruction { opcode, arg };
        } else {
            continue;
        }

        let mut console = Console::from_program(&program);
        console.run_until_loop_or_end();
        if console.pc >= input.len() {
            return console.accumulator;
        }

        program[i] = old_instruction;
    }

    0
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
    });
}

impl FromStr for OpCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nop" => Ok(OpCode::Nop),
            "acc" => Ok(OpCode::Acc),
            "jmp" => Ok(OpCode::Jmp),
            _ => Err(())
        }
    }
}
impl FromStr for Instruction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let opcode = parts.next().and_then(|s| s.parse::<OpCode>().ok()).unwrap();
        let arg = parts.next().and_then(|s| s.parse::<i32>().ok()).unwrap();

        Ok(Instruction { opcode, arg })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    Ok(reader.lines().map(|l| l.unwrap().parse::<Instruction>().unwrap()).collect())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT)), 8);
    }
}
