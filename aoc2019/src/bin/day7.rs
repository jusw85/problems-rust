// https://adventofcode.com/2019/day/7
//
// --- Day 7: Amplification Circuit ---
//
// Based on the navigational maps, you're going to need to send more power to your ship's thrusters to reach Santa in time. To do this, you'll need to configure a series of amplifiers already installed on the ship.
//
// There are five amplifiers connected in series; each one receives an input signal and produces an output signal. They are connected such that the first amplifier's output leads to the second amplifier's input, the second amplifier's output leads to the third amplifier's input, and so on. The first amplifier's input value is 0, and the last amplifier's output leads to your ship's thrusters.
//
//     O-------O  O-------O  O-------O  O-------O  O-------O
// 0 ->| Amp A |->| Amp B |->| Amp C |->| Amp D |->| Amp E |-> (to thrusters)
//     O-------O  O-------O  O-------O  O-------O  O-------O
//
// The Elves have sent you some Amplifier Controller Software (your puzzle input), a program that should run on your existing Intcode computer. Each amplifier will need to run a copy of the program.
//
// When a copy of the program starts running on an amplifier, it will first use an input instruction to ask the amplifier for its current phase setting (an integer from 0 to 4). Each phase setting is used exactly once, but the Elves can't remember which amplifier needs which phase setting.
//
// The program will then call another input instruction to get the amplifier's input signal, compute the correct output signal, and supply it back to the amplifier with an output instruction. (If the amplifier has not yet received an input signal, it waits until one arrives.)
//
// Your job is to find the largest output signal that can be sent to the thrusters by trying every possible combination of phase settings on the amplifiers. Make sure that memory is not shared or reused between copies of the program.
//
// For example, suppose you want to try the phase setting sequence 3,1,2,4,0, which would mean setting amplifier A to phase setting 3, amplifier B to setting 1, C to 2, D to 4, and E to 0. Then, you could determine the output signal that gets sent from amplifier E to the thrusters with the following steps:
//
//     Start the copy of the amplifier controller software that will run on amplifier A. At its first input instruction, provide it the amplifier's phase setting, 3. At its second input instruction, provide it the input signal, 0. After some calculations, it will use an output instruction to indicate the amplifier's output signal.
//     Start the software for amplifier B. Provide it the phase setting (1) and then whatever output signal was produced from amplifier A. It will then produce a new output signal destined for amplifier C.
//     Start the software for amplifier C, provide the phase setting (2) and the value from amplifier B, then collect its output signal.
//     Run amplifier D's software, provide the phase setting (4) and input value, and collect its output signal.
//     Run amplifier E's software, provide the phase setting (0) and input value, and collect its output signal.
//
// The final output signal from amplifier E would be sent to the thrusters. However, this phase setting sequence may not have been the best one; another sequence might have sent a higher signal to the thrusters.
//
// Here are some example programs:
//
//     Max thruster signal 43210 (from phase setting sequence 4,3,2,1,0):
//
//     3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0
//
//     Max thruster signal 54321 (from phase setting sequence 0,1,2,3,4):
//
//     3,23,3,24,1002,24,10,24,1002,23,-1,23,
//     101,5,23,23,1,24,23,23,4,23,99,0,0
//
//     Max thruster signal 65210 (from phase setting sequence 1,0,4,3,2):
//
//     3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
//     1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0
//
// Try every combination of phase settings on the amplifiers. What is the highest signal that can be sent to the thrusters?
//
// Your puzzle answer was 437860.
// --- Part Two ---
//
// It's no good - in this configuration, the amplifiers can't generate a large enough output signal to produce the thrust you'll need. The Elves quickly talk you through rewiring the amplifiers into a feedback loop:
//
//       O-------O  O-------O  O-------O  O-------O  O-------O
// 0 -+->| Amp A |->| Amp B |->| Amp C |->| Amp D |->| Amp E |-.
//    |  O-------O  O-------O  O-------O  O-------O  O-------O |
//    |                                                        |
//    '--------------------------------------------------------+
//                                                             |
//                                                             v
//                                                      (to thrusters)
//
// Most of the amplifiers are connected as they were before; amplifier A's output is connected to amplifier B's input, and so on. However, the output from amplifier E is now connected into amplifier A's input. This creates the feedback loop: the signal will be sent through the amplifiers many times.
//
// In feedback loop mode, the amplifiers need totally different phase settings: integers from 5 to 9, again each used exactly once. These settings will cause the Amplifier Controller Software to repeatedly take input and produce output many times before halting. Provide each amplifier its phase setting at its first input instruction; all further input/output instructions are for signals.
//
// Don't restart the Amplifier Controller Software on any amplifier during this process. Each one should continue receiving and sending signals until it halts.
//
// All signals sent or received in this process will be between pairs of amplifiers except the very first signal and the very last signal. To start the process, a 0 signal is sent to amplifier A's input exactly once.
//
// Eventually, the software on the amplifiers will halt after they have processed the final loop. When this happens, the last output signal from amplifier E is sent to the thrusters. Your job is to find the largest output signal that can be sent to the thrusters using the new phase settings and feedback loop arrangement.
//
// Here are some example programs:
//
//     Max thruster signal 139629729 (from phase setting sequence 9,8,7,6,5):
//
//     3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
//     27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5
//
//     Max thruster signal 18216 (from phase setting sequence 9,7,8,5,6):
//
//     3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
//     -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
//     53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10
//
// Try every combination of the new phase settings on the amplifier feedback loop. What is the highest signal that can be sent to the thrusters?
//
// Your puzzle answer was 49810599.

use std::{fs, thread};
use std::sync::mpsc;

use anyhow::Context;

use crate::prog::Prog;

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
    part2(&nums)?;
    Ok(())
}

fn part1(nums: &Vec<i32>) -> Result<()> {
    let phases = [0, 1, 2, 3, 4];
    let (signal, phase) = max_amplified_signal(&nums, &phases, false)?;
    println!("{:?}", (signal, phase));
    Ok(())
}

fn part2(nums: &Vec<i32>) -> Result<()> {
    let phases = [5, 6, 7, 8, 9];
    let (signal, phase) = max_amplified_signal(&nums, &phases, true)?;
    println!("{:?}", (signal, phase));
    Ok(())
}

fn max_amplified_signal(nums: &[i32], phases: &[i32], is_loop: bool) -> Result<(i32, Vec<i32>)> {
    let phases = phases.to_vec();
    let mut max_phases: Vec<i32> = Vec::new();
    let mut max_signal = i32::MIN;

    let perms = Permutation::new(phases);
    for phases_perm in perms {
        let signal = amplify_signal(nums, &phases_perm, is_loop)?;
        if signal > max_signal {
            max_signal = signal;
            max_phases = phases_perm;
        }
    }
    Ok((max_signal, max_phases))
}

fn amplify_signal(nums: &[i32], phases: &[i32], is_loop: bool) -> Result<i32> {
    let mut channels = Vec::new();
    let mut input_clones = Vec::new();

    let (input, from_input) = mpsc::sync_channel(0);
    input_clones.push(input.clone());

    let mut next_recv = from_input;
    for _ in 0..(phases.len() - 1) {
        let (send, recv) = mpsc::sync_channel(0);
        input_clones.push(send.clone());
        channels.push((next_recv, [send].to_vec()));
        next_recv = recv;
    }

    let (output, from_output) = mpsc::sync_channel(0);
    let last_outputs = if is_loop { [input, output].to_vec() } else { [output].to_vec() };
    channels.push((next_recv, last_outputs));

    let mut threads = Vec::new();
    for (recv, sends) in channels {
        let nums = nums.to_vec();
        threads.push(thread::spawn(move || -> Result<()> {
            Prog::new(nums, recv, sends).exec()?;
            Ok(())
        }));
    }

    for (input, phase) in input_clones.iter().zip(phases) {
        input.send(*phase)?;
    }
    input_clones[0].send(0)?;
    let result = from_output.recv()?;

    for thread in threads {
        thread.join().unwrap()?;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() -> Result<()> {
        let phases = [4, 3, 2, 1, 0];

        let nums = [3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0];
        let signal = amplify_signal(&nums, &phases, false)?;
        assert_eq!(43210, signal);

        Ok(())
    }

    #[test]
    fn test() -> Result<()> {
        let phases = [0, 1, 2, 3, 4];

        let nums = [3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0];
        let (signal, phase) = max_amplified_signal(&nums, &phases, false)?;
        assert_eq!(43210, signal);
        assert_eq!([4, 3, 2, 1, 0], &phase[..]);

        let nums = [3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23,
            101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0];
        let (signal, phase) = max_amplified_signal(&nums, &phases, false)?;
        assert_eq!(54321, signal);
        assert_eq!([0, 1, 2, 3, 4], &phase[..]);

        let nums = [3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33,
            1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0];
        let (signal, phase) = max_amplified_signal(&nums, &phases, false)?;
        assert_eq!(65210, signal);
        assert_eq!([1, 0, 4, 3, 2], &phase[..]);

        Ok(())
    }

    #[test]
    fn test_loop() -> Result<()> {
        let phases = [9, 8, 7, 6, 5];

        let nums = [3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26,
            27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5];
        let (signal, phase) = max_amplified_signal(&nums, &phases, true)?;
        assert_eq!(139629729, signal);
        assert_eq!([9, 8, 7, 6, 5], &phase[..]);

        let phases = [5, 8, 7, 6, 9];
        let nums = [3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10];
        let (signal, phase) = max_amplified_signal(&nums, &phases, true)?;
        assert_eq!(18216, signal);
        assert_eq!([9, 7, 8, 5, 6], &phase[..]);

        Ok(())
    }

    #[test]
    fn test_perm() {
        let a = [1, 2, 3].to_vec();
        let mut x = Permutation::new(a);
        assert_eq!(x.next(), Some(vec![1, 2, 3]));
        assert_eq!(x.next(), Some(vec![2, 1, 3]));
        assert_eq!(x.next(), Some(vec![3, 1, 2]));
        assert_eq!(x.next(), Some(vec![1, 3, 2]));
        assert_eq!(x.next(), Some(vec![2, 3, 1]));
        assert_eq!(x.next(), Some(vec![3, 2, 1]));
        assert_eq!(x.next(), None);
    }
}

struct Permutation<T> {
    i: i32,
    c: Vec<usize>,
    a: Vec<T>,
}

impl<T: Clone> Permutation<T> {
    fn new(a: Vec<T>) -> Permutation<T> {
        let c = vec![0; a.len()];
        Permutation {
            i: -1,
            c,
            a,
        }
    }
}

impl<T: Clone> Iterator for Permutation<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        let Permutation { i: ip, c, a } = self;
        if *ip < 0 {
            *ip = 0;
            return Some(a.clone());
        }
        let mut i = *ip as usize;
        while i < c.len() {
            if c[i] < i {
                if i % 2 == 0 {
                    a.swap(0, i);
                } else {
                    a.swap(c[i], i);
                }
                c[i] += 1;
                *ip = 0;
                return Some(a.clone());
            } else {
                c[i] = 0;
                i += 1;
            }
        }
        None
    }
}

mod prog {
    use std::convert::TryFrom;
    use std::iter;
    use std::sync::mpsc::{Receiver, SyncSender};

    use super::Result;

    pub struct Prog {
        nums: Vec<i32>,
        i: usize,
        input: Receiver<i32>,
        outputs: Vec<SyncSender<i32>>,
    }

    impl Prog {
        pub fn new(nums: Vec<i32>,
                   input: Receiver<i32>,
                   outputs: Vec<SyncSender<i32>>) -> Prog {
            Prog {
                nums,
                i: 0,
                input,
                outputs,
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
                    _ => anyhow::bail!("Unrecognized opcode: {}", op),
                };
            }
            Ok(())
        }

        fn read_val(&self, mode: u32, idx: usize) -> Result<i32> {
            match mode {
                0 => {
                    let idx = usize::try_from(self.nums[idx])?;
                    Ok(self.nums[idx])
                }
                1 => Ok(self.nums[idx]),
                _ => anyhow::bail!("Unrecognized parameter mode: {}", mode),
            }
        }

        fn read_pos(&self, mode: u32, idx: usize) -> Result<usize> {
            match mode {
                0 => Ok(usize::try_from(self.nums[idx])?),
                _ => anyhow::bail!("Invalid parameter mode for pos: {}", mode),
            }
        }
    }
}
