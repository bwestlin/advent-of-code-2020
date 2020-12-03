extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;

use std::env;
use std::str::FromStr;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use utils::*;

type Input = Vec<Row>;

#[derive(Debug)]
struct Row {
    policy: Policy,
    password: String
}

#[derive(Debug)]
struct Policy {
    from: usize,
    to: usize,
    chr: char
}

impl Policy {

    fn conforms1(&self, password: &str) -> bool {
        let mut counts: [u8; 256] = [0; 256];

        for chr in password.chars() {
            counts[chr as usize] += 1;
        }

       let chr_count = counts[self.chr as usize] as usize;
       chr_count >= self.from && chr_count <= self.to
    }

    fn conforms2(&self, password: &str) -> bool {
        let chars: Vec<_> = password.chars().collect();

        let mut cnt = 0;
        if chars[self.from - 1] == self.chr {
            cnt += 1;
        }
        if chars[self.to - 1] == self.chr {
            cnt += 1;
        }

        cnt == 1
    }
}

fn solve(input: &Input) -> (usize, usize) {
    return input.iter()
        .fold((0, 0), |(p1, p2), row| {
            (
                p1 + if row.policy.conforms1(&row.password) { 1 } else { 0 },
                p2 + if row.policy.conforms2(&row.password) { 1 } else { 0 }
            )
        });
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let (part1, part2) = solve(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
    });
}

impl FromStr for Row {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();
        }
        let caps = RE.captures(s).unwrap();

        Ok(Row {
            policy: Policy {
                from: caps[1].parse::<usize>()?,
                to: caps[2].parse::<usize>()?,
                chr: caps[3].chars().next().unwrap()
            },
            password: caps[4].to_string()
        })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    Ok(reader.lines().map(|l| l.unwrap().parse::<Row>().unwrap()).collect())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&as_input(INPUT)).0, 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(&as_input(INPUT)).1, 1);
    }
}
