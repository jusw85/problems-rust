// https://adventofcode.com/2019/day/24
//
// --- Day 24: Planet of Discord ---
//
// You land on Eris, your last stop before reaching Santa. As soon as you do, your sensors start picking up strange life forms moving around: Eris is infested with bugs! With an over 24-hour roundtrip for messages between you and Earth, you'll have to deal with this problem on your own.
//
// Eris isn't a very large place; a scan of the entire area fits into a 5x5 grid (your puzzle input). The scan shows bugs (#) and empty spaces (.).
//
// Each minute, The bugs live and die based on the number of bugs in the four adjacent tiles:
//
//     A bug dies (becoming an empty space) unless there is exactly one bug adjacent to it.
//     An empty space becomes infested with a bug if exactly one or two bugs are adjacent to it.
//
// Otherwise, a bug or empty space remains the same. (Tiles on the edges of the grid have fewer than four adjacent tiles; the missing tiles count as empty space.) This process happens in every location simultaneously; that is, within the same minute, the number of adjacent bugs is counted for every tile first, and then the tiles are updated.
//
// Here are the first few minutes of an example scenario:
//
// Initial state:
// ....#
// #..#.
// #..##
// ..#..
// #....
//
// After 1 minute:
// #..#.
// ####.
// ###.#
// ##.##
// .##..
//
// After 2 minutes:
// #####
// ....#
// ....#
// ...#.
// #.###
//
// After 3 minutes:
// #....
// ####.
// ...##
// #.##.
// .##.#
//
// After 4 minutes:
// ####.
// ....#
// ##..#
// .....
// ##...
//
// To understand the nature of the bugs, watch for the first time a layout of bugs and empty spaces matches any previous layout. In the example above, the first layout to appear twice is:
//
// .....
// .....
// .....
// #....
// .#...
//
// To calculate the biodiversity rating for this layout, consider each tile left-to-right in the top row, then left-to-right in the second row, and so on. Each of these tiles is worth biodiversity points equal to increasing powers of two: 1, 2, 4, 8, 16, 32, and so on. Add up the biodiversity points for tiles with bugs; in this example, the 16th tile (32768 points) and 22nd tile (2097152 points) have bugs, a total biodiversity rating of 2129920.
//
// What is the biodiversity rating for the first layout that appears twice?
//
// Your puzzle answer was 32513278.
// --- Part Two ---
//
// After careful analysis, one thing is certain: you have no idea where all these bugs are coming from.
//
// Then, you remember: Eris is an old Plutonian settlement! Clearly, the bugs are coming from recursively-folded space.
//
// This 5x5 grid is only one level in an infinite number of recursion levels. The tile in the middle of the grid is actually another 5x5 grid, the grid in your scan is contained as the middle tile of a larger 5x5 grid, and so on. Two levels of grids look like this:
//
//      |     |         |     |
//      |     |         |     |
//      |     |         |     |
// -----+-----+---------+-----+-----
//      |     |         |     |
//      |     |         |     |
//      |     |         |     |
// -----+-----+---------+-----+-----
//      |     | | | | | |     |
//      |     |-+-+-+-+-|     |
//      |     | | | | | |     |
//      |     |-+-+-+-+-|     |
//      |     | | |?| | |     |
//      |     |-+-+-+-+-|     |
//      |     | | | | | |     |
//      |     |-+-+-+-+-|     |
//      |     | | | | | |     |
// -----+-----+---------+-----+-----
//      |     |         |     |
//      |     |         |     |
//      |     |         |     |
// -----+-----+---------+-----+-----
//      |     |         |     |
//      |     |         |     |
//      |     |         |     |
//
// (To save space, some of the tiles are not drawn to scale.) Remember, this is only a small part of the infinitely recursive grid; there is a 5x5 grid that contains this diagram, and a 5x5 grid that contains that one, and so on. Also, the ? in the diagram contains another 5x5 grid, which itself contains another 5x5 grid, and so on.
//
// The scan you took (your puzzle input) shows where the bugs are on a single level of this structure. The middle tile of your scan is empty to accommodate the recursive grids within it. Initially, no other levels contain bugs.
//
// Tiles still count as adjacent if they are directly up, down, left, or right of a given tile. Some tiles have adjacent tiles at a recursion level above or below its own level. For example:
//
//      |     |         |     |
//   1  |  2  |    3    |  4  |  5
//      |     |         |     |
// -----+-----+---------+-----+-----
//      |     |         |     |
//   6  |  7  |    8    |  9  |  10
//      |     |         |     |
// -----+-----+---------+-----+-----
//      |     |A|B|C|D|E|     |
//      |     |-+-+-+-+-|     |
//      |     |F|G|H|I|J|     |
//      |     |-+-+-+-+-|     |
//  11  | 12  |K|L|?|N|O|  14 |  15
//      |     |-+-+-+-+-|     |
//      |     |P|Q|R|S|T|     |
//      |     |-+-+-+-+-|     |
//      |     |U|V|W|X|Y|     |
// -----+-----+---------+-----+-----
//      |     |         |     |
//  16  | 17  |    18   |  19 |  20
//      |     |         |     |
// -----+-----+---------+-----+-----
//      |     |         |     |
//  21  | 22  |    23   |  24 |  25
//      |     |         |     |
//
//     Tile 19 has four adjacent tiles: 14, 18, 20, and 24.
//     Tile G has four adjacent tiles: B, F, H, and L.
//     Tile D has four adjacent tiles: 8, C, E, and I.
//     Tile E has four adjacent tiles: 8, D, 14, and J.
//     Tile 14 has eight adjacent tiles: 9, E, J, O, T, Y, 15, and 19.
//     Tile N has eight adjacent tiles: I, O, S, and five tiles within the sub-grid marked ?.
//
// The rules about bugs living and dying are the same as before.
//
// For example, consider the same initial state as above:
//
// ....#
// #..#.
// #.?##
// ..#..
// #....
//
// The center tile is drawn as ? to indicate the next recursive grid. Call this level 0; the grid within this one is level 1, and the grid that contains this one is level -1. Then, after ten minutes, the grid at each level would look like this:
//
// Depth -5:
// ..#..
// .#.#.
// ..?.#
// .#.#.
// ..#..
//
// Depth -4:
// ...#.
// ...##
// ..?..
// ...##
// ...#.
//
// Depth -3:
// #.#..
// .#...
// ..?..
// .#...
// #.#..
//
// Depth -2:
// .#.##
// ....#
// ..?.#
// ...##
// .###.
//
// Depth -1:
// #..##
// ...##
// ..?..
// ...#.
// .####
//
// Depth 0:
// .#...
// .#.##
// .#?..
// .....
// .....
//
// Depth 1:
// .##..
// #..##
// ..?.#
// ##.##
// #####
//
// Depth 2:
// ###..
// ##.#.
// #.?..
// .#.##
// #.#..
//
// Depth 3:
// ..###
// .....
// #.?..
// #....
// #...#
//
// Depth 4:
// .###.
// #..#.
// #.?..
// ##.#.
// .....
//
// Depth 5:
// ####.
// #..#.
// #.?#.
// ####.
// .....
//
// In this example, after 10 minutes, a total of 99 bugs are present.
//
// Starting with your scan, how many bugs are present after 200 minutes?
//
// Your puzzle answer was 1912.

use std::collections::{HashSet, VecDeque};
use std::convert::{TryFrom, TryInto};
use std::fs;

use crate::geom::{Direction, Vector2};

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2019/day24")?;
    part1(&input);
    part2(&input);
    Ok(())
}

fn part1(s: &str) {
    let mut grid = parse_grid(s);
    let mut seen = HashSet::new();
    while seen.insert(grid) {
        grid = tick(&grid);
    }
    let res = biodiversity(&grid);
    println!("{}", res);
}

fn part2(s: &str) {
    let grid = parse_grid(s);
    let mut grids = VecDeque::new();
    grids.push_back(grid);

    for _ in 0..200 {
        grids = tick_recursive(&grids);
    }

    let total_bugs = grids.iter()
        .map(|grid| grid.iter()
            .flat_map(|line| line.iter())
            .filter(|&&c| c == b'#')
            .count()
        ).sum::<usize>();
    println!("{}", total_bugs);
}

fn tick_recursive(grids: &VecDeque<[[u8; 5]; 5]>) -> VecDeque<[[u8; 5]; 5]> {
    let tick_grid_recursive = |level: i32, grid: &[[u8; 5]; 5]| -> [[u8; 5]; 5] {
        let count_adj_tile_recursive = |y: usize, x: usize| -> usize {
            let mut count = count_adj_tile(&grid, y, x);
            if level > 0 {
                let prev_grid = grids[(level - 1) as usize];
                if y == 0 {
                    if prev_grid[1][2] == b'#' { count += 1 }
                } else if y == 4 {
                    if prev_grid[3][2] == b'#' { count += 1 }
                }
                if x == 0 {
                    if prev_grid[2][1] == b'#' { count += 1 }
                } else if x == 4 {
                    if prev_grid[2][3] == b'#' { count += 1 }
                }
            }
            if level < (grids.len() - 1) as i32 {
                let next_grid = grids[(level + 1) as usize];
                if (y, x) == (2, 1) {
                    count += count_adj_wall(&next_grid, Direction::W);
                } else if (y, x) == (2, 3) {
                    count += count_adj_wall(&next_grid, Direction::E);
                } else if (y, x) == (1, 2) {
                    count += count_adj_wall(&next_grid, Direction::N);
                } else if (y, x) == (3, 2) {
                    count += count_adj_wall(&next_grid, Direction::S);
                }
            }
            count
        };

        let mut new_grid = [[0u8; 5]; 5];
        for (y, row) in grid.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if (y, x) == (2, 2) {
                    new_grid[y][x] = b'.';
                    continue;
                }
                let adj = count_adj_tile_recursive(y, x);
                new_grid[y][x] =
                    if c == b'#' {
                        if adj == 1 { b'#' } else { b'.' }
                    } else {
                        assert_eq!(c, b'.');
                        if adj == 1 || adj == 2 { b'#' } else { b'.' }
                    };
            }
        }
        new_grid
    };

    let mut res = VecDeque::new();
    for (level, grid) in grids.iter().enumerate() {
        let new_grid = tick_grid_recursive(level as i32, grid);
        res.push_back(new_grid);
    }
    let first_grid = grids.front().unwrap();
    if count_adj_wall(first_grid, Direction::N) > 0 ||
        count_adj_wall(first_grid, Direction::E) > 0 ||
        count_adj_wall(first_grid, Direction::S) > 0 ||
        count_adj_wall(first_grid, Direction::W) > 0 {
        let new_grid = tick_grid_recursive(-1, &[[b'.'; 5]; 5]);
        res.push_front(new_grid);
    }
    let last_grid = grids.back().unwrap();
    if last_grid[1][2] == b'#' ||
        last_grid[2][1] == b'#' ||
        last_grid[2][3] == b'#' ||
        last_grid[3][2] == b'#' {
        let new_grid = tick_grid_recursive(grids.len() as i32, &[[b'.'; 5]; 5]);
        res.push_back(new_grid);
    }
    res
}

fn count_adj_tile(grid: &[[u8; 5]; 5], y: usize, x: usize) -> usize {
    let dirs = match (y, x) {
        (0, 0) => [Direction::S, Direction::E].iter(),
        (0, 4) => [Direction::W, Direction::S].iter(),
        (4, 4) => [Direction::N, Direction::W].iter(),
        (4, 0) => [Direction::E, Direction::N].iter(),
        (0, _) => [Direction::W, Direction::S, Direction::E].iter(),
        (_, 4) => [Direction::N, Direction::W, Direction::S].iter(),
        (4, _) => [Direction::E, Direction::N, Direction::W].iter(),
        (_, 0) => [Direction::S, Direction::E, Direction::N].iter(),
        _ => Direction::VALUES.iter(),
    };
    let pos = Vector2::new(x, y);
    dirs.map(|dir| pos + dir.dxdy())
        .filter(|new_pos| grid[new_pos.y as usize][new_pos.x as usize] == b'#')
        .count()
}

fn count_adj_wall(grid: &[[u8; 5]; 5], dir: Direction) -> usize {
    match dir {
        Direction::N => grid[0].iter().filter(|&&c| c == b'#').count(),
        Direction::E => grid.iter().map(|l| l[4]).filter(|&c| c == b'#').count(),
        Direction::S => grid[4].iter().filter(|&&c| c == b'#').count(),
        Direction::W => grid.iter().map(|l| l[0]).filter(|&c| c == b'#').count(),
    }
}

fn parse_grid(s: &str) -> [[u8; 5]; 5] {
    s.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| <[u8; 5]>::try_from(line.as_bytes()).unwrap())
        .collect::<Vec<_>>()
        .try_into().unwrap()
}

fn biodiversity(grid: &[[u8; 5]; 5]) -> u32 {
    grid.iter()
        .flat_map(|line| line.iter())
        .enumerate()
        .filter(|(_, &c)| c == b'#')
        .map(|(i, _)| 2_u32.pow(i as u32))
        .sum::<u32>()
}

fn tick(grid: &[[u8; 5]; 5]) -> [[u8; 5]; 5] {
    let mut new_grid = [[0u8; 5]; 5];
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            let adj = count_adj_tile(&grid, y, x);
            new_grid[y][x] =
                if c == b'#' {
                    if adj == 1 { b'#' } else { b'.' }
                } else {
                    assert_eq!(c, b'.');
                    if adj == 1 || adj == 2 { b'#' } else { b'.' }
                };
        }
    }
    new_grid
}

#[allow(dead_code)]
fn pprint(grid: &[[u8; 5]; 5]) {
    for line in grid {
        println!("{}", std::str::from_utf8(line).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        .....
        .....
        .....
        #....
        .#...
        ";
        assert_eq!(2129920, biodiversity(&parse_grid(s)));

        let s = r"
        ....#
        #..#.
        #..##
        ..#..
        #....
        ";
        let g = parse_grid(s);
        assert_eq!(1, count_adj_wall(&g, Direction::N));
        assert_eq!(2, count_adj_wall(&g, Direction::E));
        assert_eq!(1, count_adj_wall(&g, Direction::S));
        assert_eq!(3, count_adj_wall(&g, Direction::W));
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
