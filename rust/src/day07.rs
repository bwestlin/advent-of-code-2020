extern crate regex;
#[macro_use] extern crate lazy_static;
extern crate utils;

use std::env;
use std::iter::FromIterator;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::str::FromStr;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use utils::*;

type Input = Vec<BagRule>;

#[derive(Debug)]
struct BagRule {
    bag: String,
    contains: HashMap<String, u32>
}

fn solve(input: &Input) -> (usize, u32) {
    let shiny_gold = "shiny gold".to_string();
    let rule_by_bag: HashMap<&String, &BagRule> = HashMap::from_iter(input.iter().map(|rule| (&rule.bag, rule)));
    let mut queue = rule_by_bag.keys().cloned().collect::<VecDeque<_>>();
    let mut contains_shiny_gold = HashSet::new();
    let mut bag_counts = HashMap::new();

    while let Some(bag) = queue.pop_front() {
        let mut counts = 0;

        if let Some(rule) = rule_by_bag.get(bag) {
            if !rule.contains.iter().all(|(bag, _)| bag_counts.contains_key(bag)) {
                queue.push_back(bag);
                continue;
            }

            for (contained_bag, n_contained) in &rule.contains {
                if contained_bag == &shiny_gold || contains_shiny_gold.contains(&contained_bag) {
                    contains_shiny_gold.insert(bag);
                } else {
                    counts += n_contained + n_contained * bag_counts.get(&contained_bag).unwrap_or(&0);
                }
            }
        }
        bag_counts.insert(bag, counts);
    }

    (
        contains_shiny_gold.len(),
        *bag_counts.get(&shiny_gold).unwrap_or(&0)
    )
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let (part1, part2) = solve(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
    });
}

impl FromStr for BagRule {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(.+ .+) bags contain (.+).$").unwrap();
        }
        let caps = RE.captures(s).unwrap();

        let bag = caps[1].to_string();
        let mut contains: HashMap<String, u32> = HashMap::new();

        for contain in caps[2].split(", ") {
            let mut parts = contain.split_ascii_whitespace();

            if let Some(first) = parts.next() {
                if let Ok(cnt) = first.parse::<u32>() {
                    let bag = parts.take(2).collect::<Vec<_>>().join(" ");
                    contains.insert(bag, cnt);
                }
            }
        }

        Ok(BagRule {
            bag,
            contains
        })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    Ok(reader.lines().map(|l| l.unwrap().parse::<BagRule>().unwrap()).collect())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";

    const INPUT2: &'static str =
       "shiny gold bags contain 2 dark red bags.
        dark red bags contain 2 dark orange bags.
        dark orange bags contain 2 dark yellow bags.
        dark yellow bags contain 2 dark green bags.
        dark green bags contain 2 dark blue bags.
        dark blue bags contain 2 dark violet bags.
        dark violet bags contain no other bags.";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&as_input(INPUT)).0, 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(&as_input(INPUT)).1, 32);
        assert_eq!(solve(&as_input(INPUT2)).1, 126);
    }
}
