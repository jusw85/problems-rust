// https://adventofcode.com/2021/day/11
//
// --- Day 11: Dumbo Octopus ---
//
// You enter a large cavern full of rare bioluminescent dumbo octopuses! They seem to not like the Christmas lights on your submarine, so you turn them off for now.
//
// There are 100 octopuses arranged neatly in a 10 by 10 grid. Each octopus slowly gains energy over time and flashes brightly for a moment when its energy is full. Although your lights are off, maybe you could navigate through the cave without disturbing the octopuses if you could predict when the flashes of light will happen.
//
// Each octopus has an energy level - your submarine can remotely measure the energy level of each octopus (your puzzle input). For example:
//
// 5483143223
// 2745854711
// 5264556173
// 6141336146
// 6357385478
// 4167524645
// 2176841721
// 6882881134
// 4846848554
// 5283751526
//
// The energy level of each octopus is a value between 0 and 9. Here, the top-left octopus has an energy level of 5, the bottom-right one has an energy level of 6, and so on.
//
// You can model the energy levels and flashes of light in steps. During a single step, the following occurs:
//
//     First, the energy level of each octopus increases by 1.
//     Then, any octopus with an energy level greater than 9 flashes. This increases the energy level of all adjacent octopuses by 1, including octopuses that are diagonally adjacent. If this causes an octopus to have an energy level greater than 9, it also flashes. This process continues as long as new octopuses keep having their energy level increased beyond 9. (An octopus can only flash at most once per step.)
//     Finally, any octopus that flashed during this step has its energy level set to 0, as it used all of its energy to flash.
//
// Adjacent flashes can cause an octopus to flash on a step even if it begins that step with very little energy. Consider the middle octopus with 1 energy in this situation:
//
// Before any steps:
// 11111
// 19991
// 19191
// 19991
// 11111
//
// After step 1:
// 34543
// 40004
// 50005
// 40004
// 34543
//
// After step 2:
// 45654
// 51115
// 61116
// 51115
// 45654
//
// An octopus is highlighted when it flashed during the given step.
//
// Here is how the larger example above progresses:
//
// Before any steps:
// 5483143223
// 2745854711
// 5264556173
// 6141336146
// 6357385478
// 4167524645
// 2176841721
// 6882881134
// 4846848554
// 5283751526
//
// After step 1:
// 6594254334
// 3856965822
// 6375667284
// 7252447257
// 7468496589
// 5278635756
// 3287952832
// 7993992245
// 5957959665
// 6394862637
//
// After step 2:
// 8807476555
// 5089087054
// 8597889608
// 8485769600
// 8700908800
// 6600088989
// 6800005943
// 0000007456
// 9000000876
// 8700006848
//
// After step 3:
// 0050900866
// 8500800575
// 9900000039
// 9700000041
// 9935080063
// 7712300000
// 7911250009
// 2211130000
// 0421125000
// 0021119000
//
// After step 4:
// 2263031977
// 0923031697
// 0032221150
// 0041111163
// 0076191174
// 0053411122
// 0042361120
// 5532241122
// 1532247211
// 1132230211
//
// After step 5:
// 4484144000
// 2044144000
// 2253333493
// 1152333274
// 1187303285
// 1164633233
// 1153472231
// 6643352233
// 2643358322
// 2243341322
//
// After step 6:
// 5595255111
// 3155255222
// 3364444605
// 2263444496
// 2298414396
// 2275744344
// 2264583342
// 7754463344
// 3754469433
// 3354452433
//
// After step 7:
// 6707366222
// 4377366333
// 4475555827
// 3496655709
// 3500625609
// 3509955566
// 3486694453
// 8865585555
// 4865580644
// 4465574644
//
// After step 8:
// 7818477333
// 5488477444
// 5697666949
// 4608766830
// 4734946730
// 4740097688
// 6900007564
// 0000009666
// 8000004755
// 6800007755
//
// After step 9:
// 9060000644
// 7800000976
// 6900000080
// 5840000082
// 5858000093
// 6962400000
// 8021250009
// 2221130009
// 9111128097
// 7911119976
//
// After step 10:
// 0481112976
// 0031112009
// 0041112504
// 0081111406
// 0099111306
// 0093511233
// 0442361130
// 5532252350
// 0532250600
// 0032240000
//
// After step 10, there have been a total of 204 flashes. Fast forwarding, here is the same configuration every 10 steps:
//
// After step 20:
// 3936556452
// 5686556806
// 4496555690
// 4448655580
// 4456865570
// 5680086577
// 7000009896
// 0000000344
// 6000000364
// 4600009543
//
// After step 30:
// 0643334118
// 4253334611
// 3374333458
// 2225333337
// 2229333338
// 2276733333
// 2754574565
// 5544458511
// 9444447111
// 7944446119
//
// After step 40:
// 6211111981
// 0421111119
// 0042111115
// 0003111115
// 0003111116
// 0065611111
// 0532351111
// 3322234597
// 2222222976
// 2222222762
//
// After step 50:
// 9655556447
// 4865556805
// 4486555690
// 4458655580
// 4574865570
// 5700086566
// 6000009887
// 8000000533
// 6800000633
// 5680000538
//
// After step 60:
// 2533334200
// 2743334640
// 2264333458
// 2225333337
// 2225333338
// 2287833333
// 3854573455
// 1854458611
// 1175447111
// 1115446111
//
// After step 70:
// 8211111164
// 0421111166
// 0042111114
// 0004211115
// 0000211116
// 0065611111
// 0532351111
// 7322235117
// 5722223475
// 4572222754
//
// After step 80:
// 1755555697
// 5965555609
// 4486555680
// 4458655580
// 4570865570
// 5700086566
// 7000008666
// 0000000990
// 0000000800
// 0000000000
//
// After step 90:
// 7433333522
// 2643333522
// 2264333458
// 2226433337
// 2222433338
// 2287833333
// 2854573333
// 4854458333
// 3387779333
// 3333333333
//
// After step 100:
// 0397666866
// 0749766918
// 0053976933
// 0004297822
// 0004229892
// 0053222877
// 0532222966
// 9322228966
// 7922286866
// 6789998766
//
// After 100 steps, there have been a total of 1656 flashes.
//
// Given the starting energy levels of the dumbo octopuses in your cavern, simulate 100 steps. How many total flashes are there after 100 steps?
//
// Your puzzle answer was 1625.
// --- Part Two ---
//
// It seems like the individual flashes aren't bright enough to navigate. However, you might have a better option: the flashes seem to be synchronizing!
//
// In the example above, the first time all octopuses flash simultaneously is step 195:
//
// After step 193:
// 5877777777
// 8877777777
// 7777777777
// 7777777777
// 7777777777
// 7777777777
// 7777777777
// 7777777777
// 7777777777
// 7777777777
//
// After step 194:
// 6988888888
// 9988888888
// 8888888888
// 8888888888
// 8888888888
// 8888888888
// 8888888888
// 8888888888
// 8888888888
// 8888888888
//
// After step 195:
// 0000000000
// 0000000000
// 0000000000
// 0000000000
// 0000000000
// 0000000000
// 0000000000
// 0000000000
// 0000000000
// 0000000000
//
// If you can calculate the exact moments when the octopuses will all flash simultaneously, you should be able to navigate through the cavern. What is the first step during which all octopuses flash?
//
// Your puzzle answer was 244.

use std::collections::HashSet;
use std::fs;

use anyhow::Result;

use crate::chargrid::Grid;
use crate::geom::{Direction, Vector2};

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2021/day11")?;
    let mut grid = Grid::parse(&input).unwrap();
    println!("{:?}", part1(&mut grid.clone(), 100));
    println!("{:?}", part2(&mut grid));
    Ok(())
}

fn part1(grid: &mut Grid, steps: u32) -> u32 {
    (0..steps).map(|_| step(grid)).sum::<u32>()
}

fn part2(grid: &mut Grid) -> u32 {
    let grid_size = (grid.num_cols * grid.num_rows) as u32;
    (1..).find(|_| step(grid) == grid_size).unwrap()
}

fn step(grid: &mut Grid) -> u32 {
    let mut q = vec![];
    let mut flashed = HashSet::new();

    fn increase_energy(p: Vector2,
                       c: &mut u8,
                       q: &mut Vec<Vector2>,
                       flashed: &mut HashSet<Vector2>)
    {
        *c += 1;
        if *c > b'9' {
            *c = b'0';
            q.push(p);
            flashed.insert(p);
        }
    }

    for (y, x, c) in grid.iter_mut() {
        increase_energy(Vector2::new(x, y), c, &mut q, &mut flashed);
    }

    while !q.is_empty() {
        let p = q.pop().unwrap();

        for dir in Direction::VALUES_8D.iter() {
            let np = p + dir.dxdy();

            if grid.contains_point(&np)
                && !flashed.contains(&np)
            {
                increase_energy(np, &mut grid[np], &mut q, &mut flashed);
            }
        }
    }
    flashed.len() as u32
}

#[cfg(test)]
mod tests {
    use crate::chargrid::Grid;

    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
        ";
        let mut grid = Grid::parse(&s).unwrap();
        assert_eq!(1656, part1(&mut grid.clone(), 100));
        assert_eq!(195, part2(&mut grid));
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

        pub fn iter_mut(&mut self) -> impl Iterator<Item=(usize, usize, &mut u8)> {
            self.grid.iter_mut().enumerate_2d()
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
