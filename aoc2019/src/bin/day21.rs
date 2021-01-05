// https://adventofcode.com/2019/day/21
//
// --- Day 21: Springdroid Adventure ---
//
// You lift off from Pluto and start flying in the direction of Santa.
//
// While experimenting further with the tractor beam, you accidentally pull an asteroid directly into your ship! It deals significant damage to your hull and causes your ship to begin tumbling violently.
//
// You can send a droid out to investigate, but the tumbling is causing enough artificial gravity that one wrong step could send the droid through a hole in the hull and flying out into space.
//
// The clear choice for this mission is a droid that can jump over the holes in the hull - a springdroid.
//
// You can use an Intcode program (your puzzle input) running on an ASCII-capable computer to program the springdroid. However, springdroids don't run Intcode; instead, they run a simplified assembly language called springscript.
//
// While a springdroid is certainly capable of navigating the artificial gravity and giant holes, it has one downside: it can only remember at most 15 springscript instructions.
//
// The springdroid will move forward automatically, constantly thinking about whether to jump. The springscript program defines the logic for this decision.
//
// Springscript programs only use Boolean values, not numbers or strings. Two registers are available: T, the temporary value register, and J, the jump register. If the jump register is true at the end of the springscript program, the springdroid will try to jump. Both of these registers start with the value false.
//
// Springdroids have a sensor that can detect whether there is ground at various distances in the direction it is facing; these values are provided in read-only registers. Your springdroid can detect ground at four distances: one tile away (A), two tiles away (B), three tiles away (C), and four tiles away (D). If there is ground at the given distance, the register will be true; if there is a hole, the register will be false.
//
// There are only three instructions available in springscript:
//
//     AND X Y sets Y to true if both X and Y are true; otherwise, it sets Y to false.
//     OR X Y sets Y to true if at least one of X or Y is true; otherwise, it sets Y to false.
//     NOT X Y sets Y to true if X is false; otherwise, it sets Y to false.
//
// In all three instructions, the second argument (Y) needs to be a writable register (either T or J). The first argument (X) can be any register (including A, B, C, or D).
//
// For example, the one-instruction program NOT A J means "if the tile immediately in front of me is not ground, jump".
//
// Or, here is a program that jumps if a three-tile-wide hole (with ground on the other side of the hole) is detected:
//
// NOT A J
// NOT B T
// AND T J
// NOT C T
// AND T J
// AND D J
//
// The Intcode program expects ASCII inputs and outputs. It will begin by displaying a prompt; then, input the desired instructions one per line. End each line with a newline (ASCII code 10). When you have finished entering your program, provide the command WALK followed by a newline to instruct the springdroid to begin surveying the hull.
//
// If the springdroid falls into space, an ASCII rendering of the last moments of its life will be produced. In these, @ is the springdroid, # is hull, and . is empty space. For example, suppose you program the springdroid like this:
//
// NOT D J
// WALK
//
// This one-instruction program sets J to true if and only if there is no ground four tiles away. In other words, it attempts to jump into any hole it finds:
//
// .................
// .................
// @................
// #####.###########
//
// .................
// .................
// .@...............
// #####.###########
//
// .................
// ..@..............
// .................
// #####.###########
//
// ...@.............
// .................
// .................
// #####.###########
//
// .................
// ....@............
// .................
// #####.###########
//
// .................
// .................
// .....@...........
// #####.###########
//
// .................
// .................
// .................
// #####@###########
//
// However, if the springdroid successfully makes it across, it will use an output instruction to indicate the amount of damage to the hull as a single giant integer outside the normal ASCII range.
//
// Program the springdroid with logic that allows it to survey the hull without falling into space. What amount of hull damage does it report?
//
// Your puzzle answer was 19357390.
// --- Part Two ---
//
// There are many areas the springdroid can't reach. You flip through the manual and discover a way to increase its sensor range.
//
// Instead of ending your springcode program with WALK, use RUN. Doing this will enable extended sensor mode, capable of sensing ground up to nine tiles away. This data is available in five new read-only registers:
//
//     Register E indicates whether there is ground five tiles away.
//     Register F indicates whether there is ground six tiles away.
//     Register G indicates whether there is ground seven tiles away.
//     Register H indicates whether there is ground eight tiles away.
//     Register I indicates whether there is ground nine tiles away.
//
// All other functions remain the same.
//
// Successfully survey the rest of the hull by ending your program with RUN. What amount of hull damage does the springdroid now report?
//
// Your puzzle answer was 1142844041.

use std::convert::TryFrom;
use std::fs;

use anyhow::Context;

use crate::prog::Prog;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2019/day21")?;
    let nums: Vec<i64> =
        input.trim()
            .split(',')
            .map(|s| s.parse()
                .with_context(|| format!("Failed to parse {}", s)))
            .collect::<Result<_>>()?;
    exec(&nums, part1_soln1())?;
    exec(&nums, part2_soln())?;
    Ok(())
}

fn exec(nums: &Vec<i64>, inp: Vec<&str>) -> Result<()> {
    let mut prog = Prog::new(nums.clone());
    for s in inp {
        prog.send_str(s);
    }
    prog.resume()?;

    for i in prog.recv_iter() {
        match u8::try_from(i) {
            Ok(c) => print!("{}", c as char),
            Err(_) => print!("{}", i.to_string()),
        }
    }
    println!();
    Ok(())
}

fn part1_soln1() -> Vec<&'static str> {
    vec!["NOT A J\n",
         "NOT B T\n",
         "OR T J\n", // j = !a | !b

         "NOT C T\n",
         "OR T J\n", // j = ((!a | !b) | !c)

         "AND D J\n", // j = ((!a | !b) | !c) & d

         "WALK\n"]
}

#[allow(dead_code)]
fn part1_soln2() -> Vec<&'static str> {
    vec!["NOT A J\n", // j = !a

         "NOT C T\n",
         "AND D T\n", // t = !c & d
         "OR T J\n", // j = !a | (!c & d)

         "NOT B T\n",
         "AND D T\n", // t = !b & d
         "OR T J\n", // j = !a | (!c & d) | (!b & d)

         "WALK\n"]
}

fn part2_soln() -> Vec<&'static str> {
    vec!["NOT A J\n",
         "NOT B T\n",
         "OR T J\n", // j = !a | !b

         "NOT C T\n",
         "OR T J\n", // j = ((!a | !b) | !c)

         "AND D J\n", // j = ((!a | !b) | !c) & d

         "NOT E T\n",
         "NOT T T\n", // t = e
         "OR H T\n", // t = (e | h)
         "AND T J\n", // j = j & (e | h)

         "RUN\n"]
}

mod prog {
    use std::collections::vec_deque::Drain;
    use std::collections::VecDeque;
    use std::convert::TryFrom;
    use std::iter;

    use super::Result;

    #[derive(Debug)]
    pub enum Status {
        Blocked,
        Stopped,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub struct Prog {
        nums: Vec<i64>,
        i: usize,
        relbase: i64,
        input: VecDeque<i64>,
        output: VecDeque<i64>,
    }

    impl Prog {
        pub fn new(nums: Vec<i64>) -> Prog {
            Prog {
                nums,
                i: 0,
                relbase: 0,
                input: VecDeque::new(),
                output: VecDeque::new(),
            }
        }

        #[allow(dead_code)]
        pub fn send_str(&mut self, s: &str) {
            for i in s.bytes().map(|c| c as i64) {
                self.send(i);
            }
        }

        #[allow(dead_code)]
        pub fn send_and_resume(&mut self, i: i64) -> Result<Status>
        {
            self.send(i);
            self.resume()
        }

        pub fn send(&mut self, i: i64) {
            self.input.push_back(i);
        }

        #[allow(dead_code)]
        pub fn recv_iter(&mut self) -> Drain<i64> {
            self.output.drain(..)
        }

        #[allow(dead_code)]
        pub fn recv_unwrap(&mut self) -> i64 {
            self.output.pop_front().unwrap()
        }

        #[allow(dead_code)]
        pub fn is_empty_output(&self) -> bool {
            !self.output.is_empty()
        }

        #[allow(dead_code)]
        pub fn len_output(&self) -> usize {
            self.output.len()
        }

        pub fn resume(&mut self) -> Result<Status> {
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
                        self.nums[pos] = match self.input.pop_front() {
                            None => return Ok(Status::Blocked),
                            Some(m) => m,
                        };
                        self.i += 2;
                    }
                    4 => {
                        let num = self.read_val(modes.next().unwrap(), self.i + 1)?;
                        self.output.push_back(num);
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
            Ok(Status::Stopped)
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
