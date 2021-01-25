// https://adventofcode.com/2020/day/21
//
// --- Day 21: Allergen Assessment ---
//
// You reach the train's last stop and the closest you can get to your vacation island without getting wet. There aren't even any boats here, but nothing can stop you now: you build a raft. You just need a few days' worth of food for your journey.
//
// You don't speak the local language, so you can't read any ingredients lists. However, sometimes, allergens are listed in a language you do understand. You should be able to use this information to determine which ingredient contains which allergen and work out which foods are safe to take with you on your trip.
//
// You start by compiling a list of foods (your puzzle input), one food per line. Each line includes that food's ingredients list followed by some or all of the allergens the food contains.
//
// Each allergen is found in exactly one ingredient. Each ingredient contains zero or one allergen. Allergens aren't always marked; when they're listed (as in (contains nuts, shellfish) after an ingredients list), the ingredient that contains each listed allergen will be somewhere in the corresponding ingredients list. However, even if an allergen isn't listed, the ingredient that contains that allergen could still be present: maybe they forgot to label it, or maybe it was labeled in a language you don't know.
//
// For example, consider the following list of foods:
//
// mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
// trh fvjkl sbzzf mxmxvkd (contains dairy)
// sqjhc fvjkl (contains soy)
// sqjhc mxmxvkd sbzzf (contains fish)
//
// The first food in the list has four ingredients (written in a language you don't understand): mxmxvkd, kfcds, sqjhc, and nhms. While the food might contain other allergens, a few allergens the food definitely contains are listed afterward: dairy and fish.
//
// The first step is to determine which ingredients can't possibly contain any of the allergens in any food in your list. In the above example, none of the ingredients kfcds, nhms, sbzzf, or trh can contain an allergen. Counting the number of times any of these ingredients appear in any ingredients list produces 5: they all appear once each except sbzzf, which appears twice.
//
// Determine which ingredients cannot possibly contain any of the allergens in your list. How many times do any of those ingredients appear?
//
// Your puzzle answer was 2635.
// --- Part Two ---
//
// Now that you've isolated the inert ingredients, you should have enough information to figure out which ingredient contains which allergen.
//
// In the above example:
//
//     mxmxvkd contains dairy.
//     sqjhc contains fish.
//     fvjkl contains soy.
//
// Arrange the ingredients alphabetically by their allergen and separate them by commas to produce your canonical dangerous ingredient list. (There should not be any spaces in your canonical dangerous ingredient list.) In the above example, this would be mxmxvkd,sqjhc,fvjkl.
//
// Time to stock your raft with supplies. What is your canonical dangerous ingredient list?
//
// Your puzzle answer was xncgqbcp,frkmp,qhqs,qnhjhn,dhsnxr,rzrktx,ntflq,lgnhmx.

use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

use aoc2020::TrimEmpty;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2020/day21")?;
    let (allergen_ingredients, all_ingredients) = parse(&input);
    let soln = solve(&allergen_ingredients);
    println!("{}", part1(&all_ingredients, &soln));
    println!("{}", part2(&soln));
    Ok(())
}

fn parse(s: &str) -> (HashMap<String, Vec<HashSet<String>>>, Vec<String>) {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"^(.*)\(contains(.*)\)$").unwrap();
    }
    let mut all = Vec::new();
    let mut res = HashMap::new();
    for line in s.lines().trim_empty() {
        let caps = RE.captures(line).unwrap();
        let ingredients = caps[1]
            .split(' ').trim_empty()
            .map(ToString::to_string)
            .collect::<HashSet<_>>();
        all.extend(ingredients.iter().map(ToOwned::to_owned));

        for (k, v) in caps[2].split(',').trim_empty()
            .map(|allergen| (allergen.to_string(), ingredients.clone()))
        {
            res.entry(k).or_insert(Vec::new()).push(v);
        }
    }
    (res, all)
}

fn solve(allergen_ingredients: &HashMap<String, Vec<HashSet<String>>>)
         -> HashMap<String, String>
{
    let mut res = allergen_ingredients.iter()
        .map(|(allergen, ingredients)| (allergen, intersects(ingredients)))
        .collect::<HashMap<_, _>>();

    let mut to_process = res.values()
        .filter(|ingredients| ingredients.len() == 1)
        .map(|ingredients| *ingredients.iter().next().unwrap())
        .collect_vec();

    while !to_process.is_empty() {
        let next = to_process.pop().unwrap();
        for ingredients in res.values_mut()
            .filter(|ingredients| ingredients.len() > 1)
        {
            ingredients.retain(|ingredient| ingredient != &next);
            if ingredients.len() == 1 {
                to_process.push(*ingredients.iter().next().unwrap());
            }
        }
    }
    res.iter().map(|(allergen, ingredient)| {
        assert_eq!(ingredient.len(), 1);
        (allergen.to_string(), ingredient.iter().next().unwrap().to_string())
    }).collect()
}

fn part1(all: &Vec<String>,
         solved: &HashMap<String, String>) -> usize {
    let solved = solved.values().collect::<HashSet<_>>();
    all.iter().filter(|&e| !solved.contains(e)).count()
}

fn part2(solved: &HashMap<String, String>) -> String {
    let mut res = solved.iter().collect_vec();
    res.sort_unstable_by(|(k1, _), (k2, _)| k1.cmp(k2));
    res.iter().map(|(_, v)| v).join(",")
}

fn intersects<T>(sets: &Vec<HashSet<T>>) -> HashSet<&T>
    where
        T: Eq + Hash
{
    if sets.is_empty() {
        return HashSet::new();
    }
    let mut res = sets[0].iter().map(Borrow::borrow).collect::<HashSet<_>>();
    for set in &sets[1..] {
        res.retain(|v| set.contains(v));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
        trh fvjkl sbzzf mxmxvkd (contains dairy)
        sqjhc fvjkl (contains soy)
        sqjhc mxmxvkd sbzzf (contains fish)
        ";
        let (recipes, all) = parse(&s);
        let solved = solve(&recipes);
        assert_eq!(5, part1(&all, &solved));
        assert_eq!("mxmxvkd,sqjhc,fvjkl", part2(&solved));
        Ok(())
    }
}
