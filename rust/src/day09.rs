extern crate utils;

use std::env;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<u64>;

#[cfg(test)]
const PREAMBLE_SIZE: usize = 5;
#[cfg(not(test))]
const PREAMBLE_SIZE: usize = 25;

fn part1(input: &Input) -> u64 {
    let mut preamble = VecDeque::new();

    for i in 0..PREAMBLE_SIZE {
        let mut sums = HashSet::new();
        for j in 0..PREAMBLE_SIZE {
            if i == j {
                continue;
            }
            sums.insert(input[i] + input[j]);
        }
        preamble.push_back(sums);
    }

    for i in PREAMBLE_SIZE..input.len() {
        let v = input[i];

        if !preamble.iter().any(|sums| sums.contains(&v)) {
            return v;
        }

        preamble.pop_front();

        let mut sums = HashSet::new();
        for j in (i - PREAMBLE_SIZE)..i {
            sums.insert(v + input[j]);
        }
        preamble.push_back(sums);
    }

    0
}

fn part2(input: &Input, p1: u64) -> u64 {

    for i in 0..input.len() {
        let mut sum = 0;
        let mut min = std::u64::MAX;
        let mut max = std::u64::MIN;

        for j in i..input.len() {
            let v = input[j];
            sum += v;
            min = std::cmp::min(v, min);
            max = std::cmp::max(v, max);

            if sum == p1 {
                return min + max;
            } else if sum > p1 {
                break;
            }
        }
    }

    0
}

fn solve(input: &Input) -> (u64, u64) {
    let p1 = part1(input);
    (p1, part2(input, p1))
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
    Ok(reader.lines().map(|l| l.unwrap().parse::<u64>().unwrap()).collect())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&as_input(INPUT)).0, 127);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(&as_input(INPUT)).1, 62);
    }
}
