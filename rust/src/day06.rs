extern crate utils;

use std::env;
use std::collections::HashMap;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<Answers>;

#[derive(Debug)]
struct Answers {
    n_people: usize,
    freq: HashMap<char, usize>
}

fn solve(input: &Input) -> (usize, usize) {
    input.iter()
        .fold((0, 0), |(p1, p2), a| {
            (
                p1 + a.freq.keys().len(),
                p2 + a.freq.values().filter(|&f| f >= &a.n_people).count()
            )
        })
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
    let mut answers = vec![];
    let mut curr_freq: HashMap<char, usize> = HashMap::new();
    let mut n_people = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        let line = line.as_str();

        if line.trim().len() == 0 {
            let mut freq: HashMap<char, usize> = HashMap::new();
            std::mem::swap(&mut curr_freq, &mut freq);
            answers.push(Answers { n_people, freq });
            n_people = 0;
        } else {
            n_people += 1;
            for chr in line.chars() {
                *curr_freq.entry(chr).or_default() += 1;
            }
        }
    }

    if curr_freq.len() > 0 {
        let mut freq: HashMap<char, usize> = HashMap::new();
        std::mem::swap(&mut curr_freq, &mut freq);
        answers.push(Answers { n_people, freq });
    }

    Ok(answers)
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "abc

        a
        b
        c

        ab
        ac

        a
        a
        a
        a

        b";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&as_input(INPUT)).0, 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(&as_input(INPUT)).1, 6);
    }
}
