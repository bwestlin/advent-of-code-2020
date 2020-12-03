extern crate utils;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Map;

#[derive(Eq, PartialEq, Clone,Copy, Debug)]
enum MapType {
    Tree, Open
}

#[derive(Debug)]
struct Map {
    rows: Vec<Vec<MapType>>
}

impl Map {
    fn at(&self, x: usize, y: usize) -> Option<MapType> {
        if y >= self.rows.len() {
            None
        } else {
            let row = &self.rows[y];
            Some(row[x % row.len()])
        }
    }

    fn num_trees_hit(&self, slope_x: usize, slope_y: usize) -> usize {
        let mut num_trees = 0;
        let mut x = 0;
        let mut y = 0;

        for _ in 0.. {
            if let Some(mt) = self.at(x, y) {
                num_trees += if mt == MapType::Tree { 1 } else { 0 };
            } else {
                break;
            }

            x += slope_x;
            y += slope_y;
        }

        num_trees
    }
}

fn part1(input: &Input) -> usize {
    input.num_trees_hit(3, 1)
}

fn part2(input: &Input) -> usize {
    let slopes = vec![
        (1_usize, 1_usize),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ];

    slopes.iter()
        .map(|(s_x, s_y)| input.num_trees_hit(*s_x, *s_y))
        .product()
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
    });
}


fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    let mut rows: Vec<Vec<MapType>> = vec![];

    for line in reader.lines().map(|l| l.unwrap()) {
        let row: Vec<_> = line.chars()
            .map(|chr| if chr == '#' { MapType::Tree } else { MapType::Open })
            .collect();
        rows.push(row);
    }

    Ok(Map { rows })
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT)), 336);
    }
}
