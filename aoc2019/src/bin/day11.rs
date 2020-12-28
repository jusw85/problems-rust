// https://adventofcode.com/2019/day/11
//
// --- Day 11: Space Police ---
//
// On the way to Jupiter, you're pulled over by the Space Police.
//
// "Attention, unmarked spacecraft! You are in violation of Space Law! All spacecraft must have a clearly visible registration identifier! You have 24 hours to comply or be sent to Space Jail!"
//
// Not wanting to be sent to Space Jail, you radio back to the Elves on Earth for help. Although it takes almost three hours for their reply signal to reach you, they send instructions for how to power up the emergency hull painting robot and even provide a small Intcode program (your puzzle input) that will cause it to paint your ship appropriately.
//
// There's just one problem: you don't have an emergency hull painting robot.
//
// You'll need to build a new emergency hull painting robot. The robot needs to be able to move around on the grid of square panels on the side of your ship, detect the color of its current panel, and paint its current panel black or white. (All of the panels are currently black.)
//
// The Intcode program will serve as the brain of the robot. The program uses input instructions to access the robot's camera: provide 0 if the robot is over a black panel or 1 if the robot is over a white panel. Then, the program will output two values:
//
//     First, it will output a value indicating the color to paint the panel the robot is over: 0 means to paint the panel black, and 1 means to paint the panel white.
//     Second, it will output a value indicating the direction the robot should turn: 0 means it should turn left 90 degrees, and 1 means it should turn right 90 degrees.
//
// After the robot turns, it should always move forward exactly one panel. The robot starts facing up.
//
// The robot will continue running for a while like this and halt when it is finished drawing. Do not restart the Intcode computer inside the robot during this process.
//
// For example, suppose the robot is about to start running. Drawing black panels as ., white panels as #, and the robot pointing the direction it is facing (< ^ > v), the initial state and region near the robot looks like this:
//
// .....
// .....
// ..^..
// .....
// .....
//
// The panel under the robot (not visible here because a ^ is shown instead) is also black, and so any input instructions at this point should be provided 0. Suppose the robot eventually outputs 1 (paint white) and then 0 (turn left). After taking these actions and moving forward one panel, the region now looks like this:
//
// .....
// .....
// .<#..
// .....
// .....
//
// Input instructions should still be provided 0. Next, the robot might output 0 (paint black) and then 0 (turn left):
//
// .....
// .....
// ..#..
// .v...
// .....
//
// After more outputs (1,0, 1,0):
//
// .....
// .....
// ..^..
// .##..
// .....
//
// The robot is now back where it started, but because it is now on a white panel, input instructions should be provided 1. After several more outputs (0,1, 1,0, 1,0), the area looks like this:
//
// .....
// ..<#.
// ...#.
// .##..
// .....
//
// Before you deploy the robot, you should probably have an estimate of the area it will cover: specifically, you need to know the number of panels it paints at least once, regardless of color. In the example above, the robot painted 6 panels at least once. (It painted its starting panel twice, but that panel is still only counted once; it also never painted the panel it ended on.)
//
// Build a new emergency hull painting robot and run the Intcode program on it. How many panels does it paint at least once?
//
// Your puzzle answer was 2373.
// --- Part Two ---
//
// You're not sure what it's trying to paint, but it's definitely not a registration identifier. The Space Police are getting impatient.
//
// Checking your external ship cameras again, you notice a white panel marked "emergency hull painting robot starting panel". The rest of the panels are still black, but it looks like the robot was expecting to start on a white panel, not a black one.
//
// Based on the Space Law Space Brochure that the Space Police attached to one of your windows, a valid registration identifier is always eight capital letters. After starting the robot on a single white panel instead, what registration identifier does it paint on your hull?
//
// Your puzzle answer was PCKRLPUK.

use std::{fs, iter, thread};
use std::collections::{HashMap, HashSet};
use std::sync::mpsc;

use anyhow::Context;

use crate::geom::{Direction, Point};
use crate::prog::{Prog, Reply};

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2019/day11")?;
    let nums: Vec<i64> =
        input.trim()
            .split(',')
            .map(|s| s.parse()
                .with_context(|| format!("Failed to parse {}", s)))
            .collect::<Result<_>>()?;

    let (_, painted) = paint(&nums, 0)?;
    println!("{}", painted.len());

    let (points, _) = paint(&nums, 1)?;
    for point in points.keys() { // check
        assert!(point.y >= 0);
        assert!(point.x >= 0);
    }
    let max_y = points.keys().map(|p| p.y).max().unwrap();
    let max_x = points.keys().map(|p| p.x).max().unwrap();
    pprint(points, max_y as usize, max_x as usize);
    Ok(())
}

fn pprint(points: HashMap<Point, i64>,
          max_y: usize,
          max_x: usize) {
    let mut grid = vec![vec![b' '; max_x + 1]; max_y + 1];

    let white_tiles = points.iter().filter(|(_, &t)| t == 1);
    for (&point, _) in white_tiles {
        grid[point.y as usize][point.x as usize] = b'#';
    }

    grid.into_iter()
        .map(|row| String::from_utf8(row).unwrap())
        .for_each(|s| println!("{}", s));
}

fn paint(nums: &Vec<i64>,
         initial_tile: i64)
         -> Result<(HashMap<Point, i64>, HashSet<Point>)> {
    let (input, from_input) = mpsc::sync_channel(16);
    let (output, from_output) = mpsc::sync_channel(0);

    let nums = nums.clone();
    let thread = thread::spawn(move || -> Result<()> {
        Prog::new(nums, from_input, [output].to_vec()).exec()?;
        Ok(())
    });

    let mut grid = HashMap::new();
    let mut painted = HashSet::new();
    let mut pos = Point::new(0, 0);
    let mut direction = Direction::N;
    grid.insert(pos, initial_tile);

    'outer: loop {
        let tile = *grid.entry(pos).or_insert(0);
        input.send(tile)?;

        let mut outputs = Vec::new();
        let replies = iter::repeat_with(|| from_output.recv());
        for reply in replies.take(2) {
            match reply? {
                Reply::Message(m) => { outputs.push(m); }
                Reply::Stopped => { break 'outer; }
            }
        }
        let new_tile = outputs[0];
        let turn = outputs[1];

        grid.insert(pos, new_tile);
        painted.insert(pos);

        direction = match direction {
            Direction::N => if turn == 0 { Direction::W } else { Direction::E },
            Direction::E => if turn == 0 { Direction::N } else { Direction::S },
            Direction::S => if turn == 0 { Direction::E } else { Direction::W },
            Direction::W => if turn == 0 { Direction::S } else { Direction::N },
        };
        pos += direction.dydx();
    }
    thread.join().unwrap()?;
    Ok((grid, painted))
}

mod geom {
    use std::ops::{Add, AddAssign};

    #[derive(Eq, PartialEq, Hash, Copy, Clone, Default, Debug)]
    pub struct Point {
        pub y: i32,
        pub x: i32,
    }

    impl Point {
        pub fn new(y: i32, x: i32) -> Point {
            Point { y, x }
        }

        #[allow(dead_code)]
        pub fn add(&mut self, v: Vector) {
            self.y += v.dy;
            self.x += v.dx;
        }
    }

    impl Add<Vector> for Point {
        type Output = Self;

        fn add(self, vector: Vector) -> Self {
            Self {
                y: self.y + vector.dy,
                x: self.x + vector.dx,
            }
        }
    }

    impl AddAssign<Vector> for Point {
        fn add_assign(&mut self, vector: Vector) {
            *self = Self {
                y: self.y + vector.dy,
                x: self.x + vector.dx,
            };
        }
    }

    #[derive(Eq, PartialEq, Hash, Copy, Clone, Default, Debug)]
    pub struct Vector {
        pub dy: i32,
        pub dx: i32,
    }

    impl Vector {
        pub fn new(dy: i32, dx: i32) -> Vector {
            Vector { dy, dx }
        }
    }

    pub enum Direction {
        N,
        E,
        S,
        W,
    }

    impl Direction {
        pub fn dydx(&self) -> Vector {
            match self {
                Direction::N => Vector::new(-1, 0),
                Direction::E => Vector::new(0, 1),
                Direction::S => Vector::new(1, 0),
                Direction::W => Vector::new(0, -1),
            }
        }
    }
}

mod prog {
    use std::convert::TryFrom;
    use std::iter;
    use std::sync::mpsc::{Receiver, SyncSender};
    use std::sync::mpsc;

    use super::Result;

    pub enum Reply {
        Message(i64),
        Stopped,
    }

    pub struct Prog {
        nums: Vec<i64>,
        i: usize,
        input: Receiver<i64>,
        outputs: Vec<SyncSender<Reply>>,
        relbase: i64,
    }

    impl Prog {
        #[allow(dead_code)]
        pub fn exec_once(nums: Vec<i64>, input_val: i64) -> Result<Vec<Reply>> {
            let (input, from_input) = mpsc::sync_channel(1);
            let (output, from_output) = mpsc::sync_channel(64);

            input.send(input_val)?;
            Prog::new(nums, from_input, [output].to_vec()).exec()?;
            let result: Vec<_> = from_output.try_iter().collect();
            Ok(result)
        }

        pub fn new(nums: Vec<i64>,
                   input: Receiver<i64>,
                   outputs: Vec<SyncSender<Reply>>) -> Prog {
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
                            .skip_while(|output| output.send(Reply::Message(num)).is_err()) {
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
            for output in self.outputs.iter() {
                output.send(Reply::Stopped)?;
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
