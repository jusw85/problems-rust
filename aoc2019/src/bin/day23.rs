// https://adventofcode.com/2019/day/23
//
// --- Day 23: Category Six ---
//
// The droids have finished repairing as much of the ship as they can. Their report indicates that this was a Category 6 disaster - not because it was that bad, but because it destroyed the stockpile of Category 6 network cables as well as most of the ship's network infrastructure.
//
// You'll need to rebuild the network from scratch.
//
// The computers on the network are standard Intcode computers that communicate by sending packets to each other. There are 50 of them in total, each running a copy of the same Network Interface Controller (NIC) software (your puzzle input). The computers have network addresses 0 through 49; when each computer boots up, it will request its network address via a single input instruction. Be sure to give each computer a unique network address.
//
// Once a computer has received its network address, it will begin doing work and communicating over the network by sending and receiving packets. All packets contain two values named X and Y. Packets sent to a computer are queued by the recipient and read in the order they are received.
//
// To send a packet to another computer, the NIC will use three output instructions that provide the destination address of the packet followed by its X and Y values. For example, three output instructions that provide the values 10, 20, 30 would send a packet with X=20 and Y=30 to the computer with address 10.
//
// To receive a packet from another computer, the NIC will use an input instruction. If the incoming packet queue is empty, provide -1. Otherwise, provide the X value of the next packet; the computer will then use a second input instruction to receive the Y value for the same packet. Once both values of the packet are read in this way, the packet is removed from the queue.
//
// Note that these input and output instructions never block. Specifically, output instructions do not wait for the sent packet to be received - the computer might send multiple packets before receiving any. Similarly, input instructions do not wait for a packet to arrive - if no packet is waiting, input instructions should receive -1.
//
// Boot up all 50 computers and attach them to your network. What is the Y value of the first packet sent to address 255?
//
// Your puzzle answer was 24268.
// --- Part Two ---
//
// Packets sent to address 255 are handled by a device called a NAT (Not Always Transmitting). The NAT is responsible for managing power consumption of the network by blocking certain packets and watching for idle periods in the computers.
//
// If a packet would be sent to address 255, the NAT receives it instead. The NAT remembers only the last packet it receives; that is, the data in each packet it receives overwrites the NAT's packet memory with the new packet's X and Y values.
//
// The NAT also monitors all computers on the network. If all computers have empty incoming packet queues and are continuously trying to receive packets without sending packets, the network is considered idle.
//
// Once the network is idle, the NAT sends only the last packet it received to address 0; this will cause the computers on the network to resume activity. In this way, the NAT can throttle power consumption of the network when the ship needs power in other areas.
//
// Monitor packets released to the computer at address 0 by the NAT. What is the first Y value delivered by the NAT to the computer at address 0 twice in a row?
//
// Your puzzle answer was 19316.


use std::fs;

use anyhow::Context;

use crate::prog::Prog;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2019/day23")?;
    let nums: Vec<i64> =
        input.trim()
            .split(',')
            .map(|s| s.parse()
                .with_context(|| format!("Failed to parse {}", s)))
            .collect::<Result<_>>()?;
    exec(&nums)?;
    Ok(())
}

fn exec(nums: &Vec<i64>) -> Result<()> {
    let mut progs = Vec::new();
    for i in 0..50 {
        let mut prog = Prog::new(nums.clone());
        prog.send(i);
        progs.push(prog);
    }

    let mut nat = Vec::new();
    let mut first_y = None;
    let mut previous_wakeup_y = None;

    loop {
        let mut idle = true;
        for i in 0..50 {
            let prog = &mut progs[i];

            if prog.is_empty_input() {
                prog.send(-1);
            }
            prog.resume()?;

            let outputs = prog.recv_iter().collect::<Vec<_>>();
            assert_eq!(outputs.len() % 3, 0);
            for output in outputs.chunks_exact(3) {
                let (id, x, y) = (output[0] as usize, output[1], output[2]);
                if id == 255 {
                    if let None = first_y {
                        first_y = Some(y);
                    }
                    nat.push((x, y));
                    continue;
                }
                let to_prog = &mut progs[id];
                to_prog.send(x);
                to_prog.send(y);
                idle = false;
            }
        }
        if idle {
            let &(x, y) = nat.last().unwrap();
            if let Some(py) = previous_wakeup_y {
                if py == y {
                    break;
                }
            }
            let prog = &mut progs[0];
            prog.send(x);
            prog.send(y);
            previous_wakeup_y = Some(y);
        }
    };
    println!("{:?}", first_y);
    println!("{:?}", previous_wakeup_y);
    Ok(())
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
