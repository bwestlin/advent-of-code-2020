extern crate utils;

use std::env;
use std::iter::FromIterator;
use std::collections::HashSet;
use std::collections::HashMap;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<Passport>;

#[derive(Debug)]
struct Passport {
    fields: HashMap<String, String>
}

impl Passport {
    fn contain_req_fields(&self, req_fields: &HashSet<&str>) -> bool {
        let keys = self.fields.keys().map(|k| k.as_str()).collect::<HashSet<_>>();
        req_fields.difference(&keys).count() == 0
    }
    fn req_fields_valid(&self, req_fields: &HashSet<&str>) -> bool {
        req_fields.iter()
            .all(|key| {
                self.fields.get(&key.to_string())
                    .map(|value| password_field_valid(key, value.as_str()))
                    .unwrap_or(false)
            })
    }
}

fn password_field_valid(field: &str, value: &str) -> bool {
    match field {
        "byr" => {
            (1920..=2002).contains(&value.parse::<u32>().unwrap())
        },
        "iyr" => {
            (2010..=2020).contains(&value.parse::<u32>().unwrap())
        },
        "eyr" => {
            (2020..=2030).contains(&value.parse::<u32>().unwrap())
        },
        "hgt" => {
            if value.len() >= 3 {
                let unit = &value[(value.len() - 2)..];
                let value = value[0..(value.len() - 2)].parse::<u32>().unwrap();
                match unit {
                    "cm" => (150..=193).contains(&value),
                    "in" => (59..=76).contains(&value),
                    _ => false
                }
            } else {
                false
            }
        },
        "hcl" => {
            let mut chrs = value.chars();
            chrs.next() == Some('#') && chrs.all(|chr| chr >= '0' && chr <= '9' || chr >= 'a' && chr <= 'f')
        },
        "ecl" => {
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|&ecl| ecl == value)
        },
        "pid" => {
            let mut chrs = value.chars();
            value.len() == 9 && chrs.all(|chr| chr >= '0' && chr <= '9')
        },
        _ => unreachable!()
    }
}

fn solve(input: &Input) -> (usize, usize) {
    let req_fields = HashSet::from_iter(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().cloned());

    input.iter()
        .fold((0, 0), |(p1, p2), passport| {
            (
                p1 + if passport.contain_req_fields(&req_fields) { 1 } else { 0 },
                p2 + if passport.req_fields_valid(&req_fields) { 1 } else { 0 }
            )
        })
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
    let mut passports = vec![];
    let mut curr_fields: HashMap<String, String> = HashMap::new();

    for line in reader.lines().map(|l| l.unwrap()) {
        let line = line.as_str();

        if line.trim().len() == 0 {
            let mut fields: HashMap<String, String> = HashMap::new();
            std::mem::swap(&mut curr_fields, &mut fields);
            passports.push(Passport { fields });
        } else {
            for valuepair in line.split_ascii_whitespace() {
                let mut iter = valuepair.split(':');
                let key = iter.next().unwrap().to_string();
                let val = iter.next().unwrap().to_string();
                curr_fields.insert(key, val);
            }
        }
    }

    if curr_fields.len() > 0 {
        let mut fields: HashMap<String, String> = HashMap::new();
        std::mem::swap(&mut curr_fields, &mut fields);
        passports.push(Passport { fields });
    }

    Ok(passports)
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_P1: &'static str =
       "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in";

    const INPUT_P2_INVALID: &'static str =
       "eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

        iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946

        hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

        hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007";

    const INPUT_P2_VALID: &'static str =
       "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f

        eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

        hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022

        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&as_input(INPUT_P1)).0, 2);
    }

    #[test]
    fn test_password_field_valid() {
        assert_eq!(password_field_valid("byr", "2002"), true);
        assert_eq!(password_field_valid("byr", "2003"), false);

        assert_eq!(password_field_valid("hgt", "60in"), true);
        assert_eq!(password_field_valid("hgt", "190cm"), true);
        assert_eq!(password_field_valid("hgt", "190in"), false);
        assert_eq!(password_field_valid("hgt", "190"), false);

        assert_eq!(password_field_valid("hcl", "#123abc"), true);
        assert_eq!(password_field_valid("hcl", "#123abz"), false);
        assert_eq!(password_field_valid("hcl", "123abc"), false);

        assert_eq!(password_field_valid("ecl", "brn"), true);
        assert_eq!(password_field_valid("ecl", "wat"), false);

        assert_eq!(password_field_valid("pid", "000000001"), true);
        assert_eq!(password_field_valid("pid", "0123456789"), false);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(&as_input(INPUT_P2_INVALID)).1, 0);
        assert_eq!(solve(&as_input(INPUT_P2_VALID)).1, 4);
    }
}
