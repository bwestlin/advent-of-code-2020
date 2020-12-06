extern crate utils;

use std::env;
use std::collections::BTreeSet;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Seat = String;

type Input = Vec<Seat>;

fn seat_id(seat: &str) -> u32 {
    let mut chrs = seat.chars();
    let mut row_range = 0..=127;
    let mut col_range = 0..=7;

    for c in chrs.by_ref().take(7) {
        let span = row_range.end() - row_range.start();
        let h_span = span / 2;
        let s = *row_range.start();
        let e = *row_range.end();
        match c {
            'F' => {
                row_range = s ..= (s + h_span);
            },
            'B' => {
                row_range = (s + h_span + 1) ..= e;
            },
            _ => unreachable!()
        }
    }

    for c in chrs {
        let span = col_range.end() - col_range.start();
        let h_span = span / 2;
        let s = *col_range.start();
        let e = *col_range.end();
        match c {
            'L' => {
                col_range = s ..= (s + h_span);
            },
            'R' => {
                col_range = (s + h_span + 1) ..= e;
            },
            _ => unreachable!()
        }
    }

    row_range.start() * 8 + col_range.start()
}

fn part1(input: &Input) -> u32 {
    input.iter()
        .map(|seat| seat_id(seat.as_str())).max()
        .unwrap_or(0)
}

fn part2(input: &Input) -> u32 {
    let seat_ids: BTreeSet<_> = input.iter()
        .map(|seat| seat_id(seat.as_str()))
        .collect();

    seat_ids.iter()
        .find(|&seat_id| {
            let check_id = seat_id + 1;
            seat_ids.get(&check_id).is_none() && seat_ids.get(&(check_id + 1)).is_some()
        })
        .map(|seat_id| seat_id + 1)
        .unwrap_or( 0)
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
    });
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    Ok(reader.lines().map(|l| l.unwrap()).collect())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "FBFBBFFRLR
        BFFFBBFRRR
        FFFBBBFRRR
        BBFFBBFRLL";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_set_id() {
        let seat_ids: Vec<_> = as_input(INPUT).iter().map(|seat| seat_id(seat)).collect();

        assert_eq!(seat_ids, vec![357, 567, 119, 820]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 820);
    }
}
