// https://adventofcode.com/2019/day/15
//
// Now, you know the repair droid is in a dead end. Backtrack with 3 (which you already know will get a reply of 1 because you already know that location is open):
//
//
//    ##
//    D.#
//     #
//
//
// Then, perhaps west (3) gets a reply of 0, south (2) gets a reply of 1, south again (2) gets a reply of 0, and then west (3) gets a reply of 2:
//
//
//    ##
//   #..#
//   D.#
//    #
//
// Now, because of the reply of 2, you know you've found the oxygen system! In this example, it was only 2 moves away from the repair droid's starting position.
//
// What is the fewest number of movement commands required to move the repair droid from its starting position to the location of the oxygen system?
//
// Your puzzle answer was 220.
// --- Part Two ---
//
// You quickly repair the oxygen system; oxygen gradually fills the area.
//
// Oxygen starts in the location containing the repaired oxygen system. It takes one minute for oxygen to spread to all open locations that are adjacent to a location that already contains oxygen. Diagonal locations are not adjacent.
//
// In the example above, suppose you've used the droid to explore the area fully and have the following map (where locations that currently contain oxygen are marked O):
//
//  ##
// #..##
// #.#..#
// #.O.#
//  ###
//
// Initially, the only location which contains oxygen is the location of the repaired oxygen system. However, after one minute, the oxygen spreads to all open (.) locations that are adjacent to a location containing oxygen:
//
//  ##
// #..##
// #.#..#
// #OOO#
//  ###
//
// After a total of two minutes, the map looks like this:
//
//  ##
// #..##
// #O#O.#
// #OOO#
//  ###
//
// After a total of three minutes:
//
//  ##
// #O.##
// #O#OO#
// #OOO#
//  ###
//
// And finally, the whole region is full of oxygen after a total of four minutes:
//
//  ##
// #OO##
// #O#OO#
// #OOO#
//  ###
//
// So, in this example, all locations contain oxygen after 4 minutes.
//
// Use the repair droid to get a complete map of the area. How many minutes will it take to fill with oxygen?
//
// Your puzzle answer was 334.

use std::{cmp, fs};
use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Context;

use crate::geom::{Direction, Vector2};
use crate::prog::Prog;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2019/day15")?;
    let nums: Vec<i64> =
        input.trim()
            .split(',')
            .map(|s| s.parse()
                .with_context(|| format!("Failed to parse {}", s)))
            .collect::<Result<_>>()?;

    let (grid, oxy_pos) = explore(&nums)?;
    oxyfill(grid, oxy_pos);
    Ok(())
}

fn explore(nums: &Vec<i64>) -> Result<(HashMap<Vector2, u8>, Vector2)> {
    let pos = Vector2::ZERO;
    let prog = Prog::new(nums.clone());
    let depth = 0u32;

    let mut grid = HashMap::new();
    let mut depths = HashMap::new();
    grid.insert(pos, b'@');
    depths.insert(pos, depth);

    let mut visited = HashSet::new();
    let mut to_process = VecDeque::new();
    visited.insert(pos);
    to_process.push_back((pos, prog, depth));
    while !to_process.is_empty() {
        let (pos, prog, depth) = to_process.pop_front().unwrap();

        for dir in Direction::VALUES.iter() {
            let new_pos = pos + dir.dxdy();
            if !visited.contains(&new_pos) {
                visited.insert(new_pos);

                let mut new_prog = prog.clone();
                new_prog.send_and_resume(match dir {
                    Direction::N => 1,
                    Direction::E => 4,
                    Direction::S => 2,
                    Direction::W => 3,
                })?;

                let tile_id = new_prog.recv_iter().next().unwrap();
                grid.insert(new_pos, id_to_tile(tile_id)?);

                match tile_id {
                    1 | 2 => {
                        let new_depth = depth + 1;
                        depths.insert(new_pos, new_depth);
                        to_process.push_back((new_pos, new_prog, new_depth));
                    }
                    _ => (),
                }
            }
        }
    }

    let screen = grid_to_screen(&grid);
    pprint(&screen);

    let (&pos, _) = grid.iter().find(|(_, &v)| v == b'X').unwrap();
    println!("{}", depths.get(&pos).unwrap());
    Ok((grid, pos))
}

fn oxyfill(grid: HashMap<Vector2, u8>, pos: Vector2) {
    let mut max_depth = 0;
    let mut visited = HashSet::new();
    let mut to_process = VecDeque::new();
    visited.insert(pos);
    to_process.push_back((pos, 0));
    while !to_process.is_empty() {
        let (pos, depth) = to_process.pop_front().unwrap();

        for dir in Direction::VALUES.iter() {
            let new_pos = pos + dir.dxdy();
            if grid[&new_pos] != b'#' && !visited.contains(&new_pos) {
                visited.insert(new_pos);
                max_depth = cmp::max(max_depth, depth + 1);
                to_process.push_back((new_pos, depth + 1));
            }
        }
    }
    println!("{}", max_depth);
}

fn grid_to_screen(grid: &HashMap<Vector2, u8>) -> Vec<Vec<u8>> {
    let offset_y = -grid.keys().map(|p| p.y).min().unwrap();
    let offset_x = -grid.keys().map(|p| p.x).min().unwrap();
    let max_y = grid.keys().map(|p| p.y).max().unwrap();
    let max_x = grid.keys().map(|p| p.x).max().unwrap();
    let screen_y = (max_y + offset_y) as usize;
    let screen_x = (max_x + offset_x) as usize;

    let mut screen = vec![vec![b' '; screen_x + 1]; screen_y + 1];

    for (&point, &tile) in grid {
        let y = (point.y + offset_y) as usize;
        let x = (point.x + offset_x) as usize;
        screen[y][x] = tile;
    }
    screen
}

fn pprint(screen: &Vec<Vec<u8>>) {
    screen.into_iter()
        .map(|row| std::str::from_utf8(row).unwrap())
        .for_each(|s| println!("{}", s));
}

fn id_to_tile(id: i64) -> Result<u8> {
    let res = match id {
        0 => b'#',
        1 => b'.',
        2 => b'X',
        _ => anyhow::bail!("unrecognized id"),
    };
    Ok(res)
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
