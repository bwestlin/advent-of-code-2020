extern crate utils;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

#[derive(Debug)]
struct Input {
    earliest_ts: u64,
    bus_ids: Vec<Option<u64>>
}

fn part1(input: &Input) -> u64 {
    let (least_wait, bus_id) = input.bus_ids.iter()
        .flatten()
        .fold((std::u64::MAX, 0), |(least_wait_time, least_wait_bus_id), &bus_id| {
            let time_left = bus_id - (input.earliest_ts % bus_id);

            if time_left < least_wait_time {
                (time_left, bus_id)
            } else {
                (least_wait_time, least_wait_bus_id)
            }
        });

    least_wait * bus_id
}

fn part2(input: &Input) -> u64 {

    let mut start = 0;
    let mut step = input.bus_ids[0].unwrap();

    for i in 1..input.bus_ids.len() {
        if let Some(bus_id) = input.bus_ids[i] {
            let bus_id = bus_id;

            let mut found = None;

            for t in (start..).step_by(step as usize) {

                if (t + i as u64) % bus_id == 0 {
                    if let Some(found) = found {
                        step = t - found;
                        start = found + step;
                        break;
                    } else {
                        if i == input.bus_ids.len() - 1 {
                            return t;
                        }
                        found = Some(t)
                    }
                }
            }
        }
    }

    0
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
    });
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    let mut lines = reader.lines();
    Ok(Input {
        earliest_ts: lines.next().unwrap()?.parse::<u64>().unwrap(),
        bus_ids: lines.next().unwrap()?.split(',').map(|i| i.parse::<u64>().ok()).collect::<Vec<_>>()
    })
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "939
        7,13,x,x,59,x,31,19";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 295);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT)), 1068781);
        assert_eq!(part2(&as_input("0\n17,x,13,19")), 3417);
        assert_eq!(part2(&as_input("0\n67,7,59,61")), 754018);
        assert_eq!(part2(&as_input("0\n67,x,7,59,61")), 779210);
        assert_eq!(part2(&as_input("0\n67,7,x,59,61")), 1261476);
        assert_eq!(part2(&as_input("0\n1789,37,47,1889")), 1202161486);
    }
}
