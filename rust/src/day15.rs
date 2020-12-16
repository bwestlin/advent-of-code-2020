extern crate utils;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<u32>;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Spoken {
    last_t: u32,
    turns_apart: u32
}

fn solve(input: &Input) -> (usize, usize) {
    const NEVER: Spoken = Spoken { last_t: std::u32::MAX, turns_apart: std::u32::MAX };
    let mut memory: Vec<Spoken> = vec![NEVER; 30000000];

    let mut last_spoken = 0;
    let mut p1_spoken = 0;

    for t in 0..30000000 {
        let spoken =
            if t < input.len() {
                input[t]
            } else {
                if memory[last_spoken].turns_apart == 0 || memory[last_spoken] == NEVER {
                    0
                } else {
                    memory[last_spoken].turns_apart
                }
            } as usize;

        let t = t as u32;
        if memory[spoken] == NEVER {
            memory[spoken] = Spoken {
                last_t: t,
                turns_apart: 0
            };
        } else {
            memory[spoken] = Spoken {
                last_t: t,
                turns_apart: t - memory[spoken].last_t
            };
        }

        if t == 2020 - 1 {
            p1_spoken = spoken;
        }
        last_spoken = spoken;
    }

    (p1_spoken, last_spoken)
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let (part1, part2) = solve(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
    });
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    Ok(reader.lines().next().unwrap()?.split(',').map(|i| i.parse::<u32>().unwrap()).collect())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&as_input("0,3,6")).0, 436);

        assert_eq!(solve(&as_input("1,3,2")).0, 1);
        assert_eq!(solve(&as_input("2,1,3")).0, 10);
        assert_eq!(solve(&as_input("1,2,3")).0, 27);
        assert_eq!(solve(&as_input("2,3,1")).0, 78);
        assert_eq!(solve(&as_input("3,2,1")).0, 438);
        assert_eq!(solve(&as_input("3,1,2")).0, 1836);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(&as_input("0,3,6")).1, 175594);

        assert_eq!(solve(&as_input("1,3,2")).1, 2578);
        assert_eq!(solve(&as_input("2,1,3")).1, 3544142);
        assert_eq!(solve(&as_input("1,2,3")).1, 261214);
        assert_eq!(solve(&as_input("2,3,1")).1, 6895259);
        assert_eq!(solve(&as_input("3,2,1")).1, 18);
        assert_eq!(solve(&as_input("3,1,2")).1, 362);
    }
}
