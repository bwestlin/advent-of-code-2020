extern crate utils;

use std::env;
use std::str::FromStr;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<Instruction>;

#[derive(Copy, Clone, Debug)]
enum Action {
    N, S, E, W, L, R, F
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    value: i32
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Vec2 {
    x: i32,
    y: i32
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Vec2 { x, y }
    }

    fn translate(&self, by: Vec2) -> Self {
        Vec2::new(self.x + by.x, self.y + by.y)
    }

    fn mul(&self, by: i32) -> Self {
        Vec2::new(self.x * by, self.y * by)
    }

    fn rotate(&self, mut by: i32) -> Self {
        if by < 0 {
            by += 360;
        }
        let mut current = self.clone();

        for _ in 0..(by / 90) {
            let next_x = current.y * -1;
            let next_y = current.x;
            current.x = next_x;
            current.y = next_y;
        }
        current
    }
}

const DIR_E: Vec2 = Vec2 { x: 1, y: 0 };
const DIR_W: Vec2 = Vec2 { x: -1, y: 0 };
const DIR_N: Vec2 = Vec2 { x: 0, y: -1 };
const DIR_S: Vec2 = Vec2 { x: 0, y: 1 };

#[derive(Debug)]
struct Ship {
    pos: Vec2,
    dir: Vec2
}

impl Ship {
    fn new() -> Self {
        Ship { pos: Vec2::new(0, 0), dir: Vec2 { x: 1, y: 0 } }
    }

    fn run(&mut self, ins: &Instruction) {
        match ins.action {
            Action::N => {
                self.pos = self.pos.translate(Vec2::new(0, ins.value * -1));
            },
            Action::S => {
                self.pos = self.pos.translate(Vec2::new(0, ins.value));
            },
            Action::E => {
                self.pos = self.pos.translate(Vec2::new(ins.value, 0));
            },
            Action::W => {
                self.pos = self.pos.translate(Vec2::new(ins.value * -1, 0));
            },
            Action::L => {
                self.dir = self.dir.rotate(ins.value * -1);
            },
            Action::R => {
                self.dir = self.dir.rotate(ins.value);
            },
            Action::F => {
                self.pos = self.pos.translate(self.dir.mul(ins.value));
            }
        }
    }

    fn run_with_waypoint(&mut self, ins: &Instruction, mut wp: Vec2) -> Vec2 {
        match ins.action {
            Action::N => {
                wp = wp.translate(DIR_N.mul(ins.value));
            },
            Action::S => {
                wp = wp.translate(DIR_S.mul(ins.value));
            },
            Action::E => {
                wp = wp.translate(DIR_E.mul(ins.value));
            },
            Action::W => {
                wp = wp.translate(DIR_W.mul(ins.value));
            },
            Action::L => {
                wp = wp.rotate(ins.value * -1);
            },
            Action::R => {
                wp = wp.rotate(ins.value);
            },
            Action::F => {
                self.pos = self.pos.translate(wp.mul(ins.value));
            }
        }
        wp
    }
}

fn part1(input: &Input) -> i32 {
    let mut ship = Ship::new();

    for ins in input {
        ship.run(ins);
    }

    ship.pos.x.abs() + ship.pos.y.abs()
}

fn part2(input: &Input) -> i32 {
    let mut ship = Ship::new();
    let mut wp = Vec2::new(10, -1);

    for ins in input {
        wp = ship.run_with_waypoint(ins, wp);
    }

    ship.pos.x.abs() + ship.pos.y.abs()
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
    });
}

impl FromStr for Instruction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = match &s[0..1] {
            "N" => Action::N,
            "S" => Action::S,
            "E" => Action::E,
            "W" => Action::W,
            "L" => Action::L,
            "R" => Action::R,
            "F" => Action::F,
            _ => unreachable!()
        };
        let value = s[1..].parse::<i32>()?;

        Ok(Instruction {
            action,
            value
        })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    Ok(reader.lines().map(|l| l.unwrap().parse::<Instruction>().unwrap()).collect())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "F10
        N3
        F7
        R90
        F11";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_vec2_rotate() {
        // 90
        assert_eq!(DIR_E.rotate(90), DIR_S);
        assert_eq!(DIR_S.rotate(90), DIR_W);
        assert_eq!(DIR_W.rotate(90), DIR_N);
        assert_eq!(DIR_N.rotate(90), DIR_E);

        // -90
        assert_eq!(DIR_E.rotate(-90), DIR_N);
        assert_eq!(DIR_N.rotate(-90), DIR_W);
        assert_eq!(DIR_W.rotate(-90), DIR_S);
        assert_eq!(DIR_S.rotate(-90), DIR_E);

        // 270
        assert_eq!(DIR_E.rotate(270), DIR_N);
        assert_eq!(DIR_S.rotate(270), DIR_E);
        assert_eq!(DIR_W.rotate(270), DIR_S);
        assert_eq!(DIR_N.rotate(270), DIR_W);

        // -270
        assert_eq!(DIR_E.rotate(-270), DIR_S);
        assert_eq!(DIR_S.rotate(-270), DIR_W);
        assert_eq!(DIR_W.rotate(-270), DIR_N);
        assert_eq!(DIR_N.rotate(-270), DIR_E);

        // Waypoint
        assert_eq!(Vec2::new(10, -4).rotate(90), Vec2::new(4, 10));
        assert_eq!(Vec2::new(10, -4).rotate(180), Vec2::new(-10, 4));
        assert_eq!(Vec2::new(10, -4).rotate(270), Vec2::new(-4, -10));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 25);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT)), 286);
    }
}
