// https://adventofcode.com/2020/day/14
//
// --- Day 14: Docking Data ---
//
// As your ferry approaches the sea port, the captain asks for your help again. The computer system that runs this port isn't compatible with the docking program on the ferry, so the docking parameters aren't being correctly initialized in the docking program's memory.
//
// After a brief inspection, you discover that the sea port's computer system uses a strange bitmask system in its initialization program. Although you don't have the correct decoder chip handy, you can emulate it in software!
//
// The initialization program (your puzzle input) can either update the bitmask or write a value to memory. Values and memory addresses are both 36-bit unsigned integers. For example, ignoring bitmasks for a moment, a line like mem[8] = 11 would write the value 11 to memory address 8.
//
// The bitmask is always given as a string of 36 bits, written with the most significant bit (representing 2^35) on the left and the least significant bit (2^0, that is, the 1s bit) on the right. The current bitmask is applied to values immediately before they are written to memory: a 0 or 1 overwrites the corresponding bit in the value, while an X leaves the bit in the value unchanged.
//
// For example, consider the following program:
//
// mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
// mem[8] = 11
// mem[7] = 101
// mem[8] = 0
//
// This program starts by specifying a bitmask (mask = ....). The mask it specifies will overwrite two bits in every written value: the 2s bit is overwritten with 0, and the 64s bit is overwritten with 1.
//
// The program then attempts to write the value 11 to memory address 8. By expanding everything out to individual bits, the mask is applied as follows:
//
// value:  000000000000000000000000000000001011  (decimal 11)
// mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
// result: 000000000000000000000000000001001001  (decimal 73)
//
// So, because of the mask, the value 73 is written to memory address 8 instead. Then, the program tries to write 101 to address 7:
//
// value:  000000000000000000000000000001100101  (decimal 101)
// mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
// result: 000000000000000000000000000001100101  (decimal 101)
//
// This time, the mask has no effect, as the bits it overwrote were already the values the mask tried to set. Finally, the program tries to write 0 to address 8:
//
// value:  000000000000000000000000000000000000  (decimal 0)
// mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
// result: 000000000000000000000000000001000000  (decimal 64)
//
// 64 is written to address 8 instead, overwriting the value that was there previously.
//
// To initialize your ferry's docking program, you need the sum of all values left in memory after the initialization program completes. (The entire 36-bit address space begins initialized to the value 0 at every address.) In the above example, only two values in memory are not zero - 101 (at address 7) and 64 (at address 8) - producing a sum of 165.
//
// Execute the initialization program. What is the sum of all values left in memory after it completes? (Do not truncate the sum to 36 bits.)
//
// Your puzzle answer was 6559449933360.
// --- Part Two ---
//
// For some reason, the sea port's computer system still can't communicate with your ferry's docking program. It must be using version 2 of the decoder chip!
//
// A version 2 decoder chip doesn't modify the values being written at all. Instead, it acts as a memory address decoder. Immediately before a value is written to memory, each bit in the bitmask modifies the corresponding bit of the destination memory address in the following way:
//
//     If the bitmask bit is 0, the corresponding memory address bit is unchanged.
//     If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
//     If the bitmask bit is X, the corresponding memory address bit is floating.
//
// A floating bit is not connected to anything and instead fluctuates unpredictably. In practice, this means the floating bits will take on all possible values, potentially causing many memory addresses to be written all at once!
//
// For example, consider the following program:
//
// mask = 000000000000000000000000000000X1001X
// mem[42] = 100
// mask = 00000000000000000000000000000000X0XX
// mem[26] = 1
//
// When this program goes to write to memory address 42, it first applies the bitmask:
//
// address: 000000000000000000000000000000101010  (decimal 42)
// mask:    000000000000000000000000000000X1001X
// result:  000000000000000000000000000000X1101X
//
// After applying the mask, four bits are overwritten, three of which are different, and two of which are floating. Floating bits take on every possible combination of values; with two floating bits, four actual memory addresses are written:
//
// 000000000000000000000000000000011010  (decimal 26)
// 000000000000000000000000000000011011  (decimal 27)
// 000000000000000000000000000000111010  (decimal 58)
// 000000000000000000000000000000111011  (decimal 59)
//
// Next, the program is about to write to memory address 26 with a different bitmask:
//
// address: 000000000000000000000000000000011010  (decimal 26)
// mask:    00000000000000000000000000000000X0XX
// result:  00000000000000000000000000000001X0XX
//
// This results in an address with three floating bits, causing writes to eight memory addresses:
//
// 000000000000000000000000000000010000  (decimal 16)
// 000000000000000000000000000000010001  (decimal 17)
// 000000000000000000000000000000010010  (decimal 18)
// 000000000000000000000000000000010011  (decimal 19)
// 000000000000000000000000000000011000  (decimal 24)
// 000000000000000000000000000000011001  (decimal 25)
// 000000000000000000000000000000011010  (decimal 26)
// 000000000000000000000000000000011011  (decimal 27)
//
// The entire 36-bit address space still begins initialized to the value 0 at every address, and you still need the sum of all values left in memory at the end of the program. In this example, the sum is 208.
//
// Execute the initialization program using an emulator for a version 2 decoder chip. What is the sum of all values left in memory after it completes?
//
// Your puzzle answer was 3369767240513.

use std::collections::HashMap;
use std::fs;

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2020/day14")?;
    let ops = parse(&input);
    println!("{}", exec1(&ops));
    println!("{}", exec2(&ops));
    Ok(())
}

fn parse(s: &str) -> Vec<Op> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"^mem\[(\d+)\]$").unwrap();
    }
    s.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut iter = line.split("=");
            let lhs = iter.next().unwrap().trim();
            let rhs = iter.next().unwrap().trim();
            if lhs.starts_with("mask") {
                Op::Mask(rhs.to_string())
            } else {
                let rhs = rhs.parse::<i64>().unwrap();
                let lhs = &RE.captures(lhs).unwrap()[1];
                let lhs = lhs.parse::<i64>().unwrap();
                Op::Mem(lhs, rhs)
            }
        })
        .collect_vec()
}

fn apply_mask(mask: &str, val: i64) -> i64 {
    let m1 = mask.replace("X", "1");
    let m0 = mask.replace("X", "0");
    let m1 = i64::from_str_radix(&m1, 2).unwrap();
    let m0 = i64::from_str_radix(&m0, 2).unwrap();
    (m1 & val) | m0
}

fn exec1(ops: &Vec<Op>) -> i64 {
    let mut mem = HashMap::new();
    let mut mask = None;
    for op in ops {
        match op {
            Op::Mask(m) => mask = Some(m),
            Op::Mem(add, val) => {
                let res = apply_mask(mask.unwrap(), *val);
                mem.insert(*add, res);
            }
        }
    }
    mem.values()
        .filter(|&&v| v != 0)
        .sum::<i64>()
}

fn exec2(ops: &Vec<Op>) -> i64 {
    let mut mem = HashMap::new();
    let mut mask = None;
    let mut floating_bits = None;
    for op in ops {
        match op {
            Op::Mask(m) => {
                mask = Some(m);
                floating_bits = Some(m.bytes().enumerate()
                    .filter_map(|(i, c)| if c == b'X' { Some(35 - i) } else { None })
                    .collect_vec());
            }
            Op::Mem(add, val) => {
                let mask = mask.unwrap();
                let mask = mask.replace("0", "T")
                    .replace("X", "0")
                    .replace("T", "X");
                let base = apply_mask(&mask, *add);

                let floating_bits = floating_bits.as_deref().unwrap();
                for num in 0..2_usize.pow(floating_bits.len() as u32) {
                    let mut floating_mask = 0;
                    for (i, j) in floating_bits.iter().enumerate() {
                        floating_mask |= (((num >> i) & 1) << j) as i64;
                    }
                    let add = base | floating_mask;
                    mem.insert(add, *val);
                }
            }
        }
    }
    mem.values()
        .filter(|&&v| v != 0)
        .sum::<i64>()
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum Op {
    Mask(String),
    Mem(i64, i64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 0
        ";
        let ops = parse(s);
        assert_eq!(165, exec1(&ops));

        let s = r"
        mask = 000000000000000000000000000000X1001X
        mem[42] = 100
        mask = 00000000000000000000000000000000X0XX
        mem[26] = 1
        ";
        let ops = parse(s);
        assert_eq!(208, exec2(&ops));
        Ok(())
    }
}
