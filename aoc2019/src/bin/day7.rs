// https://adventofcode.com/2019/day/7
//
//
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::{cmp, fs, io};
use std::io::Read;

use anyhow::Context;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2019/day7")?;
    let nums: Vec<i32> =
        input.trim()
            .split(',')
            .map(|s| s.parse()
                .with_context(|| format!("Failed to parse {}", s)))
            .collect::<Result<_>>()?;

    part1(&nums)?;
    // part2(&nums)?;
    Ok(())
}

fn part1(nums: &Vec<i32>) -> Result<()> {
    let phases = [0, 1, 2, 3, 4];
    let (signal, phase) = max_amplified_signal(&nums, &phases)?;
    println!("{:?}", (signal, phase));
    Ok(())
}

fn part2(nums: &Vec<i32>) -> Result<()> {
    // exec(5, &mut nums.clone(), &mut io::stdout())?;
    Ok(())
}

fn amplify_signal(nums: &[i32], phases: &[i32]) -> Result<i32> {
    let mut input = 0;
    for phase in phases {
        let output = prog::exec(*phase, input, &mut nums.to_vec())?;
        anyhow::ensure!(output.len() == 1, anyhow::anyhow!("incorrect output args: {}", output.len()));
        input = output[0];
    }
    Ok(input)
}

fn max_amplified_signal(nums: &[i32], phases: &[i32]) -> Result<(i32, Vec<i32>)> {
    let mut phases = phases.to_vec();
    let mut max_phases: Vec<i32> = Vec::new();
    let mut max_signal = i32::MIN;
    permutohedron::heap_recursive(&mut phases, |phases_perm| -> Result<()> {
        let signal = amplify_signal(nums, phases_perm)?;
        if signal > max_signal {
            max_signal = signal;
            max_phases.clear();
            max_phases.extend_from_slice(phases_perm);
        }
        Ok(())
    })?;
    Ok((max_signal, max_phases))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let phases = [0, 1, 2, 3, 4];

        let nums = vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0];
        let (signal, phase) = max_amplified_signal(&nums, &phases)?;
        assert_eq!(43210, signal);
        assert_eq!([4, 3, 2, 1, 0], &phase[..]);

        let nums = vec![3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23,
                        101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0];
        let (signal, phase) = max_amplified_signal(&nums, &phases)?;
        assert_eq!(54321, signal);
        assert_eq!([0, 1, 2, 3, 4], &phase[..]);

        let nums = vec![3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33,
                        1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0];
        let (signal, phase) = max_amplified_signal(&nums, &phases)?;
        assert_eq!(65210, signal);
        assert_eq!([1, 0, 4, 3, 2], &phase[..]);

        Ok(())
    }

    #[test]
    fn test_loop() -> Result<()> {
        let phases = [9, 8, 7, 6, 5];
        let nums = vec![3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27,
                        2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                        -1, 28, 1005, 28, 6, 99, 0, 0, 5];
        // let out = amplify_signal(&nums, &phases)?;
        let out = prog::exec(9, 0, &mut nums.clone())?;
        println!("{:?}", out);
        // let (signal, phase) = max_amplified_signal(&nums, &phases)?;
        // assert_eq!(43210, signal);
        // assert_eq!([4, 3, 2, 1, 0], &phase[..]);
        Ok(())
    }
}

mod prog {
    use std::convert::TryFrom;
    use std::iter;

    use super::Result;

    pub fn exec(phase: i32,
                input: i32,
                nums: &mut [i32]) -> Result<Vec<i32>> {
        let inputs = [phase, input];
        let mut inputs = inputs.iter();
        let mut output = Vec::new();
        let mut i: usize = 0;
        while i < nums.len() {
            let op = nums[i] % 100;
            if op == 99 {
                break;
            }
            let modes = (nums[i] / 100).to_string();
            let mut modes = modes.chars().rev()
                .map(|i| i.to_digit(10).unwrap())
                .chain(iter::repeat(0));

            match op {
                1 | 2 | 7 | 8 => {
                    let num1 = read_val(modes.next().unwrap(), i + 1, nums)?;
                    let num2 = read_val(modes.next().unwrap(), i + 2, nums)?;
                    let pos = read_pos(modes.next().unwrap(), i + 3, nums)?;
                    nums[pos] = match op {
                        1 => num1 + num2,
                        2 => num1 * num2,
                        7 => if num1 < num2 { 1 } else { 0 },
                        8 => if num1 == num2 { 1 } else { 0 },
                        _ => unreachable!(),
                    };
                    i += 4;
                }
                3 => {
                    let pos = read_pos(modes.next().unwrap(), i + 1, nums)?;
                    nums[pos] = *inputs.next().ok_or_else(|| anyhow::anyhow!("insufficient inputs"))?;
                    i += 2;
                }
                4 => {
                    let num = read_val(modes.next().unwrap(), i + 1, nums)?;
                    output.push(num);
                    i += 2;
                }
                5 | 6 => {
                    let num = read_val(modes.next().unwrap(), i + 1, nums)?;
                    let cond = match op {
                        5 => num != 0,
                        6 => num == 0,
                        _ => unreachable!(),
                    };
                    if cond {
                        let pos = read_val(modes.next().unwrap(), i + 2, nums)?;
                        i = usize::try_from(pos)?
                    } else {
                        i += 3;
                    }
                }
                _ => anyhow::bail!("Unrecognized opcode: {}", op),
            };
        }
        Ok(output)
    }

    fn read_val(mode: u32, idx: usize, nums: &[i32]) -> Result<i32> {
        match mode {
            0 => {
                let idx = usize::try_from(nums[idx])?;
                Ok(nums[idx])
            }
            1 => Ok(nums[idx]),
            _ => anyhow::bail!("Unrecognized parameter mode: {}", mode),
        }
    }

    fn read_pos(mode: u32, idx: usize, nums: &[i32]) -> Result<usize> {
        match mode {
            0 => Ok(usize::try_from(nums[idx])?),
            _ => anyhow::bail!("Invalid parameter mode for pos: {}", mode),
        }
    }
}
