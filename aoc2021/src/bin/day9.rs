// https://adventofcode.com/2021/day/9
//
// --- Day 9: Smoke Basin ---
//
// These caves seem to be lava tubes. Parts are even still volcanically active; small hydrothermal vents release smoke into the caves that slowly settles like rain.
//
// If you can model how the smoke flows through the caves, you might be able to avoid it and be that much safer. The submarine generates a heightmap of the floor of the nearby caves for you (your puzzle input).
//
// Smoke flows to the lowest point of the area it's in. For example, consider the following heightmap:
//
// 2199943210
// 3987894921
// 9856789892
// 8767896789
// 9899965678
//
// Each number corresponds to the height of a particular location, where 9 is the highest and 0 is the lowest a location can be.
//
// Your first goal is to find the low points - the locations that are lower than any of its adjacent locations. Most locations have four adjacent locations (up, down, left, and right); locations on the edge or corner of the map have three or two adjacent locations, respectively. (Diagonal locations do not count as adjacent.)
//
// In the above example, there are four low points, all highlighted: two are in the first row (a 1 and a 0), one is in the third row (a 5), and one is in the bottom row (also a 5). All other locations on the heightmap have some lower adjacent location, and so are not low points.
//
// The risk level of a low point is 1 plus its height. In the above example, the risk levels of the low points are 2, 1, 6, and 6. The sum of the risk levels of all low points in the heightmap is therefore 15.
//
// Find all of the low points on your heightmap. What is the sum of the risk levels of all low points on your heightmap?
//
// Your puzzle answer was 489.
// --- Part Two ---
//
// Next, you need to find the largest basins so you know what areas are most important to avoid.
//
// A basin is all locations that eventually flow downward to a single low point. Therefore, every low point has a basin, although some basins are very small. Locations of height 9 do not count as being in any basin, and all other locations will always be part of exactly one basin.
//
// The size of a basin is the number of locations within the basin, including the low point. The example above has four basins.
//
// The top-left basin, size 3:
//
// 2199943210
// 3987894921
// 9856789892
// 8767896789
// 9899965678
//
// The top-right basin, size 9:
//
// 2199943210
// 3987894921
// 9856789892
// 8767896789
// 9899965678
//
// The middle basin, size 14:
//
// 2199943210
// 3987894921
// 9856789892
// 8767896789
// 9899965678
//
// The bottom-right basin, size 9:
//
// 2199943210
// 3987894921
// 9856789892
// 8767896789
// 9899965678
//
// Find the three largest basins and multiply their sizes together. In the above example, this is 9 * 14 * 9 = 1134.
//
// What do you get if you multiply together the sizes of the three largest basins?
//
// Your puzzle answer was 1056330.

use std::collections::HashSet;
use std::fs;

use anyhow::Result;
use itertools::Itertools;

use crate::chargrid::Grid;
use crate::geom::{Direction, Vector2};

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2021/day9")?;
    let grid = Grid::parse(&input)?;
    let low_points = find_low_points(&grid);
    println!("{:?}", compute_risk(&low_points));

    let sizes = compute_basin_sizes(&grid, &low_points);
    println!("{:?}", product_biggest_3(&sizes));
    Ok(())
}

fn find_low_points(grid: &Grid) -> Vec<(Vector2, u8)> {
    grid.iter().filter_map(|(y, x, &val)| {
        let pos = Vector2::new(x, y);
        Direction::VALUES_4D.iter().all(|dir| {
            let np = pos + dir.dxdy();
            !grid.contains_point(&np) || grid[np] > val
        }).then(|| (pos, val))
    }).collect_vec()
}

fn compute_risk(low_points: &Vec<(Vector2, u8)>) -> u32 {
    low_points.iter()
        .map(|&(_, val)| (val - b'0' + 1) as u32)
        .sum()
}

fn compute_basin_sizes(grid: &Grid, low_points: &Vec<(Vector2, u8)>) -> Vec<usize> {
    low_points.iter().map(|&(initial_point, _)| {
        let mut q = vec![initial_point];
        let mut visited = HashSet::new();

        while !q.is_empty() {
            let p = q.pop().unwrap();
            visited.insert(p);

            for dir in Direction::VALUES_4D.iter() {
                let np = p + dir.dxdy();

                if grid.contains_point(&np)
                    && !visited.contains(&np)
                    && grid[np] != b'9'
                {
                    q.push(np);
                }
            }
        }
        visited.len()
    }).collect_vec()
}

fn product_biggest_3(sizes: &Vec<usize>) -> u32 {
    let mut sizes = sizes.clone();
    sizes.sort_unstable();
    sizes[sizes.len() - 3..].iter()
        .map(|&v| v as u32)
        .product()
}

#[cfg(test)]
mod tests {
    use crate::chargrid::Grid;

    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
        ";
        let grid = Grid::parse(&s)?;
        let low_points = find_low_points(&grid);
        assert_eq!(15, compute_risk(&low_points));

        let sizes = compute_basin_sizes(&grid, &low_points);
        assert_eq!(1134, product_biggest_3(&sizes));
        Ok(())
    }
}

mod chargrid {
    use std::fmt::{Display, Formatter};
    use std::fmt;
    use std::ops::{Index, IndexMut};

    use anyhow::Result;
    use itertools::Itertools;

    use aoc2021::{Enumerate2D, TrimEmpty};

    use crate::geom::Vector2;

    #[derive(Clone, Hash, Eq, PartialEq, Debug)]
    pub struct Grid {
        grid: Vec<Vec<u8>>,
        pub num_rows: usize,
        pub num_cols: usize,
    }

    #[allow(dead_code)]
    impl Grid {
        pub fn new(grid: Vec<Vec<u8>>) -> Grid {
            let num_rows = grid.len();
            let num_cols = grid[0].len();
            Grid { grid, num_rows, num_cols }
        }

        pub fn parse(s: &str) -> Result<Grid> {
            let grid = s.lines()
                .trim_empty()
                .map(|line| line.as_bytes().to_vec())
                .collect_vec();
            if grid.is_empty() {
                anyhow::bail!("empty grid");
            }
            let num_rows = grid.len();
            let num_cols = grid[0].len();

            if grid.iter().any(|row| row.len() != num_cols) {
                anyhow::bail!("jagged grid");
            }
            Ok(Grid { grid, num_rows, num_cols })
        }

        pub fn contains_point(&self, point: &Vector2) -> bool {
            point.x >= 0 && point.y >= 0
                && (point.x as usize) < self.num_cols
                && (point.y as usize) < self.num_rows
        }

        pub fn iter(&self) -> impl Iterator<Item=(usize, usize, &u8)> {
            self.grid.iter().enumerate_2d()
        }

        pub fn iter_row(&self) -> impl Iterator<Item=&Vec<u8>> {
            self.grid.iter()
        }

        pub fn flip_x(&self) -> Grid {
            let (num_rows, num_cols) = (self.num_rows, self.num_cols);
            let mut grid = vec![vec![b'.'; num_cols]; num_rows];
            for (y, x, c) in self.iter() {
                grid[y][num_cols - 1 - x] = *c;
            }
            Grid { grid, num_rows, num_cols }
        }

        pub fn rotate_cw(&self) -> Grid {
            let (num_rows, num_cols) = (self.num_rows, self.num_cols);
            let mut grid = vec![vec![b'.'; num_rows]; num_cols];
            for (y, x, c) in self.iter() {
                grid[x][num_rows - 1 - y] = *c;
            }
            Grid { grid, num_cols: num_rows, num_rows: num_cols }
        }

        pub fn variations(&self) -> VariationIter {
            VariationIter {
                initial: self,
                num_rotations: 0,
                is_flipped: false,
                next: Some(self.clone()),
            }
        }
    }

    pub struct VariationIter<'a> {
        initial: &'a Grid,
        num_rotations: usize,
        is_flipped: bool,
        next: Option<Grid>,
    }

    impl<'a> Iterator for VariationIter<'a> {
        type Item = Grid;

        fn next(&mut self) -> Option<Self::Item> {
            let item = self.next.take()?;

            self.next = if self.num_rotations < 3 {
                self.num_rotations += 1;
                Some(item.rotate_cw())
            } else if !self.is_flipped {
                self.is_flipped = true;
                self.num_rotations = 0;
                Some(self.initial.flip_x())
            } else {
                None
            };
            Some(item)
        }
    }

    impl Display for Grid {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            for line in self.grid.iter() {
                let s = std::str::from_utf8(&line).unwrap();
                writeln!(f, "{}", s)?;
            }
            Ok(())
        }
    }

    impl Index<Vector2> for Grid {
        type Output = u8;

        fn index(&self, index: Vector2) -> &Self::Output {
            let y = index.y as usize;
            let x = index.x as usize;
            &self.grid[y][x]
        }
    }

    impl IndexMut<Vector2> for Grid {
        fn index_mut(&mut self, index: Vector2) -> &mut Self::Output {
            let y = index.y as usize;
            let x = index.x as usize;
            &mut self.grid[y][x]
        }
    }

    impl Index<usize> for Grid {
        type Output = Vec<u8>;

        fn index(&self, index: usize) -> &Self::Output {
            &self.grid[index]
        }
    }

    impl IndexMut<usize> for Grid {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.grid[index]
        }
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
