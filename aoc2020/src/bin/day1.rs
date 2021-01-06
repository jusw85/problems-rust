// https://adventofcode.com/2020/day/1
//
// --- Day 1: Report Repair ---
//
// After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.
//
// The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.
//
// To save your vacation, you need to get all fifty stars by December 25th.
//
// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//
// Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.
//
// Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.
//
// For example, suppose your expense report contained the following:
//
// 1721
// 979
// 366
// 299
// 675
// 1456
//
// In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.
//
// Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?
//
// Your puzzle answer was 1013211.
// --- Part Two ---
//
// The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.
//
// Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.
//
// In your expense report, what is the product of the three entries that sum to 2020?
//
// Your puzzle answer was 13891280.

use std::collections::HashSet;
use std::fs;

use anyhow::Context;
use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2020/day1")?;
    let nums = parse(&input)?;
    println!("{:?}", part1(&nums).unwrap());
    println!("{:?}", part2(&nums).unwrap());
    Ok(())
}

fn parse(s: &str) -> Result<HashSet<i32>> {
    s.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|s| s.parse::<i32>()
            .with_context(|| format!("failed to parse: {}", s)))
        .collect()
}

fn part1(nums: &HashSet<i32>) -> Option<i32> {
    for i in nums.iter() {
        let j = 2020 - i;
        if nums.contains(&j) {
            return Some(i * j);
        }
    }
    None
}

fn part2(nums: &HashSet<i32>) -> Option<i32> {
    for i in nums.iter() {
        for j in nums.iter() {
            let k = 2020 - i - j;
            if nums.contains(&k) {
                return Some(i * j * k);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
            1721
            979
            366
            299
            675
            1456
        ";
        let nums = parse(s)?;
        assert_eq!(514579, part1(&nums).unwrap());
        assert_eq!(241861950, part2(&nums).unwrap());
        Ok(())
    }
}
