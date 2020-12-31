// https://adventofcode.com/2019/day/17
//
// --- Day 17: Set and Forget ---
//
// An early warning system detects an incoming solar flare and automatically activates the ship's electromagnetic shield. Unfortunately, this has cut off the Wi-Fi for many small robots that, unaware of the impending danger, are now trapped on exterior scaffolding on the unsafe side of the shield. To rescue them, you'll have to act quickly!
//
// The only tools at your disposal are some wired cameras and a small vacuum robot currently asleep at its charging station. The video quality is poor, but the vacuum robot has a needlessly bright LED that makes it easy to spot no matter where it is.
//
// An Intcode program, the Aft Scaffolding Control and Information Interface (ASCII, your puzzle input), provides access to the cameras and the vacuum robot. Currently, because the vacuum robot is asleep, you can only access the cameras.
//
// Running the ASCII program on your Intcode computer will provide the current view of the scaffolds. This is output, purely coincidentally, as ASCII code: 35 means #, 46 means ., 10 starts a new line of output below the current one, and so on. (Within a line, characters are drawn left-to-right.)
//
// In the camera output, # represents a scaffold and . represents open space. The vacuum robot is visible as ^, v, <, or > depending on whether it is facing up, down, left, or right respectively. When drawn like this, the vacuum robot is always on a scaffold; if the vacuum robot ever walks off of a scaffold and begins tumbling through space uncontrollably, it will instead be visible as X.
//
// In general, the scaffold forms a path, but it sometimes loops back onto itself. For example, suppose you can see the following view from the cameras:
//
// ..#..........
// ..#..........
// #######...###
// #.#...#...#.#
// #############
// ..#...#...#..
// ..#####...^..
//
// Here, the vacuum robot, ^ is facing up and sitting at one end of the scaffold near the bottom-right of the image. The scaffold continues up, loops across itself several times, and ends at the top-left of the image.
//
// The first step is to calibrate the cameras by getting the alignment parameters of some well-defined points. Locate all scaffold intersections; for each, its alignment parameter is the distance between its left edge and the left edge of the view multiplied by the distance between its top edge and the top edge of the view. Here, the intersections from the above image are marked O:
//
// ..#..........
// ..#..........
// ##O####...###
// #.#...#...#.#
// ##O###O###O##
// ..#...#...#..
// ..#####...^..
//
// For these intersections:
//
//     The top-left intersection is 2 units from the left of the image and 2 units from the top of the image, so its alignment parameter is 2 * 2 = 4.
//     The bottom-left intersection is 2 units from the left and 4 units from the top, so its alignment parameter is 2 * 4 = 8.
//     The bottom-middle intersection is 6 from the left and 4 from the top, so its alignment parameter is 24.
//     The bottom-right intersection's alignment parameter is 40.
//
// To calibrate the cameras, you need the sum of the alignment parameters. In the above example, this is 76.
//
// Run your ASCII program. What is the sum of the alignment parameters for the scaffold intersections?
//
// Your puzzle answer was 13580.
// --- Part Two ---
//
// Now for the tricky part: notifying all the other robots about the solar flare. The vacuum robot can do this automatically if it gets into range of a robot. However, you can't see the other robots on the camera, so you need to be thorough instead: you need to make the vacuum robot visit every part of the scaffold at least once.
//
// The vacuum robot normally wanders randomly, but there isn't time for that today. Instead, you can override its movement logic with new rules.
//
// Force the vacuum robot to wake up by changing the value in your ASCII program at address 0 from 1 to 2. When you do this, you will be automatically prompted for the new movement rules that the vacuum robot should use. The ASCII program will use input instructions to receive them, but they need to be provided as ASCII code; end each line of logic with a single newline, ASCII code 10.
//
// First, you will be prompted for the main movement routine. The main routine may only call the movement functions: A, B, or C. Supply the movement functions to use as ASCII text, separating them with commas (,, ASCII code 44), and ending the list with a newline (ASCII code 10). For example, to call A twice, then alternate between B and C three times, provide the string A,A,B,C,B,C,B,C and then a newline.
//
// Then, you will be prompted for each movement function. Movement functions may use L to turn left, R to turn right, or a number to move forward that many units. Movement functions may not call other movement functions. Again, separate the actions with commas and end the list with a newline. For example, to move forward 10 units, turn left, move forward 8 units, turn right, and finally move forward 6 units, provide the string 10,L,8,R,6 and then a newline.
//
// Finally, you will be asked whether you want to see a continuous video feed; provide either y or n and a newline. Enabling the continuous video feed can help you see what's going on, but it also requires a significant amount of processing power, and may even cause your Intcode computer to overheat.
//
// Due to the limited amount of memory in the vacuum robot, the ASCII definitions of the main routine and the movement functions may each contain at most 20 characters, not counting the newline.
//
// For example, consider the following camera feed:
//
// #######...#####
// #.....#...#...#
// #.....#...#...#
// ......#...#...#
// ......#...###.#
// ......#.....#.#
// ^########...#.#
// ......#.#...#.#
// ......#########
// ........#...#..
// ....#########..
// ....#...#......
// ....#...#......
// ....#...#......
// ....#####......
//
// In order for the vacuum robot to visit every part of the scaffold at least once, one path it could take is:
//
// R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2
//
// Without the memory limit, you could just supply this whole string to function A and have the main routine call A once. However, you'll need to split it into smaller parts.
//
// One approach is:
//
//     Main routine: A,B,C,B,A,C
//     (ASCII input: 65, 44, 66, 44, 67, 44, 66, 44, 65, 44, 67, 10)
//     Function A:   R,8,R,8
//     (ASCII input: 82, 44, 56, 44, 82, 44, 56, 10)
//     Function B:   R,4,R,4,R,8
//     (ASCII input: 82, 44, 52, 44, 82, 44, 52, 44, 82, 44, 56, 10)
//     Function C:   L,6,L,2
//     (ASCII input: 76, 44, 54, 44, 76, 44, 50, 10)
//
// Visually, this would break the desired path into the following parts:
//
// A,        B,            C,        B,            A,        C
// R,8,R,8,  R,4,R,4,R,8,  L,6,L,2,  R,4,R,4,R,8,  R,8,R,8,  L,6,L,2
//
// CCCCCCA...BBBBB
// C.....A...B...B
// C.....A...B...B
// ......A...B...B
// ......A...CCC.B
// ......A.....C.B
// ^AAAAAAAA...C.B
// ......A.A...C.B
// ......AAAAAA#AB
// ........A...C..
// ....BBBB#BBBB..
// ....B...A......
// ....B...A......
// ....B...A......
// ....BBBBA......
//
// Of course, the scaffolding outside your ship is much more complex.
//
// As the vacuum robot finds other robots and notifies them of the impending solar flare, it also can't help but leave them squeaky clean, collecting any space dust it finds. Once it finishes the programmed set of movements, assuming it hasn't drifted off into space, the cleaning robot will return to its docking station and report the amount of space dust it collected as a large, non-ASCII value in a single output instruction.
//
// After visiting every part of the scaffold at least once, how much dust does the vacuum robot report it has collected?
//
// Your puzzle answer was 1063081.

use std::collections::HashSet;
use std::fs;

use anyhow::Context;

use crate::geom::{Direction, Vector2};
use crate::prog::Prog;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2019/day17")?;
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
    let mut prog = Prog::new(nums.clone());
    prog.resume()?;
    let chars = prog.recv_iter()
        .map(|i| i as u8)
        .collect::<Vec<_>>()
        .split(|&c| c == b'\n')
        .filter(|cs| cs.len() > 0)
        .map(|cs| cs.to_vec())
        .collect::<Vec<_>>();

    for line in &chars {
        println!("{}", String::from_utf8(line.clone())?);
    }

    let mut intersections = HashSet::new();
    for y in 1..(chars.len() - 1) {
        let row = &chars[y];
        for x in 1..(row.len() - 1) {
            if chars[y][x] == b'#' {
                let pos = Vector2::new(x as i64, y as i64);
                let mut is_intersection = true;
                for dir in Direction::VALUES.iter() {
                    let new_pos = pos + dir.dxdy();
                    if chars[new_pos.y as usize][new_pos.x as usize] != b'#' {
                        is_intersection = false;
                        break;
                    }
                }
                if is_intersection {
                    intersections.insert(pos);
                }
            }
        }
    }

    let res = intersections.into_iter()
        .fold(0, |sum, pt| {
            sum + (pt.y * pt.x)
        });
    println!("{}", res);

    Ok(())
}

// ..............................................###########......
// ..............................................#.........#......
// ..............................................#.........#......
// ..............................................#.........#......
// ......#######.................................#.........#......
// ......#.....#.................................#.........#......
// ......#.....#.........#.......................#.........#......
// ......#.....#.........#.......................#.........#......
// ......#.....#.........#.......................#.........#......
// ......#.....#.........#.......................#.........#......
// ......#.....#############...................#############......
// ......#...............#.#...................#.#................
// ......#...............#############.....#######................
// ......#.................#.........#.....#...#..................
// ......#.................#.........#.....#...#..................
// ......#.................#.........#.....#...#..................
// ^######.................#############...#.#########............
// ..................................#.#...#.#.#.....#............
// ..................................#############...#............
// ....................................#...#.#.#.#...#............
// ....................................#...#############..........
// ....................................#.....#.#.#...#.#..........
// ....................................#########.#...#############
// ..........................................#...#.....#.........#
// ..........................................#...#.....#.........#
// ..........................................#...#.....#.........#
// ........................................#######.....#.........#
// ........................................#.#.........#.........#
// ..............................#############.........#.........#
// ..............................#.........#...........#.........#
// ..............................#.........#...........#.........#
// ..............................#.........#...........#.........#
// ..............................#.........#...........###########
// ..............................#.........#......................
// ..............................#.........#......................
// ..............................#.........#......................
// ..............................#.........#......................
// ..............................#.........#......................
// ..............................###########......................

// R6 L12 R6
// R6 L12 R6   L12 R6 L8 L12
// R12 L10 L10 L12 R6 L8 L12
// R12 L10 L10 L12 R6 L8 L12
// R12 L10 L10 L12 R6 L8 L12
// R6 L12 R6

fn part2(nums: &Vec<i64>) -> Result<()> {
    let mut nums = nums.clone();
    nums[0] = 2;
    let mut prog = Prog::new(nums);

    let s0 = "A,A,B,C,B,C,B,C,B,A\n";
    let s1 = "R,6,L,12,R,6\n";
    let s2 = "L,12,R,6,L,8,L,12\n";
    let s3 = "R,12,L,10,L,10\n";
    let s4 = "n\n";
    send_str_to_prog(s0, &mut prog);
    send_str_to_prog(s1, &mut prog);
    send_str_to_prog(s2, &mut prog);
    send_str_to_prog(s3, &mut prog);
    send_str_to_prog(s4, &mut prog);
    prog.resume()?;

    let mut out = prog.recv_iter().collect::<Vec<_>>();
    let score = out.pop().unwrap();

    let chars = out.into_iter().map(|i| i as u8).collect::<Vec<_>>();
    let s = String::from_utf8(chars)?;
    println!("{}", s);
    println!("{}", score);

    Ok(())
}

fn send_str_to_prog(s: &str, prog: &mut Prog) {
    for i in s.bytes().map(|c| c as i64) {
        prog.send(i);
    }
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
        pub fn send_and_resume(&mut self, i: i64) -> Result<Status>
        {
            self.send(i);
            self.resume()
        }

        pub fn send(&mut self, i: i64) {
            self.input.push_back(i);
        }

        pub fn recv_iter(&mut self) -> Drain<i64> {
            self.output.drain(..)
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

mod geom {
    use std::ops::{Add, AddAssign, Neg, SubAssign};

    #[derive(Eq, PartialEq, Hash, Copy, Clone, Default, Debug)]
    pub struct Vector2 {
        pub x: i64,
        pub y: i64,
    }

    impl Vector2 {
        #[allow(dead_code)]
        pub const ZERO: Vector2 = Vector2 { x: 0, y: 0 };

        pub fn new(x: i64, y: i64) -> Vector2 {
            Vector2 { x, y }
        }
    }


    impl Add for Vector2 {
        type Output = Self;

        fn add(self, other: Vector2) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl AddAssign for Vector2 {
        fn add_assign(&mut self, other: Vector2) {
            *self = Self {
                x: self.x + other.x,
                y: self.y + other.y,
            };
        }
    }

    impl SubAssign for Vector2 {
        fn sub_assign(&mut self, other: Vector2) {
            *self = Self {
                x: self.x - other.x,
                y: self.y - other.y,
            };
        }
    }

    impl Neg for Vector2 {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Vector2 {
                x: -self.x,
                y: -self.y,
            }
        }
    }

    pub enum Direction {
        N,
        E,
        S,
        W,
    }

    impl Direction {
        pub const VALUES: [Direction; 4] = [
            Direction::N,
            Direction::E,
            Direction::S,
            Direction::W,
        ];

        pub fn dxdy(&self) -> Vector2 {
            match self {
                Direction::N => Vector2::new(0, -1),
                Direction::E => Vector2::new(1, 0),
                Direction::S => Vector2::new(0, 1),
                Direction::W => Vector2::new(-1, 0),
            }
        }

        #[allow(dead_code)]
        pub fn cw(&self) -> Direction {
            match self {
                Direction::N => Direction::E,
                Direction::E => Direction::S,
                Direction::S => Direction::W,
                Direction::W => Direction::N,
            }
        }

        #[allow(dead_code)]
        pub fn ccw(&self) -> Direction {
            match self {
                Direction::N => Direction::W,
                Direction::E => Direction::N,
                Direction::S => Direction::E,
                Direction::W => Direction::S,
            }
        }
    }
}
