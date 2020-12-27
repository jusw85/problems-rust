// https://adventofcode.com/2019/day/9
//
// --- Day 9: Sensor Boost ---
//
// You've just said goodbye to the rebooted rover and left Mars when you receive a faint distress signal coming from the asteroid belt. It must be the Ceres monitoring station!
//
// In order to lock on to the signal, you'll need to boost your sensors. The Elves send up the latest BOOST program - Basic Operation Of System Test.
//
// While BOOST (your puzzle input) is capable of boosting your sensors, for tenuous safety reasons, it refuses to do so until the computer it runs on passes some checks to demonstrate it is a complete Intcode computer.
//
// Your existing Intcode computer is missing one key feature: it needs support for parameters in relative mode.
//
// Parameters in mode 2, relative mode, behave very similarly to parameters in position mode: the parameter is interpreted as a position. Like position mode, parameters in relative mode can be read from or written to.
//
// The important difference is that relative mode parameters don't count from address 0. Instead, they count from a value called the relative base. The relative base starts at 0.
//
// The address a relative mode parameter refers to is itself plus the current relative base. When the relative base is 0, relative mode parameters and position mode parameters with the same value refer to the same address.
//
// For example, given a relative base of 50, a relative mode parameter of -7 refers to memory address 50 + -7 = 43.
//
// The relative base is modified with the relative base offset instruction:
//
//     Opcode 9 adjusts the relative base by the value of its only parameter. The relative base increases (or decreases, if the value is negative) by the value of the parameter.
//
// For example, if the relative base is 2000, then after the instruction 109,19, the relative base would be 2019. If the next instruction were 204,-34, then the value at address 1985 would be output.
//
// Your Intcode computer will also need a few other capabilities:
//
//     The computer's available memory should be much larger than the initial program. Memory beyond the initial program starts with the value 0 and can be read or written like any other memory. (It is invalid to try to access memory at a negative address, though.)
//     The computer should have support for large numbers. Some instructions near the beginning of the BOOST program will verify this capability.
//
// Here are some example programs that use these features:
//
//     109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99 takes no input and produces a copy of itself as output.
//     1102,34915192,34915192,7,4,7,99,0 should output a 16-digit number.
//     104,1125899906842624,99 should output the large number in the middle.
//
// The BOOST program will ask for a single input; run it in test mode by providing it the value 1. It will perform a series of checks on each opcode, output any opcodes (and the associated parameter modes) that seem to be functioning incorrectly, and finally output a BOOST keycode.
//
// Once your Intcode computer is fully functional, the BOOST program should report no malfunctioning opcodes when run in test mode; it should only output a single value, the BOOST keycode. What BOOST keycode does it produce?
//
// Your puzzle answer was 2890527621.
// --- Part Two ---
//
// You now have a complete Intcode computer.
//
// Finally, you can lock on to the Ceres distress signal! You just need to boost your sensors using the BOOST program.
//
// The program runs in sensor boost mode by providing the input instruction the value 2. Once run, it will boost the sensors automatically, but it might take a few seconds to complete the operation on slower hardware. In sensor boost mode, the program will output a single value: the coordinates of the distress signal.
//
// Run the BOOST program in sensor boost mode. What are the coordinates of the distress signal?
//
// Your puzzle answer was 66772.

use std::fs;

use anyhow::Context;

use crate::prog::Prog;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2019/day9")?;
    let nums: Vec<i64> =
        input.trim()
            .split(',')
            .map(|s| s.parse()
                .with_context(|| format!("Failed to parse {}", s)))
            .collect::<Result<_>>()?;

    part1(&nums)?;
    part2(&nums)?;
    Ok(())
}

fn part1(nums: &Vec<i64>) -> Result<()> {
    let outs = Prog::exec_once(nums.clone(), 1)?;
    println!("{:?}", outs);
    Ok(())
}

fn part2(nums: &Vec<i64>) -> Result<()> {
    let outs = Prog::exec_once(nums.clone(), 2)?;
    println!("{:?}", outs);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let nums = vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99];
        let outs = Prog::exec_once(nums.clone(), 1)?;
        assert_eq!(nums, outs);

        let nums = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let outs = Prog::exec_once(nums.clone(), 1)?;
        assert_eq!(16, outs[0].to_string().len());

        let nums = vec![104, 1125899906842624, 99];
        let outs = Prog::exec_once(nums, 1)?;
        assert_eq!(vec![1125899906842624], outs);
        Ok(())
    }
}

mod prog {
    use std::convert::TryFrom;
    use std::iter;
    use std::sync::mpsc::{Receiver, SyncSender};
    use std::sync::mpsc;

    use super::Result;

    pub struct Prog {
        nums: Vec<i64>,
        i: usize,
        input: Receiver<i64>,
        outputs: Vec<SyncSender<i64>>,
        relbase: i64,
    }

    impl Prog {
        pub fn exec_once(nums: Vec<i64>, input_val: i64) -> Result<Vec<i64>> {
            let (input, from_input) = mpsc::sync_channel(1);
            let (output, from_output) = mpsc::sync_channel(64);

            input.send(input_val)?;
            Prog::new(nums, from_input, [output].to_vec()).exec()?;
            let result: Vec<_> = from_output.try_iter().collect();
            Ok(result)
        }

        pub fn new(nums: Vec<i64>,
                   input: Receiver<i64>,
                   outputs: Vec<SyncSender<i64>>) -> Prog {
            Prog {
                nums,
                i: 0,
                input,
                outputs,
                relbase: 0,
            }
        }

        pub fn exec(&mut self) -> Result<()> {
            while self.i < self.nums.len() {
                let op = self.nums[self.i] % 100;
                if op == 99 {
                    break;
                }
                let modes = (self.nums[self.i] / 100).to_string();
                let mut modes = modes.chars().rev()
                    .map(|i| i.to_digit(10).unwrap())
                    .chain(iter::repeat(0));

                match op {
                    1 | 2 | 7 | 8 => {
                        let num1 = self.read_val(modes.next().unwrap(), self.i + 1)?;
                        let num2 = self.read_val(modes.next().unwrap(), self.i + 2)?;
                        let pos = self.read_pos(modes.next().unwrap(), self.i + 3)?;
                        self.nums[pos] = match op {
                            1 => num1 + num2,
                            2 => num1 * num2,
                            7 => if num1 < num2 { 1 } else { 0 },
                            8 => if num1 == num2 { 1 } else { 0 },
                            _ => unreachable!(),
                        };
                        self.i += 4;
                    }
                    3 => {
                        let pos = self.read_pos(modes.next().unwrap(), self.i + 1)?;
                        self.nums[pos] = self.input.recv()?;
                        self.i += 2;
                    }
                    4 => {
                        let num = self.read_val(modes.next().unwrap(), self.i + 1)?;

                        let mut sent = false;
                        for _ in self.outputs.iter()
                            .skip_while(|output| output.send(num).is_err()) {
                            sent = true;
                            break;
                        }
                        if !sent { anyhow::bail!("all outputs failed"); }

                        self.i += 2;
                    }
                    5 | 6 => {
                        let num = self.read_val(modes.next().unwrap(), self.i + 1)?;
                        let cond = match op {
                            5 => num != 0,
                            6 => num == 0,
                            _ => unreachable!(),
                        };
                        if cond {
                            let pos = self.read_val(modes.next().unwrap(), self.i + 2)?;
                            self.i = usize::try_from(pos)?
                        } else {
                            self.i += 3;
                        }
                    }
                    9 => {
                        let num = self.read_val(modes.next().unwrap(), self.i + 1)?;
                        self.relbase += num;
                        self.i += 2;
                    }
                    _ => anyhow::bail!("Unrecognized opcode: {}", op),
                };
                self.check_extend(self.i);
            }
            Ok(())
        }

        fn read_val(&mut self, mode: u32, idx: usize) -> Result<i64> {
            let idx = match mode {
                0 => usize::try_from(self.get_raw_val(idx))?,
                1 => idx,
                2 => usize::try_from(self.get_raw_val(idx) + self.relbase)?,
                _ => anyhow::bail!("Unrecognized parameter mode: {}", mode),
            };
            Ok(self.get_raw_val(idx))
        }

        fn read_pos(&mut self, mode: u32, idx: usize) -> Result<usize> {
            let idx = match mode {
                0 => usize::try_from(self.get_raw_val(idx))?,
                2 => usize::try_from(self.get_raw_val(idx) + self.relbase)?,
                _ => anyhow::bail!("Invalid parameter mode for pos: {}", mode),
            };
            self.check_extend(idx);
            Ok(idx)
        }

        fn check_extend(&mut self, idx: usize) {
            let Prog { nums, .. } = self;
            if idx >= nums.len() {
                let extend_len = idx - nums.len() + 1;
                nums.extend(iter::repeat(0).take(extend_len));
            }
        }

        fn get_raw_val(&mut self, idx: usize) -> i64 {
            self.check_extend(idx);
            self.nums[idx]
        }
    }
}
