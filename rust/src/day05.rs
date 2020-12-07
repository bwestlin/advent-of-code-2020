extern crate utils;

use std::env;
use std::collections::BTreeSet;
use std::ops::RangeInclusive;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Seat = String;

type Input = Vec<Seat>;

fn bsp_to_val(s: &str, l_chr: char, h_chr: char, mut range: RangeInclusive<usize>) -> usize {
    for c in s.chars() {
        let span = range.end() - range.start();
        let h_span = span / 2;
        let s = *range.start();
        let e = *range.end();

        if c == l_chr {
            range = s ..= (s + h_span);
        } else if c == h_chr {
            range = (s + h_span + 1) ..= e;
        } else {
            unreachable!()
        }
    }

    *range.start()
}

fn seat_id(seat: &str) -> usize {
    let row = bsp_to_val(&seat[..7], 'F', 'B', 0..=127);
    let col = bsp_to_val(&seat[7..], 'L', 'R', 0..=7);
    row * 8 + col
}

fn part1(input: &Input) -> usize {
    input.iter()
        .map(|seat| seat_id(seat.as_str())).max()
        .unwrap_or(0)
}

fn part2(input: &Input) -> usize {
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
