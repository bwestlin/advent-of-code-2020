extern crate utils;

use std::env;
use std::collections::VecDeque;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<u32>;

fn n_combinations(n_diffs: usize) -> usize {
    let mut queue = VecDeque::new();
    queue.push_front(0);
    let mut cnt = 0;

    while let Some(idx) = queue.pop_front() {
        if idx == n_diffs {
            cnt += 1;
        }
        for i in (idx + 1)..=n_diffs {
            if i - idx > 3 {
                continue;
            }
            queue.push_front(i);
        }
    }

    cnt
}

fn solve(input: &Input) -> (usize, usize) {
    let mut input = input.clone();
    input.push(0);
    input.sort();
    input.push(input.last().unwrap() + 3);

    let mut diffs = Vec::with_capacity(input.len());

    let mut last = 0;
    for &i in input.iter().skip(1) {
        let diff = i - last;
        diffs.push(diff);
        last = i;
    }

    let p1 = diffs.iter().filter(|&&n| n == 1).count()
           * diffs.iter().filter(|&&n| n == 3).count();

    let p2 = diffs
        .split(|&diff| diff == 3)
        .fold(1, |cnt, ones| {
            cnt * n_combinations(ones.len())
        });

    (p1, p2)
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
    Ok(reader.lines().map(|l| l.unwrap().parse::<u32>().unwrap()).collect())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &'static str =
       "16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4";

    const INPUT2: &'static str =
       "28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&as_input(INPUT1)).0, 7 * 5);
        assert_eq!(solve(&as_input(INPUT2)).0, 22 * 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(&as_input(INPUT1)).1, 8);
        assert_eq!(solve(&as_input(INPUT2)).1, 19208);
    }
}
