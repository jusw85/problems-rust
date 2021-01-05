// https://adventofcode.com/2019/day/20
//
// --- Day 20: Donut Maze ---
//
// You notice a strange pattern on the surface of Pluto and land nearby to get a closer look. Upon closer inspection, you realize you've come across one of the famous space-warping mazes of the long-lost Pluto civilization!
//
// Because there isn't much space on Pluto, the civilization that used to live here thrived by inventing a method for folding spacetime. Although the technology is no longer understood, mazes like this one provide a small glimpse into the daily life of an ancient Pluto citizen.
//
// This maze is shaped like a donut. Portals along the inner and outer edge of the donut can instantly teleport you from one side to the other. For example:
//
//          A
//          A
//   #######.#########
//   #######.........#
//   #######.#######.#
//   #######.#######.#
//   #######.#######.#
//   #####  B    ###.#
// BC...##  C    ###.#
//   ##.##       ###.#
//   ##...DE  F  ###.#
//   #####    G  ###.#
//   #########.#####.#
// DE..#######...###.#
//   #.#########.###.#
// FG..#########.....#
//   ###########.#####
//              Z
//              Z
//
// This map of the maze shows solid walls (#) and open passages (.). Every maze on Pluto has a start (the open tile next to AA) and an end (the open tile next to ZZ). Mazes on Pluto also have portals; this maze has three pairs of portals: BC, DE, and FG. When on an open tile next to one of these labels, a single step can take you to the other tile with the same label. (You can only walk on . tiles; labels and empty space are not traversable.)
//
// One path through the maze doesn't require any portals. Starting at AA, you could go down 1, right 8, down 12, left 4, and down 1 to reach ZZ, a total of 26 steps.
//
// However, there is a shorter path: You could walk from AA to the inner BC portal (4 steps), warp to the outer BC portal (1 step), walk to the inner DE (6 steps), warp to the outer DE (1 step), walk to the outer FG (4 steps), warp to the inner FG (1 step), and finally walk to ZZ (6 steps). In total, this is only 23 steps.
//
// Here is a larger example:
//
//                    A
//                    A
//   #################.#############
//   #.#...#...................#.#.#
//   #.#.#.###.###.###.#########.#.#
//   #.#.#.......#...#.....#.#.#...#
//   #.#########.###.#####.#.#.###.#
//   #.............#.#.....#.......#
//   ###.###########.###.#####.#.#.#
//   #.....#        A   C    #.#.#.#
//   #######        S   P    #####.#
//   #.#...#                 #......VT
//   #.#.#.#                 #.#####
//   #...#.#               YN....#.#
//   #.###.#                 #####.#
// DI....#.#                 #.....#
//   #####.#                 #.###.#
// ZZ......#               QG....#..AS
//   ###.###                 #######
// JO..#.#.#                 #.....#
//   #.#.#.#                 ###.#.#
//   #...#..DI             BU....#..LF
//   #####.#                 #.#####
// YN......#               VT..#....QG
//   #.###.#                 #.###.#
//   #.#...#                 #.....#
//   ###.###    J L     J    #.#.###
//   #.....#    O F     P    #.#...#
//   #.###.#####.#.#####.#####.###.#
//   #...#.#.#...#.....#.....#.#...#
//   #.#####.###.###.#.#.#########.#
//   #...#.#.....#...#.#.#.#.....#.#
//   #.###.#####.###.###.#.#.#######
//   #.#.........#...#.............#
//   #########.###.###.#############
//            B   J   C
//            U   P   P
//
// Here, AA has no direct path to ZZ, but it does connect to AS and CP. By passing through AS, QG, BU, and JO, you can reach ZZ in 58 steps.
//
// In your maze, how many steps does it take to get from the open tile marked AA to the open tile marked ZZ?
//
// Your puzzle answer was 526.
// --- Part Two ---
//
// Strangely, the exit isn't open when you reach it. Then, you remember: the ancient Plutonians were famous for building recursive spaces.
//
// The marked connections in the maze aren't portals: they physically connect to a larger or smaller copy of the maze. Specifically, the labeled tiles around the inside edge actually connect to a smaller copy of the same maze, and the smaller copy's inner labeled tiles connect to yet a smaller copy, and so on.
//
// When you enter the maze, you are at the outermost level; when at the outermost level, only the outer labels AA and ZZ function (as the start and end, respectively); all other outer labeled tiles are effectively walls. At any other level, AA and ZZ count as walls, but the other outer labeled tiles bring you one level outward.
//
// Your goal is to find a path through the maze that brings you back to ZZ at the outermost level of the maze.
//
// In the first example above, the shortest path is now the loop around the right side. If the starting level is 0, then taking the previously-shortest path would pass through BC (to level 1), DE (to level 2), and FG (back to level 1). Because this is not the outermost level, ZZ is a wall, and the only option is to go back around to BC, which would only send you even deeper into the recursive maze.
//
// In the second example above, there is no path that brings you to ZZ at the outermost level.
//
// Here is a more interesting example:
//
//              Z L X W       C
//              Z P Q B       K
//   ###########.#.#.#.#######.###############
//   #...#.......#.#.......#.#.......#.#.#...#
//   ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
//   #.#...#.#.#...#.#.#...#...#...#.#.......#
//   #.###.#######.###.###.#.###.###.#.#######
//   #...#.......#.#...#...#.............#...#
//   #.#########.#######.#.#######.#######.###
//   #...#.#    F       R I       Z    #.#.#.#
//   #.###.#    D       E C       H    #.#.#.#
//   #.#...#                           #...#.#
//   #.###.#                           #.###.#
//   #.#....OA                       WB..#.#..ZH
//   #.###.#                           #.#.#.#
// CJ......#                           #.....#
//   #######                           #######
//   #.#....CK                         #......IC
//   #.###.#                           #.###.#
//   #.....#                           #...#.#
//   ###.###                           #.#.#.#
// XF....#.#                         RF..#.#.#
//   #####.#                           #######
//   #......CJ                       NM..#...#
//   ###.#.#                           #.###.#
// RE....#.#                           #......RF
//   ###.###        X   X       L      #.#.#.#
//   #.....#        F   Q       P      #.#.#.#
//   ###.###########.###.#######.#########.###
//   #.....#...#.....#.......#...#.....#.#...#
//   #####.#.###.#######.#######.###.###.#.#.#
//   #.......#.......#.#.#.#.#...#...#...#.#.#
//   #####.###.#####.#.#.#.#.###.###.#.###.###
//   #.......#.....#.#...#...............#...#
//   #############.#.#.###.###################
//                A O F   N
//                A A D   M
//
// One shortest path through the maze is the following:
//
//     Walk from AA to XF (16 steps)
//     Recurse into level 1 through XF (1 step)
//     Walk from XF to CK (10 steps)
//     Recurse into level 2 through CK (1 step)
//     Walk from CK to ZH (14 steps)
//     Recurse into level 3 through ZH (1 step)
//     Walk from ZH to WB (10 steps)
//     Recurse into level 4 through WB (1 step)
//     Walk from WB to IC (10 steps)
//     Recurse into level 5 through IC (1 step)
//     Walk from IC to RF (10 steps)
//     Recurse into level 6 through RF (1 step)
//     Walk from RF to NM (8 steps)
//     Recurse into level 7 through NM (1 step)
//     Walk from NM to LP (12 steps)
//     Recurse into level 8 through LP (1 step)
//     Walk from LP to FD (24 steps)
//     Recurse into level 9 through FD (1 step)
//     Walk from FD to XQ (8 steps)
//     Recurse into level 10 through XQ (1 step)
//     Walk from XQ to WB (4 steps)
//     Return to level 9 through WB (1 step)
//     Walk from WB to ZH (10 steps)
//     Return to level 8 through ZH (1 step)
//     Walk from ZH to CK (14 steps)
//     Return to level 7 through CK (1 step)
//     Walk from CK to XF (10 steps)
//     Return to level 6 through XF (1 step)
//     Walk from XF to OA (14 steps)
//     Return to level 5 through OA (1 step)
//     Walk from OA to CJ (8 steps)
//     Return to level 4 through CJ (1 step)
//     Walk from CJ to RE (8 steps)
//     Return to level 3 through RE (1 step)
//     Walk from RE to IC (4 steps)
//     Recurse into level 4 through IC (1 step)
//     Walk from IC to RF (10 steps)
//     Recurse into level 5 through RF (1 step)
//     Walk from RF to NM (8 steps)
//     Recurse into level 6 through NM (1 step)
//     Walk from NM to LP (12 steps)
//     Recurse into level 7 through LP (1 step)
//     Walk from LP to FD (24 steps)
//     Recurse into level 8 through FD (1 step)
//     Walk from FD to XQ (8 steps)
//     Recurse into level 9 through XQ (1 step)
//     Walk from XQ to WB (4 steps)
//     Return to level 8 through WB (1 step)
//     Walk from WB to ZH (10 steps)
//     Return to level 7 through ZH (1 step)
//     Walk from ZH to CK (14 steps)
//     Return to level 6 through CK (1 step)
//     Walk from CK to XF (10 steps)
//     Return to level 5 through XF (1 step)
//     Walk from XF to OA (14 steps)
//     Return to level 4 through OA (1 step)
//     Walk from OA to CJ (8 steps)
//     Return to level 3 through CJ (1 step)
//     Walk from CJ to RE (8 steps)
//     Return to level 2 through RE (1 step)
//     Walk from RE to XQ (14 steps)
//     Return to level 1 through XQ (1 step)
//     Walk from XQ to FD (8 steps)
//     Return to level 0 through FD (1 step)
//     Walk from FD to ZZ (18 steps)
//
// This path takes a total of 396 steps to move from AA at the outermost layer to ZZ at the outermost layer.
//
// In your maze, when accounting for recursion, how many steps does it take to get from the open tile marked AA to the open tile marked ZZ, both at the outermost layer?
//
// Your puzzle answer was 6292.

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

use regex::{Captures, Regex};

use crate::geom::{Direction, Vector2};

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2019/day20")?;
    println!("{}", explore(&input, false));
    println!("{}", explore(&input, true));
    Ok(())
}

fn explore(s: &str, is_recursive: bool) -> i32 {
    let grid = s.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (start_pos, end_pos, portals, alignments) = parse_grid(&grid);

    let mut res = None;
    let mut visited = HashSet::new();
    let mut to_process = VecDeque::new();
    visited.insert((start_pos, 0));
    to_process.push_back(((start_pos, 0), 0));
    while !to_process.is_empty() {
        let ((pos, level), depth) = to_process.pop_front().unwrap();
        if pos == end_pos && level == 0 {
            res = Some(depth);
            break;
        }
        for dir in Direction::VALUES.iter() {
            let mut next_pos = pos + dir.dxdy();
            let mut next_level = level;
            let c = grid[next_pos.y as usize][next_pos.x as usize];
            let can_move = match c {
                b'A'..=b'Z' => {
                    if pos == start_pos || pos == end_pos {
                        false
                    } else {
                        if is_recursive {
                            next_level += match alignments[&pos] {
                                Alignment::Outer => -1,
                                Alignment::Inner => 1,
                            }
                        }
                        if next_level < 0 {
                            false
                        } else {
                            next_pos = portals[&pos];
                            true
                        }
                    }
                }
                b'.' => true,
                b'#' => false,
                _ => panic!("unrecognized tile"),
            };
            let next_state = (next_pos, next_level);
            if can_move && !visited.contains(&next_state) {
                to_process.push_back((next_state, depth + 1));
                visited.insert(next_state);
            }
        }
    }
    res.unwrap()
}

fn parse_grid(grid: &Vec<Vec<u8>>)
              -> (Vector2, Vector2,
                  HashMap<Vector2, Vector2>,
                  HashMap<Vector2, Alignment>) {
    lazy_static::lazy_static! {
        static ref RE_LEFT: Regex = Regex::new(r"(?P<label>[A-Z]{2})(?P<tile>\.)").unwrap();
        static ref RE_RIGHT: Regex = Regex::new(r"(?P<tile>\.)(?P<label>[A-Z]{2})").unwrap();
    }

    let mut labels = HashMap::new();

    let mut add_row_labels = |grid: &Vec<Vec<u8>>, is_tranposed: bool| {
        for (y, line) in grid.iter().enumerate() {
            let s = std::str::from_utf8(line).unwrap();
            let mid = s.len() / 2;
            let process_cap =
                |cap: Captures, is_left: bool| -> (String, Vector2, Alignment) {
                    let label = cap.name("label").unwrap().as_str().to_string();
                    let x = cap.name("tile").unwrap().start();
                    let pos =
                        if is_tranposed {
                            Vector2::new(y, x)
                        } else {
                            Vector2::new(x, y)
                        };
                    let alignment =
                        if (x < mid) ^ is_left {
                            Alignment::Inner
                        } else {
                            Alignment::Outer
                        };
                    (label, pos, alignment)
                };

            for (label, pos, alignment) in
            RE_LEFT.captures_iter(s).map(|cap| process_cap(cap, true)).chain(
                RE_RIGHT.captures_iter(s).map(|cap| process_cap(cap, false))) {
                labels.entry(label).or_insert(Vec::new()).push((pos, alignment));
            }
        }
    };
    add_row_labels(&grid, false);
    add_row_labels(&transpose(&grid), true);

    let mut start_pos = None;
    let mut end_pos = None;
    let mut alignments = HashMap::new();
    let mut portals = HashMap::new();

    for (label, pos_aligns) in labels.iter() {
        if label == "AA" {
            assert!(start_pos.is_none());
            assert_eq!(pos_aligns.len(), 1);
            let (pos, align) = pos_aligns[0];
            assert_eq!(align, Alignment::Outer);
            start_pos = Some(pos);
            continue;
        }
        if label == "ZZ" {
            assert!(end_pos.is_none());
            assert_eq!(pos_aligns.len(), 1);
            let (pos, align) = pos_aligns[0];
            assert_eq!(align, Alignment::Outer);
            end_pos = Some(pos);
            continue;
        }
        assert_eq!(pos_aligns.len(), 2);
        let (pos0, align0) = pos_aligns[0];
        let (pos1, align1) = pos_aligns[1];
        alignments.insert(pos0, align0);
        alignments.insert(pos1, align1);
        portals.insert(pos0, pos1);
        portals.insert(pos1, pos0);
    }
    let start_pos = start_pos.expect("AA not found");
    let end_pos = end_pos.expect("ZZ not found");
    (start_pos, end_pos, portals, alignments)
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Alignment {
    Outer,
    Inner,
}

#[allow(dead_code)]
fn pprint(grid: &Vec<Vec<u8>>) {
    for line in grid.iter() {
        let s = std::str::from_utf8(&line).unwrap();
        println!("{}", s);
    }
}

fn transpose(a: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let n = a.len();
    let m = a[0].len();
    let mut res = vec![vec![0u8; n]; m];
    for (y, row) in a.iter().enumerate() {
        for (x, elem) in row.iter().enumerate() {
            res[x][y] = *elem;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
         A         
         A         
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z     
             Z     ";
        assert_eq!(23, explore(s, false));

        let s = r"
                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ";
        assert_eq!(58, explore(s, false));

        let s = r"
             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";
        assert_eq!(396, explore(s, true));
        Ok(())
    }
}

mod geom {
    use std::ops::{Add, AddAssign, Neg, SubAssign};

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
