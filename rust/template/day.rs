extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;

use std::env;
use std::cmp;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::str::FromStr;
use std::error::Error;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use utils::*;

type Input = Vec<Data>;

#[derive(Debug)]
struct Data {
    d: i32
}

fn part1(input: &Input) -> i32 {
    dbg!(input);
    0
}

fn part2(input: &Input) -> i32 {
    0
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
    });
}

impl FromStr for Data {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Data {
            d: s.parse::<i32>()?
        })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    Ok(reader.lines().map(|l| l.unwrap().parse::<Data>().unwrap()).collect())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 1337);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&as_input(INPUT)), 1337);
    // }
}
