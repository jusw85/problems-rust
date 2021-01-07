// https://adventofcode.com/2020/day/2
//
// --- Day 2: Password Philosophy ---
//
// Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.
//
// The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.
//
// Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.
//
// To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.
//
// For example, suppose you have the following list:
//
// 1-3 a: abcde
// 1-3 b: cdefg
// 2-9 c: ccccccccc
//
// Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.
//
// In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.
//
// How many passwords are valid according to their policies?
//
// Your puzzle answer was 528.
// --- Part Two ---
//
// While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.
//
// The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.
//
// Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.
//
// Given the same example list from above:
//
//     1-3 a: abcde is valid: position 1 contains a and position 3 does not.
//     1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
//     2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
//
// How many passwords are valid according to the new interpretation of the policies?
//
// Your puzzle answer was 497.

use std::fs;
use std::str::FromStr;

use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2020/day2")?;
    let passwords = input.lines()
        .map(|line| line.parse::<Password>())
        .collect::<Result<Vec<_>>>()?;

    let count = passwords.iter()
        .filter(|p| p.is_valid1())
        .count();
    println!("{}", count);

    let count = passwords.iter()
        .filter(|p| p.is_valid2())
        .count();
    println!("{}", count);
    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Password {
    n1: usize,
    n2: usize,
    c: u8,
    s: Vec<u8>,
}

impl Password {
    fn is_valid1(&self) -> bool {
        let Password { n1, n2, c, s } = self;
        let count = s.iter()
            .filter(|&e| e == c)
            .count();
        count >= *n1 && count <= *n2
    }

    fn is_valid2(&self) -> bool {
        let Password { n1, n2, c, s } = self;
        let c1 = s[n1 - 1] == *c;
        let c2 = s[n2 - 1] == *c;
        c1 ^ c2
    }
}

impl FromStr for Password {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
        }
        for cap in RE.captures_iter(s) {
            let n1 = cap[1].parse::<usize>()?;
            let n2 = cap[2].parse::<usize>()?;
            let c = cap[3].as_bytes()[0];
            let s = cap[4].as_bytes().to_vec();
            let res = Password { n1, n2, c, s };
            return Ok(res);
        }
        anyhow::bail!("unable to parse password")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let p = "1-3 a: abcde".parse::<Password>()?;
        assert!(p.is_valid1());
        assert!(p.is_valid2());

        let p = "1-3 b: cdefg".parse::<Password>()?;
        assert!(!p.is_valid1());
        assert!(!p.is_valid2());

        let p = "2-9 c: ccccccccc".parse::<Password>()?;
        assert!(p.is_valid1());
        assert!(!p.is_valid2());
        Ok(())
    }
}
