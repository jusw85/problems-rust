// https://adventofcode.com/2020/day/6
//
// --- Day 6: Custom Customs ---
//
// As your flight approaches the regional airport where you'll switch to a much larger plane, customs declaration forms are distributed to the passengers.
//
// The form asks a series of 26 yes-or-no questions marked a through z. All you need to do is identify the questions for which anyone in your group answers "yes". Since your group is just you, this doesn't take very long.
//
// However, the person sitting next to you seems to be experiencing a language barrier and asks if you can help. For each of the people in their group, you write down the questions for which they answer "yes", one per line. For example:
//
// abcx
// abcy
// abcz
//
// In this group, there are 6 questions to which anyone answered "yes": a, b, c, x, y, and z. (Duplicate answers to the same question don't count extra; each question counts at most once.)
//
// Another group asks for your help, then another, and eventually you've collected answers from every group on the plane (your puzzle input). Each group's answers are separated by a blank line, and within each group, each person's answers are on a single line. For example:
//
// abc
//
// a
// b
// c
//
// ab
// ac
//
// a
// a
// a
// a
//
// b
//
// This list represents answers from five groups:
//
//     The first group contains one person who answered "yes" to 3 questions: a, b, and c.
//     The second group contains three people; combined, they answered "yes" to 3 questions: a, b, and c.
//     The third group contains two people; combined, they answered "yes" to 3 questions: a, b, and c.
//     The fourth group contains four people; combined, they answered "yes" to only 1 question, a.
//     The last group contains one person who answered "yes" to only 1 question, b.
//
// In this example, the sum of these counts is 3 + 3 + 3 + 1 + 1 = 11.
//
// For each group, count the number of questions to which anyone answered "yes". What is the sum of those counts?
//
// Your puzzle answer was 6930.
// --- Part Two ---
//
// As you finish the last group's customs declaration, you notice that you misread one word in the instructions:
//
// You don't need to identify the questions to which anyone answered "yes"; you need to identify the questions to which everyone answered "yes"!
//
// Using the same example as above:
//
// abc
//
// a
// b
// c
//
// ab
// ac
//
// a
// a
// a
// a
//
// b
//
// This list represents answers from five groups:
//
//     In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
//     In the second group, there is no question to which everyone answered "yes".
//     In the third group, everyone answered yes to only 1 question, a. Since some people did not answer "yes" to b or c, they don't count.
//     In the fourth group, everyone answered yes to only 1 question, a.
//     In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.
//
// In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.
//
// For each group, count the number of questions to which everyone answered "yes". What is the sum of those counts?
//
// Your puzzle answer was 3585.

use std::collections::{HashMap, HashSet};
use std::fs;

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2020/day6")?;
    let vs = parse(&input);
    println!("{}", sum_groups(&vs, count_group1));
    println!("{}", sum_groups(&vs, count_group2));
    Ok(())
}

fn sum_groups<F>(vs: &Vec<Vec<String>>, count_fn: F) -> usize
    where F: Fn(&Vec<String>) -> usize {
    vs.iter()
        .map(|v| count_fn(v))
        .sum::<usize>()
}

fn count_group1(v: &Vec<String>) -> usize {
    let mut set = HashSet::new();
    for c in v.iter().flat_map(|line| line.chars()) {
        set.insert(c);
    }
    set.len()
}

fn count_group2(v: &Vec<String>) -> usize {
    let mut map = HashMap::new();
    for c in v.iter().flat_map(|line| line.chars()) {
        *map.entry(c).or_insert(0_usize) += 1;
    }
    map.values()
        .filter(|&&i| i == v.len())
        .count()
}

fn parse(s: &str) -> Vec<Vec<String>> {
    s.split("\n\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.lines()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect_vec())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        abc

        a
        b
        c

        ab
        ac

        a
        a
        a
        a

        b
        ";
        let vs = parse(s);
        assert_eq!(11, sum_groups(&vs, count_group1));
        assert_eq!(6, sum_groups(&vs, count_group2));
        Ok(())
    }
}
