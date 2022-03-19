// https://adventofcode.com/2021/day/5
//
// --- Day 5: Hydrothermal Venture ---
//
// You come across a field of hydrothermal vents on the ocean floor! These vents constantly produce large, opaque clouds, so it would be best to avoid them if possible.
//
// They tend to form in lines; the submarine helpfully produces a list of nearby lines of vents (your puzzle input) for you to review. For example:
//
// 0,9 -> 5,9
// 8,0 -> 0,8
// 9,4 -> 3,4
// 2,2 -> 2,1
// 7,0 -> 7,4
// 6,4 -> 2,0
// 0,9 -> 2,9
// 3,4 -> 1,4
// 0,0 -> 8,8
// 5,5 -> 8,2
//
// Each line of vents is given as a line segment in the format x1,y1 -> x2,y2 where x1,y1 are the coordinates of one end the line segment and x2,y2 are the coordinates of the other end. These line segments include the points at both ends. In other words:
//
//     An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
//     An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.
//
// For now, only consider horizontal and vertical lines: lines where either x1 = x2 or y1 = y2.
//
// So, the horizontal and vertical lines from the above list would produce the following diagram:
//
// .......1..
// ..1....1..
// ..1....1..
// .......1..
// .112111211
// ..........
// ..........
// ..........
// ..........
// 222111....
//
// In this diagram, the top left corner is 0,0 and the bottom right corner is 9,9. Each position is shown as the number of lines which cover that point or . if no line covers that point. The top-left pair of 1s, for example, comes from 2,2 -> 2,1; the very bottom row is formed by the overlapping lines 0,9 -> 5,9 and 0,9 -> 2,9.
//
// To avoid the most dangerous areas, you need to determine the number of points where at least two lines overlap. In the above example, this is anywhere in the diagram with a 2 or larger - a total of 5 points.
//
// Consider only horizontal and vertical lines. At how many points do at least two lines overlap?
//
// Your puzzle answer was 7142.
// --- Part Two ---
//
// Unfortunately, considering only horizontal and vertical lines doesn't give you the full picture; you need to also consider diagonal lines.
//
// Because of the limits of the hydrothermal vent mapping system, the lines in your list will only ever be horizontal, vertical, or a diagonal line at exactly 45 degrees. In other words:
//
//     An entry like 1,1 -> 3,3 covers points 1,1, 2,2, and 3,3.
//     An entry like 9,7 -> 7,9 covers points 9,7, 8,8, and 7,9.
//
// Considering all lines from the above example would now produce the following diagram:
//
// 1.1....11.
// .111...2..
// ..2.1.111.
// ...1.2.2..
// .112313211
// ...1.2....
// ..1...1...
// .1.....1..
// 1.......1.
// 222111....
//
// You still need to determine the number of points where at least two lines overlap. In the above example, this is still anywhere in the diagram with a 2 or larger - now a total of 12 points.
//
// Consider all of the lines. At how many points do at least two lines overlap?
//
// Your puzzle answer was 20012.

use std::collections::HashMap;
use std::fs;

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

use aoc2021::TrimEmpty;

use crate::geom::Vector2;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2021/day5")?;
    let lines = parse(&input);
    println!("{:?}", solve(&lines, false));
    println!("{:?}", solve(&lines, true));
    Ok(())
}

fn parse(s: &str) -> Vec<Line> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    }
    s.lines().trim_empty().map(|line| {
        let caps = RE.captures(line).unwrap();
        let p1 = Vector2::new(
            caps[1].parse::<u32>().unwrap(),
            caps[2].parse::<u32>().unwrap());
        let p2 = Vector2::new(
            caps[3].parse::<u32>().unwrap(),
            caps[4].parse::<u32>().unwrap());
        Line { p1, p2 }
    }).collect_vec()
}

fn solve(lines: &Vec<Line>, do_diagonal: bool) -> usize {
    let mut counts = HashMap::new();

    for line in lines {
        if line.is_vertical() ||
            line.is_horizontal() ||
            (do_diagonal && line.is_diagonal())
        {
            let p1p2 = line.p2 - line.p1;
            let dxdy = Vector2::new(p1p2.x.signum(), p1p2.y.signum());

            let mut p = line.p1;
            while p != (line.p2 + dxdy) {
                *counts.entry(p).or_insert(0) += 1;
                p += dxdy;
            }
        }
    }
    counts.iter().filter(|(_, &v)| v > 1).count()
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Line {
    p1: Vector2,
    p2: Vector2,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    fn is_diagonal(&self) -> bool {
        (self.p1.x - self.p2.x).abs() ==
            (self.p1.y - self.p2.y).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
        ";
        let lines = parse(s);
        assert_eq!(5, solve(&lines, false));
        assert_eq!(12, solve(&lines, true));
        Ok(())
    }
}

mod geom {
    use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

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

        #[allow(dead_code)]
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

    impl Sub for Vector2 {
        type Output = Self;

        fn sub(self, other: Vector2) -> Self {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
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
}