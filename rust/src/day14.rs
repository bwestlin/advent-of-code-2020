extern crate utils;

use std::env;
use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<Instruction>;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum MaskOp {
    NoOp, Zero, One
}

struct BitMask {
    bits: [MaskOp; 36]
}

impl BitMask {
    fn new() -> Self {
        BitMask { bits: [MaskOp::NoOp; 36] }
    }

    fn from_value(value: &u64) -> Self {
        use MaskOp::*;
        let mut bits = [MaskOp::Zero; 36];

        for i in 0..36 {
            bits[i] = match (value >> (35 - i)) & 1 {
                0 => Zero,
                1 => One,
                _ => unreachable!(),
            }
        }

        BitMask { bits }
    }

    fn decode(&self, mut value: u64) -> u64 {
        use MaskOp::*;

        for (i, &op) in self.bits.iter().enumerate() {
            match op {
                Zero => {
                     value &= std::u64::MAX ^ (1 << (35 - i));
                },
                One  => {
                    value |= 1 << (35 - i)
                },
                _ => {}
            }
        }
        value
    }

    fn addr_decode(&self, addr: &u64) -> BitMask {
        use MaskOp::*;
        let mut addr_mask = BitMask::from_value(addr);

        for i in 0..self.bits.len() {
            match self.bits[i] {
                One  => addr_mask.bits[i] = One,
                NoOp => addr_mask.bits[i] = NoOp,
                _ => {}
            }
        }
        addr_mask
    }

    fn bits_to_s(&self) -> String {
        use MaskOp::*;
        fn mask_op(mask_op: &MaskOp) -> char {
            match mask_op {
                NoOp => 'X',
                Zero => '0',
                One  => '1',
            }
        }
        self.bits.iter().map(mask_op).collect()
    }
}

impl fmt::Debug for BitMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BitMask")
         .field("bits", &self.bits_to_s())
         .finish()
    }
}

#[derive(Debug)]
enum Instruction {
    SetMask(BitMask), MemAssign(u64, u64)
}

fn part1(input: &Input) -> u64 {
    use Instruction::*;

    let mut mem = HashMap::new();
    let mut curr_mask = &BitMask::new();

    for ins in input {
        match ins {
            SetMask(mask) => curr_mask = mask,
            MemAssign(addr, value) => {
                let result = curr_mask.decode(*value);

                #[cfg(feature = "print")] {
                    println!("value:   {:036b}  (decimal {})", value, value);
                    println!("mask:    {})", curr_mask.bits_to_s());
                    println!("value:   {:036b}  (decimal {})\n", result, result);
                }

                mem.insert(addr, result);
            }
        }
    }

    mem.values().sum::<u64>()
}

fn mask_to_addresses(mask: &BitMask) -> Vec<u64> {
    use MaskOp::*;
    let mut current = mask.bits.iter().enumerate()
        .fold(0_u64, |mut v, (i, &b)| {
            if b == One {
                v |= 1 << (35 - i);
            }
            v
        });

    let mut addrs = vec![];
    addrs.push(current);

    let floating_idxs = mask.bits.iter().enumerate().filter(|(_, &op)| op == NoOp).map(|(i, _)| i).collect::<Vec<_>>();
    let mut floating_idx = floating_idxs.len() - 1;

    'outer: loop {
        let bit_idx = floating_idxs[floating_idx];
        let bit = current >> (35 - bit_idx) & 1;

        if bit == 0 {
            current ^= 1 << (35 - bit_idx);
        }
        else {
            current &= std::u64::MAX ^ (1 << (35 - bit_idx));
            floating_idx -= 1;

            loop {
                let bit_idx = floating_idxs[floating_idx];
                let bit = current >> (35 - bit_idx) & 1;

                if bit == 1 {
                    current &= std::u64::MAX ^ (1 << (35 - bit_idx));
                    if floating_idx == 0 {
                        break 'outer;
                    }
                    floating_idx -= 1;
                } else {
                    current ^= 1 << (35 - bit_idx);
                    floating_idx = floating_idxs.len() - 1;
                    break;
                }
            }
        }
        addrs.push(current);
    }

    addrs
}

fn part2(input: &Input) -> u64 {
    use Instruction::*;

    let mut mem = HashMap::new();
    let mut curr_mask = &BitMask::new();

    for ins in input {
        match ins {
            SetMask(mask) => curr_mask = mask,
            MemAssign(addr, value) => {
                let result = curr_mask.addr_decode(addr);

                #[cfg(feature = "print")] {
                    println!("address: {:036b}  (decimal {})", addr, addr);
                    println!("mask:    {})", curr_mask.bits_to_s());
                    println!("result:  {}\n", result.bits_to_s());
                }

                for a in mask_to_addresses(&result) {
                    mem.insert(a, *value);
                }
            }
        }
    }

    mem.values().sum::<u64>()
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
        use Instruction::*;
        use MaskOp::*;
        let mut parts = s.split('=').map(|s| s.trim());
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();

        let ins =
            if s.starts_with("mask") {
                let mut bitmask = BitMask::new();
                for (i, c) in b.chars().enumerate() {
                    bitmask.bits[i] = match c {
                        'X' => NoOp,
                        '0' => Zero,
                        '1' => One,
                        _ => unreachable!()
                    };
                }
                SetMask(bitmask)
            } else {
                let addr = &a["mem[".len()..(a.len() - 1)];
                MemAssign(addr.parse::<u64>()?, b.parse::<u64>()?)
            };

        Ok(ins)
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

    const INPUT1: &'static str =
       "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 0";

    const INPUT2: &'static str =
       "mask = 000000000000000000000000000000X1001X
        mem[42] = 100
        mask = 00000000000000000000000000000000X0XX
        mem[26] = 1";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT1)), 165);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT2)), 208);
    }
}
