// https://adventofcode.com/2021/day/13
//
// --- Day 13: Transparent Origami ---
//
// You reach another volcanically active part of the cave. It would be nice if you could do some kind of thermal imaging so you could tell ahead of time which caves are too hot to safely enter.
//
// Fortunately, the submarine seems to be equipped with a thermal camera! When you activate it, you are greeted with:
//
// Congratulations on your purchase! To activate this infrared thermal imaging
// camera system, please enter the code found on page 1 of the manual.
//
// Apparently, the Elves have never used this feature. To your surprise, you manage to find the manual; as you go to open it, page 1 falls out. It's a large sheet of transparent paper! The transparent paper is marked with random dots and includes instructions on how to fold it up (your puzzle input). For example:
//
// 6,10
// 0,14
// 9,10
// 0,3
// 10,4
// 4,11
// 6,0
// 6,12
// 4,1
// 0,13
// 10,12
// 3,4
// 3,0
// 8,4
// 1,10
// 2,14
// 8,10
// 9,0
//
// fold along y=7
// fold along x=5
//
// The first section is a list of dots on the transparent paper. 0,0 represents the top-left coordinate. The first value, x, increases to the right. The second value, y, increases downward. So, the coordinate 3,0 is to the right of 0,0, and the coordinate 0,7 is below 0,0. The coordinates in this example form the following pattern, where # is a dot on the paper and . is an empty, unmarked position:
//
// ...#..#..#.
// ....#......
// ...........
// #..........
// ...#....#.#
// ...........
// ...........
// ...........
// ...........
// ...........
// .#....#.##.
// ....#......
// ......#...#
// #..........
// #.#........
//
// Then, there is a list of fold instructions. Each instruction indicates a line on the transparent paper and wants you to fold the paper up (for horizontal y=... lines) or left (for vertical x=... lines). In this example, the first fold instruction is fold along y=7, which designates the line formed by all of the positions where y is 7 (marked here with -):
//
// ...#..#..#.
// ....#......
// ...........
// #..........
// ...#....#.#
// ...........
// ...........
// -----------
// ...........
// ...........
// .#....#.##.
// ....#......
// ......#...#
// #..........
// #.#........
//
// Because this is a horizontal line, fold the bottom half up. Some of the dots might end up overlapping after the fold is complete, but dots will never appear exactly on a fold line. The result of doing this fold looks like this:
//
// #.##..#..#.
// #...#......
// ......#...#
// #...#......
// .#.#..#.###
// ...........
// ...........
//
// Now, only 17 dots are visible.
//
// Notice, for example, the two dots in the bottom left corner before the transparent paper is folded; after the fold is complete, those dots appear in the top left corner (at 0,0 and 0,1). Because the paper is transparent, the dot just below them in the result (at 0,3) remains visible, as it can be seen through the transparent paper.
//
// Also notice that some dots can end up overlapping; in this case, the dots merge together and become a single dot.
//
// The second fold instruction is fold along x=5, which indicates this line:
//
// #.##.|#..#.
// #...#|.....
// .....|#...#
// #...#|.....
// .#.#.|#.###
// .....|.....
// .....|.....
//
// Because this is a vertical line, fold left:
//
// #####
// #...#
// #...#
// #...#
// #####
// .....
// .....
//
// The instructions made a square!
//
// The transparent paper is pretty big, so for now, focus on just completing the first fold. After the first fold in the example above, 17 dots are visible - dots that end up overlapping after the fold is completed count as a single dot.
//
// How many dots are visible after completing just the first fold instruction on your transparent paper?
//
// Your puzzle answer was 781.
// --- Part Two ---
//
// Finish folding the transparent paper according to the instructions. The manual says the code is always eight capital letters.
//
// What code do you use to activate the infrared thermal imaging camera system?
//
// Your puzzle answer was PERCGJPB.

use std::collections::HashSet;
use std::fs;

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

use aoc2021::TrimEmpty;

use crate::geom::Vector2;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2021/day13")?;
    let (points, folds) = parse(&input);
    println!("{:?}", part1(&points, &folds));
    part2(&points, &folds);
    Ok(())
}

fn parse(s: &str) -> (HashSet<Vector2>, Vec<Fold>) {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"^fold along (x|y)=(\d+)$").unwrap();
    }
    let (points, folds) = s.split_once("\n\n").unwrap();
    let points = points.lines().trim_empty().map(|line| {
        let (x, y) = line.split_once(',').unwrap();
        let x = x.parse::<i32>().unwrap();
        let y = y.parse::<i32>().unwrap();
        Vector2::new(x, y)
    }).collect();

    let folds = folds.lines().trim_empty().map(|line| {
        let caps = RE.captures(line).unwrap();
        let foldtype = match &caps[1] {
            "x" => FoldType::VERTICAL,
            "y" => FoldType::HORIZONTAL,
            _ => unreachable!()
        };
        let val = caps[2].parse().unwrap();
        Fold { foldtype, val }
    }).collect_vec();

    (points, folds)
}

fn part1(points: &HashSet<Vector2>, folds: &Vec<Fold>) -> usize {
    fold(points, folds.first().unwrap()).len()
}

fn part2(points: &HashSet<Vector2>, folds: &Vec<Fold>) {
    let ps = folds.iter().fold(None, |ps, f| {
        match ps {
            None => Some(fold(points, f)),
            Some(ps) => Some(fold(&ps, f)),
        }
    }).unwrap();
    pprint(&ps);
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum FoldType {
    HORIZONTAL,
    VERTICAL,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Fold {
    foldtype: FoldType,
    val: i64,
}

fn fold(points: &HashSet<Vector2>, fold: &Fold) -> HashSet<Vector2> {
    points.iter().map(|&Vector2 { x, y }| {
        match fold.foldtype {
            FoldType::HORIZONTAL if y > fold.val => Vector2::new(x, (2 * fold.val) - y),
            FoldType::VERTICAL if x > fold.val => Vector2::new((2 * fold.val) - x, y),
            _ => Vector2::new(x, y),
        }
    }).collect()
}

fn pprint(points: &HashSet<Vector2>) {
    let max_y = points.iter().map(|p| p.y).max().unwrap() as usize;
    let max_x = points.iter().map(|p| p.x).max().unwrap() as usize;
    let mut screen = vec![vec![b' '; max_x + 1]; max_y + 1];

    for point in points {
        screen[point.y as usize][point.x as usize] = b'#';
    }

    screen.into_iter()
        .map(|row| String::from_utf8(row).unwrap())
        .for_each(|s| println!("{}", s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
        ";
        let (points, folds) = parse(&s);
        assert_eq!(17, part1(&points, &folds));
        part2(&points, &folds);
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

    #[allow(dead_code)]
    impl Vector2 {
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

    #[allow(dead_code)]
    impl Direction {
        pub const VALUES_4D: [Direction; 4] = [
            Direction::N,
            Direction::E,
            Direction::S,
            Direction::W,
        ];

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

        pub fn cw(&self, num_turns: isize) -> Direction {
            let dir = ((*self as isize) + num_turns) % 8;
            dir.try_into().unwrap()
        }

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
