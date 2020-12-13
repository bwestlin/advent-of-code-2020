extern crate utils;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Grid;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum GridState {
    Floor, EmptySeat, OccupiedSeat
}

impl GridState {
    fn parse(chr: char) -> GridState {
        match chr {
            '.' => GridState::Floor,
            'L' => GridState::EmptySeat,
            '#' => GridState::OccupiedSeat,
            _ => unreachable!()
        }
    }
}

#[derive(Clone, Debug)]
struct Grid {
    rows: Vec<Vec<GridState>>
}

impl Grid {
    fn at(&self, x: usize, y: usize) -> Option<GridState> {
        self.rows.get(y).and_then(|row| row.get(x).cloned())
    }

    fn count_occupied_adjacent(&self, x: usize, y: usize) -> usize {
        let x = x as i32;
        let y = y as i32;
        let mut cnt = 0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let tx = x + dx;
                let ty = y + dy;

                if tx >= 0 && ty >= 0 {
                    cnt += self.at(tx as usize, ty as usize)
                        .filter(|&s| s == GridState::OccupiedSeat)
                        .map(|_| 1)
                        .unwrap_or(0);

                }
            }
        }
        cnt
    }

    fn count_occupied_directional(&self, x: usize, y: usize) -> usize {
        let x = x as i32;
        let y = y as i32;
        let mut cnt = 0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let mut tx = x;
                let mut ty = y;

                let seat = loop {
                    tx += dx;
                    ty += dy;

                    if tx >= 0 && ty >= 0 {
                        match self.at(tx as usize, ty as usize) {
                            Some(GridState::Floor) => continue,
                            at => break at
                        }

                    } else {
                        break None;
                    }
                };

                cnt += seat
                    .filter(|&s| s == GridState::OccupiedSeat)
                    .map(|_| 1)
                    .unwrap_or(0);
            }
        }
        cnt
    }

    fn num_occupied(&self) -> usize{
        self.rows.iter()
            .map(|row| row.iter().filter(|&&s| s == GridState::OccupiedSeat).count())
            .sum()
    }

    fn step(&self, occupied_threshold: usize, count_occupied: fn(&Grid, usize, usize) -> usize) -> (usize, Grid) {
        let mut next_grid = self.clone();
        let mut num_changed = 0;

        for y in 0..self.rows.len() {
            let row = &self.rows[y];

            for x in 0..row.len() {
                if let Some(state) = self.at(x, y) {
                    if state == GridState::Floor {
                        continue;
                    }

                    let adj_count = count_occupied(&self, x, y);

                    let next_state =
                        if state == GridState::EmptySeat && adj_count == 0 {
                            GridState::OccupiedSeat
                        } else if state == GridState::OccupiedSeat && adj_count >= occupied_threshold {
                            GridState::EmptySeat
                        } else {
                            state
                        };

                    if next_state != next_grid.rows[y][x] {
                        next_grid.rows[y][x] = next_state;
                        num_changed += 1;
                    }
                }
            }
        }
        (num_changed, next_grid)
    }
}

fn num_occupied_when_stable(mut grid: Grid,
                            occupied_threshold: usize,
                            count_occupied: fn(&Grid, usize, usize) -> usize) -> usize {
    loop {
        let (num_changed, next_grid) = grid.step(occupied_threshold, count_occupied);
        if num_changed == 0 {
            break next_grid.num_occupied()
        }
        grid = next_grid;
    }
}

fn part1(input: &Input) -> usize {
    num_occupied_when_stable(input.clone(), 4, Grid::count_occupied_adjacent)
}

fn part2(input: &Input) -> usize {
    num_occupied_when_stable(input.clone(), 5, Grid::count_occupied_directional)
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
    });
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    let rows = reader.lines()
        .map(|l| l.unwrap().chars().map(GridState::parse).collect::<Vec<_>>())
        .collect();
    Ok(Grid { rows })
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT)), 26);
    }
}
