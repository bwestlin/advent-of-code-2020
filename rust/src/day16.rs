extern crate utils;

use std::env;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

#[derive(Debug)]
struct Input {
    rules: Vec<TicketRule>,
    tickets: Vec<Ticket>
}

#[derive(Debug)]
struct TicketRule {
    id: String,
    ranges: Vec<RangeInclusive<u32>>
}

impl TicketRule {
    fn valid(&self, value: u32) -> bool {
        self.ranges.iter().any(|r| r.contains(&value))
    }
}

#[derive(Debug)]
struct Ticket {
    fields: Vec<u32>
}

fn part1(input: &Input) -> u32 {
    input.tickets.iter()
        .skip(1)
        .flat_map(|ticket| {
            ticket.fields.iter().filter(|&&f| !input.rules.iter().any(|r| r.valid(f)))
        })
        .sum()
}

fn part2(input: &Input) -> u64 {
    let mut valid_tickets: Vec<_> = input.tickets.iter().skip(1)
        .filter(|ticket| {
            ticket.fields.iter().filter(|&&f| !input.rules.iter().any(|r| r.valid(f))).count() == 0
        })
        .collect();

    let your_ticket = &input.tickets[0];
    valid_tickets.push(your_ticket);

    let n_fields = valid_tickets[0].fields.len();
    let n_rules = input.rules.len();
    let mut possible_rules_by_field: Vec<HashSet<usize>> = vec![HashSet::new(); n_fields];

    for field_idx in 0..n_fields {
        for rule_idx in 0..n_rules {
            let rule = &input.rules[rule_idx];
            if valid_tickets.iter().all(|ticket| rule.valid(ticket.fields[field_idx])) {
                possible_rules_by_field[field_idx].insert(rule_idx);
            }
        }
    }

    let mut field_ids = vec![None; n_fields];
    let mut missing_rule_idxs = (0..n_rules).collect::<HashSet<_>>();

    for i in 0.. {
        let rule_idx = i % n_rules;

        let mut possible_fields = (0..n_fields)
            .filter(|&field_idx| possible_rules_by_field[field_idx].contains(&rule_idx));

        if let Some(field_idx) = possible_fields.next() {

            if possible_fields.next().is_none() {
                field_ids[field_idx] = Some(&input.rules[rule_idx].id[..]);
                possible_rules_by_field[field_idx].clear();
                missing_rule_idxs.remove(&rule_idx);
                if missing_rule_idxs.is_empty() {
                    break;
                }
            }
        }
    }

    field_ids.iter().enumerate()
        .filter(|(_, field_id)| field_id.matches(|fid| fid.starts_with("departure")))
        .map(|(i, _)| your_ticket.fields[i] as u64)
        .product()
}

pub trait OptionExt<T> {
    fn matches<P: FnMut(&T) -> bool>(self, predicate: P) -> bool;
}

impl<T> OptionExt<T> for Option<T> {
    fn matches<P: FnMut(&T) -> bool>(self, predicate: P) -> bool {
        self.filter(predicate).is_some()
    }
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
    });
}

impl FromStr for TicketRule {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':').map(|p| p.trim());
        let id = parts.next().unwrap().to_string();
        let ranges = parts.next().unwrap()
            .split(" or ")
            .map(|r| {
                let mut sp = r.split('-');
                sp.next().unwrap().parse::<u32>().unwrap()..=sp.next().unwrap().parse::<u32>().unwrap()
            })
            .collect();

        Ok(TicketRule { id, ranges })
    }
}

impl FromStr for Ticket {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s.split(',').map(|p| p.parse::<u32>().unwrap()).collect();
        Ok(Ticket { fields })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    let mut lines = reader.lines().flat_map(|l| l.ok());

    let rules = lines.by_ref().take_while(|l| l.len() > 0).map(|l| l.parse::<TicketRule>().unwrap()).collect();
    let mut tickets = vec![
        lines.by_ref().skip(1).next().map(|l| l.parse::<Ticket>().unwrap()).unwrap()
    ];
    for nearby in lines.by_ref().skip(2).map(|l| l.parse::<Ticket>().unwrap()) {
        tickets.push(nearby);
    }

    Ok(Input { rules, tickets })
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50

        your ticket:
        7,1,14

        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 71);
    }
}
