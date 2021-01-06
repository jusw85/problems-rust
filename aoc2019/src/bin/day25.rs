// https://adventofcode.com/2019/day/25
//
// --- Day 25: Cryostasis ---
//
// As you approach Santa's ship, your sensors report two important details:
//
// First, that you might be too late: the internal temperature is -40 degrees.
//
// Second, that one faint life signature is somewhere on the ship.
//
// The airlock door is locked with a code; your best option is to send in a small droid to investigate the situation. You attach your ship to Santa's, break a small hole in the hull, and let the droid run in before you seal it up again. Before your ship starts freezing, you detach your ship and set it to automatically stay within range of Santa's ship.
//
// This droid can follow basic instructions and report on its surroundings; you can communicate with it through an Intcode program (your puzzle input) running on an ASCII-capable computer.
//
// As the droid moves through its environment, it will describe what it encounters. When it says Command?, you can give it a single instruction terminated with a newline (ASCII code 10). Possible instructions are:
//
//     Movement via north, south, east, or west.
//     To take an item the droid sees in the environment, use the command take <name of item>. For example, if the droid reports seeing a red ball, you can pick it up with take red ball.
//     To drop an item the droid is carrying, use the command drop <name of item>. For example, if the droid is carrying a green ball, you can drop it with drop green ball.
//     To get a list of all of the items the droid is currently carrying, use the command inv (for "inventory").
//
// Extra spaces or other characters aren't allowed - instructions must be provided precisely.
//
// Santa's ship is a Reindeer-class starship; these ships use pressure-sensitive floors to determine the identity of droids and crew members. The standard configuration for these starships is for all droids to weigh exactly the same amount to make them easier to detect. If you need to get past such a sensor, you might be able to reach the correct weight by carrying items from the environment.
//
// Look around the ship and see if you can find the password for the main airlock.
//
// Your puzzle answer was 136839232.
// --- Part Two ---
//
// As you move through the main airlock, the air inside the ship is already heating up to reasonable levels. Santa explains that he didn't notice you coming because he was just taking a quick nap. The ship wasn't frozen; he just had the thermostat set to "North Pole".
//
// You make your way over to the navigation console. It beeps. "Status: Stranded. Please supply measurements from 49 stars to recalibrate."
//
// "49 stars? But the Elves told me you needed fifty--"
//
// Santa just smiles and nods his head toward the window. There, in the distance, you can see the center of the Solar System: the Sun!
//
// The navigation console beeps again.
//
// If you like, you can
// .

use std::collections::{HashMap, VecDeque};
use std::fs;

use anyhow::Context;

use crate::prog::{Prog, Status};

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2019/day25")?;
    let nums: Vec<i64> =
        input.trim()
            .split(',')
            .map(|s| s.parse()
                .with_context(|| format!("Failed to parse {}", s)))
            .collect::<Result<_>>()?;
    exec(&nums)?;
    Ok(())
}

fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    s.trim().to_string()
}

const AUTO: &str = "s,take fixed point,n,\
                    w,w,w,take hologram,e,e,e,\
                    n,take candy cane,\
                    w,take antenna,\
                    s,take whirled peas,n,\
                    w,take shell,e,e,\
                    n,n,take polygon,\
                    s,w,take fuel cell,w";

fn exec(nums: &Vec<i64>) -> Result<()> {
    let mut items = HashMap::new();
    items.insert(1, "fixed point"); // +
    items.insert(2, "hologram"); // -
    items.insert(3, "candy cane"); // +
    items.insert(4, "antenna"); // -
    items.insert(5, "whirled peas"); // -
    items.insert(6, "shell"); // +
    items.insert(7, "polygon"); // +
    items.insert(8, "fuel cell"); // -

    let mut prog = Prog::new(nums.clone());
    let mut queued_commands = VecDeque::new();
    loop {
        let state = prog.resume()?;
        println!("{}", prog.recv_string());
        if state == Status::Stopped {
            break;
        }

        if queued_commands.is_empty() {
            let s = read_stdin();
            match &*s {
                "auto" => {
                    queued_commands.extend(
                        AUTO.split(",").map(|s| s.to_string()));
                }
                _ => queued_commands.push_front(s),
            }
        }

        let mut s = queued_commands.pop_front().unwrap();
        let mut sp = s.trim();
        sp = match sp {
            "n" => "north",
            "s" => "south",
            "e" => "east",
            "w" => "west",
            "i" => "inv",
            "quit" => { break; }
            _ => sp,
        };
        s = sp.to_string();

        let num = &s[s.len() - 1..].parse::<i32>();
        if let Ok(d) = num {
            let obj = items[&d];
            let com = match &s[..1] {
                "d" => Some("drop"),
                "g" => Some("take"),
                _ => None,
            };
            if let Some(c) = com {
                s = format!("{} {}", c, obj);
            }
        }
        println!("{}", s);
        prog.send_str(&s);
        prog.send_str("\n");
    }
    Ok(())
}

mod prog {
    use std::collections::vec_deque::Drain;
    use std::collections::VecDeque;
    use std::convert::TryFrom;
    use std::fmt::Write;
    use std::iter;

    use super::Result;

    #[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
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
        pub fn recv_string(&mut self) -> String {
            let mut s = String::new();
            for i in self.recv_iter() {
                match u8::try_from(i) {
                    Ok(c) => write!(&mut s, "{}", c as char).unwrap(),
                    Err(_) => write!(&mut s, "{}", i.to_string()).unwrap(),
                };
            }
            s
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
        pub fn is_empty_input(&self) -> bool {
            !self.input.is_empty()
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
