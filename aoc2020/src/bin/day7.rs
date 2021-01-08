// https://adventofcode.com/2020/day/7
//
// --- Day 7: Handy Haversacks ---
//
// You land at the regional airport in time for your next flight. In fact, it looks like you'll even have time to grab some food: all flights are currently delayed due to issues in luggage processing.
//
// Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents; bags must be color-coded and must contain specific quantities of other color-coded bags. Apparently, nobody responsible for these regulations considered how long they would take to enforce!
//
// For example, consider the following rules:
//
// light red bags contain 1 bright white bag, 2 muted yellow bags.
// dark orange bags contain 3 bright white bags, 4 muted yellow bags.
// bright white bags contain 1 shiny gold bag.
// muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
// shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
// dark olive bags contain 3 faded blue bags, 4 dotted black bags.
// vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
// faded blue bags contain no other bags.
// dotted black bags contain no other bags.
//
// These rules specify the required contents for 9 bag types. In this example, every faded blue bag is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.
//
// You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many different bag colors would be valid for the outermost bag? (In other words: how many colors can, eventually, contain at least one shiny gold bag?)
//
// In the above rules, the following options would be available to you:
//
//     A bright white bag, which can hold your shiny gold bag directly.
//     A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
//     A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
//     A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
//
// So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.
//
// How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is quite long; make sure you get all of it.)
//
// Your puzzle answer was 257.
// --- Part Two ---
//
// It's getting pretty expensive to fly these days - not because of ticket prices, but because of the ridiculous number of bags you need to buy!
//
// Consider again your shiny gold bag and the rules from the above example:
//
//     faded blue bags contain 0 other bags.
//     dotted black bags contain 0 other bags.
//     vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
//     dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.
//
// So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2 vibrant plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!
//
// Of course, the actual rules have a small chance of going several levels deeper than this example; be sure to count all of the bags, even if the nesting becomes topologically impractical!
//
// Here's another example:
//
// shiny gold bags contain 2 dark red bags.
// dark red bags contain 2 dark orange bags.
// dark orange bags contain 2 dark yellow bags.
// dark yellow bags contain 2 dark green bags.
// dark green bags contain 2 dark blue bags.
// dark blue bags contain 2 dark violet bags.
// dark violet bags contain no other bags.
//
// In this example, a single shiny gold bag must contain 126 other bags.
//
// How many individual bags are required inside your single shiny gold bag?
//
// Your puzzle answer was 1038.

use std::collections::{HashMap, HashSet};
use std::fs;

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2020/day7")?;
    let rules = parse(&input);
    println!("{}", count_outer_colour(&rules, "shiny gold"));
    println!("{}", count_inner_bag(&rules, "shiny gold"));
    Ok(())
}

fn count_inner_bag(rules: &Rules, initial_col: &str) -> u32 {
    fn count_rec<'a>(col: &'a str,
                     mut counts: &mut HashMap<&'a str, u32>,
                     graph: &'a HashMap<String, Vec<(u32, String)>>) -> u32 {
        if counts.contains_key(col) {
            return counts[col];
        }
        let mut res = 0;
        for (num, adj_col) in graph[col].iter() {
            res += num + (num * count_rec(adj_col, &mut counts, graph));
        }
        counts.insert(col, res);
        res
    }
    count_rec(initial_col, &mut HashMap::new(), &rules.f)
}

fn count_outer_colour(rules: &Rules, initial_col: &str) -> usize {
    let graph = &rules.b;
    let mut q = Vec::new();
    let mut visited = HashSet::new();

    q.push(initial_col);
    visited.insert(initial_col);
    while !q.is_empty() {
        let col = q.pop().unwrap();
        for next_col in graph[col].iter() {
            let next_col = &**next_col;
            if !visited.contains(next_col) {
                q.push(next_col);
                visited.insert(next_col);
            }
        }
    }
    visited.len() - 1
}

fn parse(s: &str) -> Rules {
    lazy_static::lazy_static! {
        static ref L_RE: Regex = Regex::new(r"([a-z]+ [a-z]+) bag").unwrap();
        static ref R_RE: Regex = Regex::new(r"([\d]+) ([a-z]+ [a-z]+) bag").unwrap();
    }
    let mut f = HashMap::new();
    let mut b = HashMap::new();
    for (l, r) in s.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.split("contain").collect_tuple().unwrap())
    {
        let col_from = L_RE.captures(l).unwrap()[1].to_string();
        b.entry(col_from.clone()).or_insert(HashSet::new());

        let vec = f.entry(col_from.clone()).or_insert(Vec::new());
        for caps in R_RE.captures_iter(r) {
            let num = caps[1].parse::<u32>().unwrap();
            let col_to = caps[2].to_string();

            b.entry(col_to.clone()).or_insert(HashSet::new())
                .insert(col_from.clone());

            vec.push((num, col_to));
        }
    }
    Rules { f, b }
}

#[derive(Debug, Clone)]
struct Rules {
    f: HashMap<String, Vec<(u32, String)>>,
    b: HashMap<String, HashSet<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.
        ";
        let rules = parse(s);
        assert_eq!(4, count_outer_colour(&rules, "shiny gold"));
        assert_eq!(32, count_inner_bag(&rules, "shiny gold"));

        Ok(())
    }
}
