// https://adventofcode.com/2019/day/14
//
//

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2019/day14")?;

    let recipes = parse(&input)?;
    part1(recipes);

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
        let to = chems.pop().unwrap();
        Ok(Recipe { from: chems, to })
    }
    let mut res = HashMap::new();
    let recipes = s.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| parse_recipe(line));
    for recipe in recipes {
        let recipe = recipe?;
        res.insert(recipe.to.name.clone(), recipe);
    }
    Ok(res)
}

fn part1(recipes: HashMap<String, Recipe>) -> i64 {
    let mut supplies = HashMap::new();
    let mut to_craft = HashSet::new();
    for key in recipes.keys() {
        supplies.insert(&**key, 0i64);
    }

    supplies.insert("ORE", 0i64);
    supplies.insert("FUEL", -1);
    to_craft.insert("FUEL");
    while !to_craft.is_empty() {
        let elem = *to_craft.iter().next().unwrap();
        let next_output = to_craft.take(elem).unwrap();

        let recipe = recipes.get(next_output).unwrap();
        let deficit = supplies.get_mut(next_output).unwrap();
        let output_per_craft = recipe.to.num;

        let crafts_required =
            (deficit.abs() / output_per_craft) +
                if *deficit % output_per_craft != 0 { 1 } else { 0 };
        *deficit += crafts_required * output_per_craft;

        for input in recipe.from.iter() {
            let input_per_craft = input.num;
            let chem_name = &*input.name;

            let chem_supply = supplies.get_mut(chem_name).unwrap();
            *chem_supply -= crafts_required * input_per_craft;
            let chem_supply = *chem_supply;
            if chem_supply < 0 && chem_name != "ORE" {
                to_craft.insert(chem_name);
            }
        }
    }
    -supplies["ORE"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
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
        assert_eq!(165, part1(recipes));

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
        assert_eq!(13312, part1(recipes));

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
        assert_eq!(180697, part1(recipes));

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
        assert_eq!(2210736, part1(recipes));
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
    from: Vec<Chem>,
    to: Chem,
}
