// https://adventofcode.com/2019/day/2

use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let input = fs::read_to_string("input/aoc2019/day2")?;
    let nums: Vec<i32> =
        input.trim()
            .split(',')
            .filter_map(|x| {
                let num = x.parse();
                if let Err(ref e) = num {
                    panic!("{}: {}", x, e);
                }
                num.ok()
            })
            .collect();
    part1(&nums);
    Ok(())
}

fn part1(nums: &Vec<i32>) {
    for num in nums {
        println!("{}", num);
    }
}
