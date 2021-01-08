// https://adventofcode.com/2020/day/8
//
// --- Day 8: Handheld Halting ---
//
// Your flight to the major airline hub reaches cruising altitude without incident. While you consider checking the in-flight menu for one of those drinks that come with a little umbrella, you are interrupted by the kid sitting next to you.
//
// Their handheld game console won't turn on! They ask if you can take a look.
//
// You narrow the problem down to a strange infinite loop in the boot code (your puzzle input) of the device. You should be able to fix it, but first you need to be able to run the code in isolation.
//
// The boot code is represented as a text file with one instruction per line of text. Each instruction consists of an operation (acc, jmp, or nop) and an argument (a signed number like +4 or -20).
//
//     acc increases or decreases a single global value called the accumulator by the value given in the argument. For example, acc +7 would increase the accumulator by 7. The accumulator starts at 0. After an acc instruction, the instruction immediately below it is executed next.
//     jmp jumps to a new instruction relative to itself. The next instruction to execute is found using the argument as an offset from the jmp instruction; for example, jmp +2 would skip the next instruction, jmp +1 would continue to the instruction immediately below it, and jmp -20 would cause the instruction 20 lines above to be executed next.
//     nop stands for No OPeration - it does nothing. The instruction immediately below it is executed next.
//
// For example, consider the following program:
//
// nop +0
// acc +1
// jmp +4
// acc +3
// jmp -3
// acc -99
// acc +1
// jmp -4
// acc +6
//
// These instructions are visited in this order:
//
// nop +0  | 1
// acc +1  | 2, 8(!)
// jmp +4  | 3
// acc +3  | 6
// jmp -3  | 7
// acc -99 |
// acc +1  | 4
// jmp -4  | 5
// acc +6  |
//
// First, the nop +0 does nothing. Then, the accumulator is increased from 0 to 1 (acc +1) and jmp +4 sets the next instruction to the other acc +1 near the bottom. After it increases the accumulator from 1 to 2, jmp -4 executes, setting the next instruction to the only acc +3. It sets the accumulator to 5, and jmp -3 causes the program to continue back at the first acc +1.
//
// This is an infinite loop: with this sequence of jumps, the program will run forever. The moment the program tries to run any instruction a second time, you know it will never terminate.
//
// Immediately before the program would run an instruction a second time, the value in the accumulator is 5.
//
// Run your copy of the boot code. Immediately before any instruction is executed a second time, what value is in the accumulator?
//
// Your puzzle answer was 1727.
// --- Part Two ---
//
// After some careful analysis, you believe that exactly one instruction is corrupted.
//
// Somewhere in the program, either a jmp is supposed to be a nop, or a nop is supposed to be a jmp. (No acc instructions were harmed in the corruption of this boot code.)
//
// The program is supposed to terminate by attempting to execute an instruction immediately after the last instruction in the file. By changing exactly one jmp or nop, you can repair the boot code and make it terminate correctly.
//
// For example, consider the same program from above:
//
// nop +0
// acc +1
// jmp +4
// acc +3
// jmp -3
// acc -99
// acc +1
// jmp -4
// acc +6
//
// If you change the first instruction from nop +0 to jmp +0, it would create a single-instruction infinite loop, never leaving that instruction. If you change almost any of the jmp instructions, the program will still eventually find another jmp instruction and loop forever.
//
// However, if you change the second-to-last instruction (from jmp -4 to nop -4), the program terminates! The instructions are visited in this order:
//
// nop +0  | 1
// acc +1  | 2
// jmp +4  | 3
// acc +3  |
// jmp -3  |
// acc -99 |
// acc +1  | 4
// nop -4  | 5
// acc +6  | 6
//
// After the last instruction (acc +6), the program terminates by attempting to run the instruction below the last instruction in the file. With this change, after the program terminates, the accumulator contains the value 8 (acc +1, acc +1, acc +6).
//
// Fix the program so that it terminates normally by changing exactly one jmp (to nop) or nop (to jmp). What is the value of the accumulator after the program terminates?
//
// Your puzzle answer was 552.

use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2020/day8")?;
    let mut prog = parse(&input)?;
    let state = prog.exec();
    println!("{:?}", state);
    println!("{}", prog.acc);

    prog.acc = 0;
    println!("{:?}", find_broken(&prog));
    Ok(())
}

fn find_broken(prog: &Prog) -> Option<isize> {
    for (i, _) in prog.ops.iter()
        .enumerate()
        .filter(|(_, &op)| op.op != OpType::Acc)
    {
        let mut prog = prog.clone();
        prog.ops[i].op = match prog.ops[i].op {
            OpType::Acc => unreachable!(),
            OpType::Jmp => OpType::Nop,
            OpType::Nop => OpType::Jmp,
        };
        let state = prog.exec();
        if state == State::Stopped {
            return Some(prog.acc);
        }
    }
    None
}

fn parse(s: &str) -> Result<Prog> {
    let ops: Vec<_> = s.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Op>())
        .try_collect()?;
    Ok(Prog::new(ops))
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Prog {
    ops: Vec<Op>,
    acc: isize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum State {
    Looping,
    Stopped,
}

impl Prog {
    fn new(ops: Vec<Op>) -> Prog {
        Prog { ops, acc: 0 }
    }

    fn exec(&mut self) -> State {
        let mut visited = HashSet::new();
        let mut idx = 0_isize;
        loop {
            if idx < 0 || idx >= self.ops.len() as isize {
                break State::Stopped;
            }
            if visited.contains(&idx) {
                break State::Looping;
            }
            visited.insert(idx);
            let Op { op, arg } = self.ops[idx as usize];
            match op {
                OpType::Acc => {
                    self.acc += arg;
                    idx += 1;
                }
                OpType::Jmp => idx += arg,
                OpType::Nop => idx += 1,
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Op {
    op: OpType,
    arg: isize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum OpType {
    Acc,
    Jmp,
    Nop,
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r"^(acc|nop|jmp) ((?:-|\+)\d+)$").unwrap();
        }
        if !RE.is_match(s) {
            anyhow::bail!("cannot parse op: {}", s);
        }
        let cap = RE.captures(s).unwrap();
        let op = match &cap[1] {
            "acc" => OpType::Acc,
            "jmp" => OpType::Jmp,
            "nop" => OpType::Nop,
            _ => unreachable!(),
        };
        let arg = cap[2].parse::<isize>().unwrap();
        Ok(Op { op, arg })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6
        ";
        let mut prog = parse(s)?;
        assert_eq!(Some(8), find_broken(&prog));

        prog.exec();
        assert_eq!(5, prog.acc);
        Ok(())
    }
}
