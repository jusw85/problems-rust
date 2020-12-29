// https://adventofcode.com/2019/day/14
//
// --- Day 14: Space Stoichiometry ---
//
// As you approach the rings of Saturn, your ship's low fuel indicator turns on. There isn't any fuel here, but the rings have plenty of raw material. Perhaps your ship's Inter-Stellar Refinery Union brand nanofactory can turn these raw materials into fuel.
//
// You ask the nanofactory to produce a list of the reactions it can perform that are relevant to this process (your puzzle input). Every reaction turns some quantities of specific input chemicals into some quantity of an output chemical. Almost every chemical is produced by exactly one reaction; the only exception, ORE, is the raw material input to the entire process and is not produced by a reaction.
//
// You just need to know how much ORE you'll need to collect before you can produce one unit of FUEL.
//
// Each reaction gives specific quantities for its inputs and output; reactions cannot be partially run, so only whole integer multiples of these quantities can be used. (It's okay to have leftover chemicals when you're done, though.) For example, the reaction 1 A, 2 B, 3 C => 2 D means that exactly 2 units of chemical D can be produced by consuming exactly 1 A, 2 B and 3 C. You can run the full reaction as many times as necessary; for example, you could produce 10 D by consuming 5 A, 10 B, and 15 C.
//
// Suppose your nanofactory produces the following list of reactions:
//
// 10 ORE => 10 A
// 1 ORE => 1 B
// 7 A, 1 B => 1 C
// 7 A, 1 C => 1 D
// 7 A, 1 D => 1 E
// 7 A, 1 E => 1 FUEL
//
// The first two reactions use only ORE as inputs; they indicate that you can produce as much of chemical A as you want (in increments of 10 units, each 10 costing 10 ORE) and as much of chemical B as you want (each costing 1 ORE). To produce 1 FUEL, a total of 31 ORE is required: 1 ORE to produce 1 B, then 30 more ORE to produce the 7 + 7 + 7 + 7 = 28 A (with 2 extra A wasted) required in the reactions to convert the B into C, C into D, D into E, and finally E into FUEL. (30 A is produced because its reaction requires that it is created in increments of 10.)
//
// Or, suppose you have the following list of reactions:
//
// 9 ORE => 2 A
// 8 ORE => 3 B
// 7 ORE => 5 C
// 3 A, 4 B => 1 AB
// 5 B, 7 C => 1 BC
// 4 C, 1 A => 1 CA
// 2 AB, 3 BC, 4 CA => 1 FUEL
//
// The above list of reactions requires 165 ORE to produce 1 FUEL:
//
//     Consume 45 ORE to produce 10 A.
//     Consume 64 ORE to produce 24 B.
//     Consume 56 ORE to produce 40 C.
//     Consume 6 A, 8 B to produce 2 AB.
//     Consume 15 B, 21 C to produce 3 BC.
//     Consume 16 C, 4 A to produce 4 CA.
//     Consume 2 AB, 3 BC, 4 CA to produce 1 FUEL.
//
// Here are some larger examples:
//
//     13312 ORE for 1 FUEL:
//
//     157 ORE => 5 NZVS
//     165 ORE => 6 DCFZ
//     44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
//     12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
//     179 ORE => 7 PSHF
//     177 ORE => 5 HKGWZ
//     7 DCFZ, 7 PSHF => 2 XJWVT
//     165 ORE => 2 GPVTF
//     3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
//
//     180697 ORE for 1 FUEL:
//
//     2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
//     17 NVRVD, 3 JNWZP => 8 VPVL
//     53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
//     22 VJHF, 37 MNCFX => 5 FWMGM
//     139 ORE => 4 NVRVD
//     144 ORE => 7 JNWZP
//     5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
//     5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
//     145 ORE => 6 MNCFX
//     1 NVRVD => 8 CXFTF
//     1 VJHF, 6 MNCFX => 4 RFSQX
//     176 ORE => 6 VJHF
//
//     2210736 ORE for 1 FUEL:
//
//     171 ORE => 8 CNZTR
//     7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
//     114 ORE => 4 BHXH
//     14 VRPVC => 6 BMBT
//     6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
//     6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
//     15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
//     13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
//     5 BMBT => 4 WPTQ
//     189 ORE => 9 KTJDG
//     1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
//     12 VRPVC, 27 CNZTR => 2 XDBXC
//     15 KTJDG, 12 BHXH => 5 XCVML
//     3 BHXH, 2 VRPVC => 7 MZWV
//     121 ORE => 7 VRPVC
//     7 XCVML => 6 RJRHP
//     5 BHXH, 4 VRPVC => 5 LTCX
//
// Given the list of reactions in your puzzle input, what is the minimum amount of ORE required to produce exactly 1 FUEL?
//
// Your puzzle answer was 114125.
// --- Part Two ---
//
// After collecting ORE for a while, you check your cargo hold: 1 trillion (1000000000000) units of ORE.
//
// With that much ore, given the examples above:
//
//     The 13312 ORE-per-FUEL example could produce 82892753 FUEL.
//     The 180697 ORE-per-FUEL example could produce 5586022 FUEL.
//     The 2210736 ORE-per-FUEL example could produce 460664 FUEL.
//
// Given 1 trillion ORE, what is the maximum amount of FUEL you can produce?
//
// Your puzzle answer was 12039407.

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;

use lazy_static::lazy_static;
use regex::Regex;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2019/day14")?;

    let recipes = parse(&input)?;
    let min_ore_per_fuel = get_required_ore(&recipes, 1);
    let ore = 1000000000000i64;
    let max_fuel = max_fuel(&recipes, ore, min_ore_per_fuel);

    println!("{}", min_ore_per_fuel);
    println!("{}", max_fuel);

    Ok(())
}

fn parse(s: &str) -> Result<HashMap<String, Recipe>> {
    fn parse_recipe(s: &str) -> Result<Recipe> {
        const LINE_RE_STR: &str = r"(\d+ [A-Z]+, )*\d+ [A-Z]+ => \d+ [A-Z]+";
        const CHEM_RE_STR: &str = r"(\d+) ([A-Z]+)";
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(LINE_RE_STR).unwrap();
            static ref CHEM_RE: Regex = Regex::new(CHEM_RE_STR).unwrap();
        }
        if !LINE_RE.is_match(s) { anyhow::bail!("invalid line: {}", s); }

        let mut chems = Vec::new();
        for cap in CHEM_RE.captures_iter(s) {
            let num = cap[1].parse::<i64>()?;
            let name = cap[2].to_string();
            let chem = Chem { num, name };
            chems.push(chem);
        }
        let output = chems.pop().unwrap();
        Ok(Recipe { inputs: chems, output })
    }
    let mut res = HashMap::new();
    let recipes = s.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| parse_recipe(line));
    for recipe in recipes {
        let recipe = recipe?;
        res.insert(recipe.output.name.clone(), recipe);
    }
    Ok(res)
}

fn get_required_ore(recipes: &HashMap<String, Recipe>, required_fuel: i64) -> i64 {
    let mut supplies = HashMap::new();
    let mut to_craft = HashSet::new();

    supplies.insert("FUEL", -required_fuel);
    to_craft.insert("FUEL");
    while !to_craft.is_empty() {
        let next_output = take_next_elem(&mut to_craft);
        let recipe = recipes.get(next_output).unwrap();
        let deficitp = supplies.entry(next_output).or_insert(0);
        let deficit = *deficitp;

        let crafts_required =
            (deficit.abs() / recipe.output.num) +
                if deficit % recipe.output.num != 0 { 1 } else { 0 };
        *deficitp += crafts_required * recipe.output.num;

        for input in recipe.inputs.iter() {
            let chem_name = &*input.name;
            let chem_supply = supplies.entry(chem_name).or_insert(0);

            *chem_supply -= crafts_required * input.num;
            if *chem_supply < 0 && chem_name != "ORE" {
                to_craft.insert(chem_name);
            }
        }
    }
    -supplies["ORE"]
}

fn take_next_elem<'a, T>(set: &mut HashSet<&'a T>) -> &'a T
    where
        &'a T: Eq + Hash,
        T: Eq + Hash + ?Sized {
    let elem = *set.iter().next().unwrap();
    set.take(elem).unwrap()
}

fn search<F>(func: F, mut left: i64, mut right: i64, target: i64) -> i64
    where F: Fn(i64) -> i64 {
    assert!(left < right);
    assert!(func(left) <= target); // invariant
    assert!(func(right) >= target); // invariant

    while left <= right {
        let mid = (left + right) / 2;
        match func(mid).cmp(&target) {
            Ordering::Less => left = mid + 1,
            Ordering::Equal => return mid,
            Ordering::Greater => right = mid - 1,
        }
    }
    right
}

fn max_fuel(recipes: &HashMap<String, Recipe>,
            ore: i64,
            min_ore_per_fuel: i64) -> i64 {
    let fuel = ore / min_ore_per_fuel;
    let func = |i| get_required_ore(&recipes, i);
    search(func, fuel, fuel * 2, ore)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let ore = 1000000000000i64;
        let s = r"
            9 ORE => 2 A
            8 ORE => 3 B
            7 ORE => 5 C
            3 A, 4 B => 1 AB
            5 B, 7 C => 1 BC
            4 C, 1 A => 1 CA
            2 AB, 3 BC, 4 CA => 1 FUEL
        ";
        let recipes = parse(s)?;
        let min_ore_per_fuel = get_required_ore(&recipes, 1);
        assert_eq!(165, min_ore_per_fuel);

        let s = r"
            157 ORE => 5 NZVS
            165 ORE => 6 DCFZ
            44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
            12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
            179 ORE => 7 PSHF
            177 ORE => 5 HKGWZ
            7 DCFZ, 7 PSHF => 2 XJWVT
            165 ORE => 2 GPVTF
            3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
        ";
        let recipes = parse(s)?;
        let min_ore_per_fuel = get_required_ore(&recipes, 1);
        assert_eq!(13312, min_ore_per_fuel);
        assert_eq!(82892753, max_fuel(&recipes, ore, min_ore_per_fuel));

        let s = r"
            2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
            17 NVRVD, 3 JNWZP => 8 VPVL
            53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
            22 VJHF, 37 MNCFX => 5 FWMGM
            139 ORE => 4 NVRVD
            144 ORE => 7 JNWZP
            5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
            5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
            145 ORE => 6 MNCFX
            1 NVRVD => 8 CXFTF
            1 VJHF, 6 MNCFX => 4 RFSQX
            176 ORE => 6 VJHF
        ";
        let recipes = parse(s)?;
        let min_ore_per_fuel = get_required_ore(&recipes, 1);
        assert_eq!(180697, min_ore_per_fuel);
        assert_eq!(5586022, max_fuel(&recipes, ore, min_ore_per_fuel));

        let s = r"
            171 ORE => 8 CNZTR
            7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
            114 ORE => 4 BHXH
            14 VRPVC => 6 BMBT
            6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
            6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
            15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
            13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
            5 BMBT => 4 WPTQ
            189 ORE => 9 KTJDG
            1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
            12 VRPVC, 27 CNZTR => 2 XDBXC
            15 KTJDG, 12 BHXH => 5 XCVML
            3 BHXH, 2 VRPVC => 7 MZWV
            121 ORE => 7 VRPVC
            7 XCVML => 6 RJRHP
            5 BHXH, 4 VRPVC => 5 LTCX
        ";
        let recipes = parse(s)?;
        let min_ore_per_fuel = get_required_ore(&recipes, 1);
        assert_eq!(2210736, min_ore_per_fuel);
        assert_eq!(460664, max_fuel(&recipes, ore, min_ore_per_fuel));
        Ok(())
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Chem {
    num: i64,
    name: String,
}

#[derive(Clone, Debug)]
struct Recipe {
    inputs: Vec<Chem>,
    output: Chem,
}
