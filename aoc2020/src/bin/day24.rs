// https://adventofcode.com/2020/day/24
//
// --- Day 24: Lobby Layout ---
//
// Your raft makes it to the tropical island; it turns out that the small crab was an excellent navigator. You make your way to the resort.
//
// As you enter the lobby, you discover a small problem: the floor is being renovated. You can't even reach the check-in desk until they've finished installing the new tile floor.
//
// The tiles are all hexagonal; they need to be arranged in a hex grid with a very specific color pattern. Not in the mood to wait, you offer to help figure out the pattern.
//
// The tiles are all white on one side and black on the other. They start with the white side facing up. The lobby is large enough to fit whatever pattern might need to appear there.
//
// A member of the renovation crew gives you a list of the tiles that need to be flipped over (your puzzle input). Each line in the list identifies a single tile that needs to be flipped by giving a series of steps starting from a reference tile in the very center of the room. (Every line starts from the same reference tile.)
//
// Because the tiles are hexagonal, every tile has six neighbors: east, southeast, southwest, west, northwest, and northeast. These directions are given in your list, respectively, as e, se, sw, w, nw, and ne. A tile is identified by a series of these directions with no delimiters; for example, esenee identifies the tile you land on if you start at the reference tile and then move one tile east, one tile southeast, one tile northeast, and one tile east.
//
// Each time a tile is identified, it flips from white to black or from black to white. Tiles might be flipped more than once. For example, a line like esew flips a tile immediately adjacent to the reference tile, and a line like nwwswee flips the reference tile itself.
//
// Here is a larger example:
//
// sesenwnenenewseeswwswswwnenewsewsw
// neeenesenwnwwswnenewnwwsewnenwseswesw
// seswneswswsenwwnwse
// nwnwneseeswswnenewneswwnewseswneseene
// swweswneswnenwsewnwneneseenw
// eesenwseswswnenwswnwnwsewwnwsene
// sewnenenenesenwsewnenwwwse
// wenwwweseeeweswwwnwwe
// wsweesenenewnwwnwsenewsenwwsesesenwne
// neeswseenwwswnwswswnw
// nenwswwsewswnenenewsenwsenwnesesenew
// enewnwewneswsewnwswenweswnenwsenwsw
// sweneswneswneneenwnewenewwneswswnese
// swwesenesewenwneswnwwneseswwne
// enesenwswwswneneswsenwnewswseenwsese
// wnwnesenesenenwwnenwsewesewsesesew
// nenewswnwewswnenesenwnesewesw
// eneswnwswnwsenenwnwnwwseeswneewsenese
// neswnwewnwnwseenwseesewsenwsweewe
// wseweeenwnesenwwwswnew
//
// In the above example, 10 tiles are flipped once (to black), and 5 more are flipped twice (to black, then back to white). After all of these instructions have been followed, a total of 10 tiles are black.
//
// Go through the renovation crew's list and determine which tiles they need to flip. After all of the instructions have been followed, how many tiles are left with the black side up?
//
// Your puzzle answer was 377.
// --- Part Two ---
//
// The tile floor in the lobby is meant to be a living art exhibit. Every day, the tiles are all flipped according to the following rules:
//
//     Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
//     Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
//
// Here, tiles immediately adjacent means the six tiles directly touching the tile in question.
//
// The rules are applied simultaneously to every tile; put another way, it is first determined which tiles need to be flipped, then they are all flipped at the same time.
//
// In the above example, the number of black tiles that are facing up after the given number of days has passed is as follows:
//
// Day 1: 15
// Day 2: 12
// Day 3: 25
// Day 4: 14
// Day 5: 23
// Day 6: 28
// Day 7: 41
// Day 8: 37
// Day 9: 49
// Day 10: 37
//
// Day 20: 132
// Day 30: 259
// Day 40: 406
// Day 50: 566
// Day 60: 788
// Day 70: 1106
// Day 80: 1373
// Day 90: 1844
// Day 100: 2208
//
// After executing this process a total of 100 times, there would be 2208 black tiles facing up.
//
// How many tiles will be black after 100 days?
//
// Your puzzle answer was 4231.

use std::collections::{HashMap, HashSet};
use std::fs;

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

use aoc2020::TrimEmpty;

use crate::geom::{Direction, Vector2};

// Axial coordinates:
// https://www.redblobgames.com/grids/hexagons/

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2020/day24")?;
    let moves = parse(&input);
    let mut blacks = flip(&moves);
    println!("{}", blacks.len());
    for _ in 0..100 {
        tick(&mut blacks);
    }
    println!("{}", blacks.len());
    Ok(())
}

fn parse(s: &str) -> Vec<Vec<Direction>> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"(e|se|sw|w|nw|ne)").unwrap();
    }
    s.lines().trim_empty()
        .map(|line|
            RE.find_iter(line)
                .map(|m| match m.as_str() {
                    "e" => Direction::E,
                    "se" => Direction::SE,
                    "sw" => Direction::SW,
                    "w" => Direction::W,
                    "nw" => Direction::NW,
                    "ne" => Direction::NE,
                    _ => unreachable!(),
                })
                .collect_vec())
        .collect_vec()
}

fn flip(paths: &Vec<Vec<Direction>>) -> HashSet<Vector2> {
    fn walk(path: &Vec<Direction>) -> Vector2 {
        path.iter().fold(Vector2::ZERO, |pos, dir|
            pos + dir.dxdy(),
        )
    }
    let mut blacks = HashSet::new();
    for coord in paths.iter().map(walk) {
        if !blacks.remove(&coord) {
            blacks.insert(coord);
        }
    }
    blacks
}

fn tick(blacks: &mut HashSet<Vector2>) {
    let mut neighbours = HashMap::new();
    for &coord in blacks.iter() {
        neighbours.entry(coord).or_insert(0);
        for dir in Direction::VALUES.iter() {
            let neighbour = coord + dir.dxdy();
            *neighbours.entry(neighbour).or_insert(0) += 1;
        }
    }
    // no overlap in conditions
    for (coord, n) in neighbours.iter() {
        match n {
            2 => { blacks.insert(*coord); }
            0 | 3..=6 => { blacks.remove(coord); }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        sesenwnenenewseeswwswswwnenewsewsw
        neeenesenwnwwswnenewnwwsewnenwseswesw
        seswneswswsenwwnwse
        nwnwneseeswswnenewneswwnewseswneseene
        swweswneswnenwsewnwneneseenw
        eesenwseswswnenwswnwnwsewwnwsene
        sewnenenenesenwsewnenwwwse
        wenwwweseeeweswwwnwwe
        wsweesenenewnwwnwsenewsenwwsesesenwne
        neeswseenwwswnwswswnw
        nenwswwsewswnenenewsenwsenwnesesenew
        enewnwewneswsewnwswenweswnenwsenwsw
        sweneswneswneneenwnewenewwneswswnese
        swwesenesewenwneswnwwneseswwne
        enesenwswwswneneswsenwnewswseenwsese
        wnwnesenesenenwwnenwsewesewsesesew
        nenewswnwewswnenesenwnesewesw
        eneswnwswnwsenenwnwnwwseeswneewsenese
        neswnwewnwnwseenwseesewsenwsweewe
        wseweeenwnesenwwwswnew
        ";
        let moves = parse(&s);
        let mut blacks = flip(&moves);
        assert_eq!(10, blacks.len());
        for _ in 0..100 {
            tick(&mut blacks);
        }
        assert_eq!(2208, blacks.len());
        Ok(())
    }
}

mod geom {
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
        E,
        SE,
        SW,
        W,
        NW,
        NE,
    }

    impl Direction {
        #[allow(dead_code)]
        pub const VALUES: [Direction; 6] = [
            Direction::E,
            Direction::SE,
            Direction::SW,
            Direction::W,
            Direction::NW,
            Direction::NE,
        ];

        #[allow(dead_code)]
        pub fn dxdy(&self) -> Vector2 {
            match self {
                Direction::E => Vector2::new(1, 0),
                Direction::SE => Vector2::new(0, 1),
                Direction::SW => Vector2::new(-1, 1),
                Direction::W => Vector2::new(-1, 0),
                Direction::NW => Vector2::new(0, -1),
                Direction::NE => Vector2::new(1, -1),
            }
        }
    }
}