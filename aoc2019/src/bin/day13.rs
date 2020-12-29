// https://adventofcode.com/2019/day/13
//
// --- Day 13: Care Package ---
//
// As you ponder the solitude of space and the ever-increasing three-hour roundtrip for messages between you and Earth, you notice that the Space Mail Indicator Light is blinking. To help keep you sane, the Elves have sent you a care package.
//
// It's a new game for the ship's arcade cabinet! Unfortunately, the arcade is all the way on the other end of the ship. Surely, it won't be hard to build your own - the care package even comes with schematics.
//
// The arcade cabinet runs Intcode software like the game the Elves sent (your puzzle input). It has a primitive screen capable of drawing square tiles on a grid. The software draws tiles to the screen with output instructions: every three output instructions specify the x position (distance from the left), y position (distance from the top), and tile id. The tile id is interpreted as follows:
//
//     0 is an empty tile. No game object appears in this tile.
//     1 is a wall tile. Walls are indestructible barriers.
//     2 is a block tile. Blocks can be broken by the ball.
//     3 is a horizontal paddle tile. The paddle is indestructible.
//     4 is a ball tile. The ball moves diagonally and bounces off objects.
//
// For example, a sequence of output values like 1,2,3,6,5,4 would draw a horizontal paddle tile (1 tile from the left and 2 tiles from the top) and a ball tile (6 tiles from the left and 5 tiles from the top).
//
// Start the game. How many block tiles are on the screen when the game exits?
//
// Your puzzle answer was 452.
// --- Part Two ---
//
// The game didn't run because you didn't put in any quarters. Unfortunately, you did not bring any quarters. Memory address 0 represents the number of quarters that have been inserted; set it to 2 to play for free.
//
// The arcade cabinet has a joystick that can move left and right. The software reads the position of the joystick with input instructions:
//
//     If the joystick is in the neutral position, provide 0.
//     If the joystick is tilted to the left, provide -1.
//     If the joystick is tilted to the right, provide 1.
//
// The arcade cabinet also has a segment display capable of showing a single number that represents the player's current score. When three output instructions specify X=-1, Y=0, the third output instruction is not a tile; the value instead specifies the new score to show in the segment display. For example, a sequence of output values like -1,0,12345 would show 12345 as the player's current score.
//
// Beat the game by breaking all the blocks. What is your score after the last block is broken?
//
// Your puzzle answer was 21415.

use std::{fs, thread};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::mpsc;

use anyhow::Context;

use crate::geom::Vector2;
use crate::prog::{Prog, Reply};

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2019/day13")?;
    let nums: Vec<i64> =
        input.trim()
            .split(',')
            .map(|s| s.parse()
                .with_context(|| format!("Failed to parse {}", s)))
            .collect::<Result<_>>()?;

    play(&nums, false)?;
    play(&nums, true)?;
    Ok(())
}

fn play(nums: &Vec<i64>, play_for_free: bool) -> Result<()> {
    let (input, from_input) = mpsc::sync_channel(0);
    let (output, from_output) = mpsc::sync_channel(0);

    let mut nums = nums.clone();
    if play_for_free { nums[0] = 2; }
    let thread = thread::spawn(move || -> Result<()> {
        Prog::new(nums.clone(), from_input, output).exec()?;
        Ok(())
    });

    let mut grid = HashMap::new();
    let mut outputs = [0; 3];
    let mut num_blocks = 0;
    let mut score = 0;
    let mut ball_pos_x = 0;
    let mut paddle_pos_x = 0;

    'outer: loop {
        let mut i = 0;
        while i < 3 {
            let reply = from_output.recv()?;
            match reply {
                Reply::Message(m) => {
                    outputs[i] = m;
                    i += 1;
                }
                Reply::Stopped => {
                    pprint(&grid);
                    break 'outer;
                }
                Reply::Blocked => {
                    // pprint(&grid);
                    input.send(
                        match paddle_pos_x.cmp(&ball_pos_x) {
                            Ordering::Less => 1,
                            Ordering::Equal => 0,
                            Ordering::Greater => -1,
                        })?;
                }
            }
        }

        let pos = Vector2::new(outputs[0], outputs[1]);
        let id = outputs[2];
        if id == 2 { num_blocks += 1; }

        if pos.x >= 0 {
            grid.insert(pos, id_to_tile(id)?);
            if id == 4 { ball_pos_x = pos.x; }
            if id == 3 { paddle_pos_x = pos.x; }
        } else if pos.x == -1 && pos.y == 0 {
            score = id;
        } else {
            anyhow::bail!("invalid pos value: {:?}", pos);
        }
    }
    thread.join().unwrap()?;
    println!("num blocks: {}", num_blocks);
    println!("score: {}", score);

    Ok(())
}

fn pprint(grid: &HashMap<Vector2, u8>) {
    for point in grid.keys() { // check
        assert!(point.y >= 0);
        assert!(point.x >= 0);
    }
    let max_y = grid.keys().map(|p| p.y).max().unwrap() as usize;
    let max_x = grid.keys().map(|p| p.x).max().unwrap() as usize;
    let mut screen = vec![vec![b' '; max_x + 1]; max_y + 1];

    for (&point, &tile) in grid {
        screen[point.y as usize][point.x as usize] = tile;
    }

    screen.into_iter()
        .map(|row| String::from_utf8(row).unwrap())
        .for_each(|s| println!("{}", s));
}

fn id_to_tile(id: i64) -> Result<u8> {
    let res = match id {
        0 => b' ',
        1 => b'#',
        2 => b'.',
        3 => b'_',
        4 => b'O',
        _ => anyhow::bail!("unrecognized id"),
    };
    Ok(res)
}

mod prog {
    use std::convert::TryFrom;
    use std::iter;
    use std::sync::mpsc::{Receiver, SyncSender, TryRecvError};

    use super::Result;

    pub enum Reply {
        Message(i64),
        Blocked,
        Stopped,
    }

    pub struct Prog {
        nums: Vec<i64>,
        i: usize,
        input: Receiver<i64>,
        output: SyncSender<Reply>,
        relbase: i64,
    }

    impl Prog {
        pub fn new(nums: Vec<i64>,
                   input: Receiver<i64>,
                   output: SyncSender<Reply>) -> Prog {
            Prog {
                nums,
                i: 0,
                input,
                output,
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
                        let res = self.input.try_recv(); // cutoff
                        let msg = match res {
                            Ok(msg) => { msg }
                            Err(e) => {
                                match e {
                                    TryRecvError::Disconnected => return Err(anyhow::Error::from(e)),
                                    TryRecvError::Empty => {
                                        // late input window from cutoff to block
                                        self.output.send(Reply::Blocked)?;
                                        self.input.recv()? // block
                                    }
                                }
                            }
                        };

                        self.nums[pos] = msg;
                        self.i += 2;
                    }
                    4 => {
                        let num = self.read_val(modes.next().unwrap(), self.i + 1)?;
                        self.output.send(Reply::Message(num))?;
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
            self.output.send(Reply::Stopped)?;
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

mod geom {
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
}
