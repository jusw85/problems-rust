// https://adventofcode.com/2020/day/12
//
// --- Day 12: Rain Risk ---
//
// Your ferry made decent progress toward the island, but the storm came in faster than anyone expected. The ferry needs to take evasive actions!
//
// Unfortunately, the ship's navigation computer seems to be malfunctioning; rather than giving a route directly to safety, it produced extremely circuitous instructions. When the captain uses the PA system to ask if anyone can help, you quickly volunteer.
//
// The navigation instructions (your puzzle input) consists of a sequence of single-character actions paired with integer input values. After staring at them for a few minutes, you work out what they probably mean:
//
//     Action N means to move north by the given value.
//     Action S means to move south by the given value.
//     Action E means to move east by the given value.
//     Action W means to move west by the given value.
//     Action L means to turn left the given number of degrees.
//     Action R means to turn right the given number of degrees.
//     Action F means to move forward by the given value in the direction the ship is currently facing.
//
// The ship starts by facing east. Only the L and R actions change the direction the ship is facing. (That is, if the ship is facing east and the next instruction is N10, the ship would move north 10 units, but would still move east if the following action were F.)
//
// For example:
//
// F10
// N3
// F7
// R90
// F11
//
// These instructions would be handled as follows:
//
//     F10 would move the ship 10 units east (because the ship starts by facing east) to east 10, north 0.
//     N3 would move the ship 3 units north to east 10, north 3.
//     F7 would move the ship another 7 units east (because the ship is still facing east) to east 17, north 3.
//     R90 would cause the ship to turn right by 90 degrees and face south; it remains at east 17, north 3.
//     F11 would move the ship 11 units south to east 17, south 8.
//
// At the end of these instructions, the ship's Manhattan distance (sum of the absolute values of its east/west position and its north/south position) from its starting position is 17 + 8 = 25.
//
// Figure out where the navigation instructions lead. What is the Manhattan distance between that location and the ship's starting position?
//
// Your puzzle answer was 1482.
// --- Part Two ---
//
// Before you can give the destination to the captain, you realize that the actual action meanings were printed on the back of the instructions the whole time.
//
// Almost all of the actions indicate how to move a waypoint which is relative to the ship's position:
//
//     Action N means to move the waypoint north by the given value.
//     Action S means to move the waypoint south by the given value.
//     Action E means to move the waypoint east by the given value.
//     Action W means to move the waypoint west by the given value.
//     Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
//     Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
//     Action F means to move forward to the waypoint a number of times equal to the given value.
//
// The waypoint starts 10 units east and 1 unit north relative to the ship. The waypoint is relative to the ship; that is, if the ship moves, the waypoint moves with it.
//
// For example, using the same instructions as above:
//
//     F10 moves the ship to the waypoint 10 times (a total of 100 units east and 10 units north), leaving the ship at east 100, north 10. The waypoint stays 10 units east and 1 unit north of the ship.
//     N3 moves the waypoint 3 units north to 10 units east and 4 units north of the ship. The ship remains at east 100, north 10.
//     F7 moves the ship to the waypoint 7 times (a total of 70 units east and 28 units north), leaving the ship at east 170, north 38. The waypoint stays 10 units east and 4 units north of the ship.
//     R90 rotates the waypoint around the ship clockwise 90 degrees, moving it to 4 units east and 10 units south of the ship. The ship remains at east 170, north 38.
//     F11 moves the ship to the waypoint 11 times (a total of 44 units east and 110 units south), leaving the ship at east 214, south 72. The waypoint stays 4 units east and 10 units south of the ship.
//
// After these operations, the ship's Manhattan distance from its starting position is 214 + 72 = 286.
//
// Figure out where the navigation instructions actually lead. What is the Manhattan distance between that location and the ship's starting position?
//
// Your puzzle answer was 48739.

use std::fs;
use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;

use crate::geom::{Direction, Vector2};

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2020/day12")?;
    let moves = parse(&input)?;
    let mut ship = Ship::default();
    ship.navigate(&moves);
    println!("{:?}", ship.pos.manhattan_distance(Vector2::ZERO));

    let mut waypoint = Vector2::new(10, -1);
    let mut ship = Ship::default();
    ship.navigate2(&moves, &mut waypoint);
    println!("{:?}", ship.pos.manhattan_distance(Vector2::ZERO));
    Ok(())
}

fn parse(s: &str) -> Result<Vec<Move>> {
    s.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Move>())
        .try_collect()
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Ship {
    pos: Vector2,
    facing: Direction,
}

impl Ship {
    fn navigate(&mut self, moves: &Vec<Move>) {
        for mov in moves {
            match mov.mov {
                b'L' => self.facing = self.facing.ccw((mov.arg / 45) as isize),
                b'R' => self.facing = self.facing.cw((mov.arg / 45) as isize),
                b'N' => self.pos += Direction::N.dxdy() * mov.arg,
                b'E' => self.pos += Direction::E.dxdy() * mov.arg,
                b'S' => self.pos += Direction::S.dxdy() * mov.arg,
                b'W' => self.pos += Direction::W.dxdy() * mov.arg,
                b'F' => self.pos += self.facing.dxdy() * mov.arg,
                _ => panic!("unrecognized move: {}", mov.mov)
            }
        }
    }

    fn navigate2(&mut self, moves: &Vec<Move>, waypoint: &Vector2) {
        fn rotate(pt: Vector2, mut angle: i64) -> Vector2 {
            angle %= 360;
            if angle < 0 { angle += 360; }
            match angle {
                0 => pt,
                90 => Vector2::new(pt.y, -pt.x),
                180 => Vector2::new(-pt.x, -pt.y),
                270 => Vector2::new(-pt.y, pt.x),
                _ => panic!("invalid angle"),
            }
        }

        let mut waypoint = waypoint.clone();
        for mov in moves {
            match mov.mov {
                b'L' => waypoint = rotate(waypoint, mov.arg),
                b'R' => waypoint = rotate(waypoint, -mov.arg),
                b'N' => waypoint += Direction::N.dxdy() * mov.arg,
                b'E' => waypoint += Direction::E.dxdy() * mov.arg,
                b'S' => waypoint += Direction::S.dxdy() * mov.arg,
                b'W' => waypoint += Direction::W.dxdy() * mov.arg,
                b'F' => self.pos += waypoint * mov.arg,
                _ => panic!("unrecognized move: {}", mov.mov)
            }
        }
    }
}

impl Default for Ship {
    fn default() -> Self {
        Ship { pos: Vector2::ZERO, facing: Direction::E }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Move {
    mov: u8,
    arg: i64,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, rest) = s.split_at(1);
        let mov = first.as_bytes()[0];
        let arg = rest.parse::<i64>()?;
        Ok(Move { mov, arg })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        F10
        N3
        F7
        R90
        F11
        ";
        let moves = parse(s)?;
        let mut ship = Ship::default();
        ship.navigate(&moves);
        assert_eq!(25, ship.pos.manhattan_distance(Vector2::ZERO));

        let mut ship = Ship::default();
        let mut waypoint = Vector2::new(10, -1);
        ship.navigate2(&moves, &mut waypoint);
        assert_eq!(286, ship.pos.manhattan_distance(Vector2::ZERO));
        Ok(())
    }
}

mod geom {
    use std::convert::{TryFrom, TryInto};
    use std::ops::{Add, AddAssign, Mul, Neg, SubAssign};

    use num::{NumCast, ToPrimitive};

    #[derive(Eq, PartialEq, Hash, Copy, Clone, Default, Debug)]
    pub struct Vector2 {
        pub x: i64,
        pub y: i64,
    }

    impl Vector2 {
        #[allow(dead_code)]
        pub const ZERO: Vector2 = Vector2 { x: 0, y: 0 };

        pub fn new<T>(x: T, y: T) -> Vector2
            where T: ToPrimitive
        {
            let x = NumCast::from::<T>(x).unwrap();
            let y = NumCast::from::<T>(y).unwrap();
            Vector2 { x, y }
        }

        pub fn manhattan_distance(&self, other: Vector2) -> i64 {
            (self.y - other.y).abs() + (self.x - other.x).abs()
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

    impl Mul<i64> for Vector2 {
        type Output = Self;

        fn mul(self, rhs: i64) -> Self::Output {
            Self {
                x: self.x * rhs,
                y: self.y * rhs,
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

    #[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
    pub enum Direction {
        N,
        NE,
        E,
        SE,
        S,
        SW,
        W,
        NW,
    }

    impl Direction {
        #[allow(dead_code)]
        pub const VALUES_4D: [Direction; 4] = [
            Direction::N,
            Direction::E,
            Direction::S,
            Direction::W,
        ];

        #[allow(dead_code)]
        pub const VALUES_8D: [Direction; 8] = [
            Direction::N,
            Direction::NE,
            Direction::E,
            Direction::SE,
            Direction::S,
            Direction::SW,
            Direction::W,
            Direction::NW,
        ];

        #[allow(dead_code)]
        pub fn dxdy(&self) -> Vector2 {
            match self {
                Direction::N => Vector2::new(0, -1),
                Direction::NE => Vector2::new(1, -1),
                Direction::E => Vector2::new(1, 0),
                Direction::SE => Vector2::new(1, 1),
                Direction::S => Vector2::new(0, 1),
                Direction::SW => Vector2::new(-1, 1),
                Direction::W => Vector2::new(-1, 0),
                Direction::NW => Vector2::new(-1, -1),
            }
        }

        #[allow(dead_code)]
        pub fn cw(&self, num_turns: isize) -> Direction {
            let dir = ((*self as isize) + num_turns) % 8;
            dir.try_into().unwrap()
        }

        #[allow(dead_code)]
        pub fn ccw(&self, num_turns: isize) -> Direction {
            let mut dir = ((*self as isize) - num_turns) % 8;
            if dir < 0 { dir += 8; }
            dir.try_into().unwrap()
        }
    }

    impl TryFrom<isize> for Direction {
        type Error = ();

        fn try_from(i: isize) -> Result<Self, Self::Error> {
            match i {
                x if x == Direction::N as isize => Ok(Direction::N),
                x if x == Direction::NE as isize => Ok(Direction::NE),
                x if x == Direction::E as isize => Ok(Direction::E),
                x if x == Direction::SE as isize => Ok(Direction::SE),
                x if x == Direction::S as isize => Ok(Direction::S),
                x if x == Direction::SW as isize => Ok(Direction::SW),
                x if x == Direction::W as isize => Ok(Direction::W),
                x if x == Direction::NW as isize => Ok(Direction::NW),
                _ => Err(()),
            }
        }
    }
}
