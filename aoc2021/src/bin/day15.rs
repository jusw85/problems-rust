// https://adventofcode.com/2021/day/15
//
// --- Day 15: Chiton ---
//
// You've almost reached the exit of the cave, but the walls are getting closer together. Your submarine can barely still fit, though; the main problem is that the walls of the cave are covered in chitons, and it would be best not to bump any of them.
//
// The cavern is large, but has a very low ceiling, restricting your motion to two dimensions. The shape of the cavern resembles a square; a quick scan of chiton density produces a map of risk level throughout the cave (your puzzle input). For example:
//
// 1163751742
// 1381373672
// 2136511328
// 3694931569
// 7463417111
// 1319128137
// 1359912421
// 3125421639
// 1293138521
// 2311944581
//
// You start in the top left position, your destination is the bottom right position, and you cannot move diagonally. The number at each position is its risk level; to determine the total risk of an entire path, add up the risk levels of each position you enter (that is, don't count the risk level of your starting position unless you enter it; leaving it adds no risk to your total).
//
// Your goal is to find a path with the lowest total risk. In this example, a path with the lowest total risk is highlighted here:
//
// 1163751742
// 1381373672
// 2136511328
// 3694931569
// 7463417111
// 1319128137
// 1359912421
// 3125421639
// 1293138521
// 2311944581
//
// The total risk of this path is 40 (the starting position is never entered, so its risk is not counted).
//
// What is the lowest total risk of any path from the top left to the bottom right?
//
// Your puzzle answer was 487.
// --- Part Two ---
//
// Now that you know how to find low-risk paths in the cave, you can try to find your way out.
//
// The entire cave is actually five times larger in both dimensions than you thought; the area you originally scanned is just one tile in a 5x5 tile area that forms the full map. Your original map tile repeats to the right and downward; each time the tile repeats to the right or downward, all of its risk levels are 1 higher than the tile immediately up or left of it. However, risk levels above 9 wrap back around to 1. So, if your original map had some position with a risk level of 8, then that same position on each of the 25 total tiles would be as follows:
//
// 8 9 1 2 3
// 9 1 2 3 4
// 1 2 3 4 5
// 2 3 4 5 6
// 3 4 5 6 7
//
// Each single digit above corresponds to the example position with a value of 8 on the top-left tile. Because the full map is actually five times larger in both dimensions, that position appears a total of 25 times, once in each duplicated tile, with the values shown above.
//
// Here is the full five-times-as-large version of the first example above, with the original map in the top left corner highlighted:
//
// 11637517422274862853338597396444961841755517295286
// 13813736722492484783351359589446246169155735727126
// 21365113283247622439435873354154698446526571955763
// 36949315694715142671582625378269373648937148475914
// 74634171118574528222968563933317967414442817852555
// 13191281372421239248353234135946434524615754563572
// 13599124212461123532357223464346833457545794456865
// 31254216394236532741534764385264587549637569865174
// 12931385212314249632342535174345364628545647573965
// 23119445813422155692453326671356443778246755488935
// 22748628533385973964449618417555172952866628316397
// 24924847833513595894462461691557357271266846838237
// 32476224394358733541546984465265719557637682166874
// 47151426715826253782693736489371484759148259586125
// 85745282229685639333179674144428178525553928963666
// 24212392483532341359464345246157545635726865674683
// 24611235323572234643468334575457944568656815567976
// 42365327415347643852645875496375698651748671976285
// 23142496323425351743453646285456475739656758684176
// 34221556924533266713564437782467554889357866599146
// 33859739644496184175551729528666283163977739427418
// 35135958944624616915573572712668468382377957949348
// 43587335415469844652657195576376821668748793277985
// 58262537826937364893714847591482595861259361697236
// 96856393331796741444281785255539289636664139174777
// 35323413594643452461575456357268656746837976785794
// 35722346434683345754579445686568155679767926678187
// 53476438526458754963756986517486719762859782187396
// 34253517434536462854564757396567586841767869795287
// 45332667135644377824675548893578665991468977611257
// 44961841755517295286662831639777394274188841538529
// 46246169155735727126684683823779579493488168151459
// 54698446526571955763768216687487932779859814388196
// 69373648937148475914825958612593616972361472718347
// 17967414442817852555392896366641391747775241285888
// 46434524615754563572686567468379767857948187896815
// 46833457545794456865681556797679266781878137789298
// 64587549637569865174867197628597821873961893298417
// 45364628545647573965675868417678697952878971816398
// 56443778246755488935786659914689776112579188722368
// 55172952866628316397773942741888415385299952649631
// 57357271266846838237795794934881681514599279262561
// 65719557637682166874879327798598143881961925499217
// 71484759148259586125936169723614727183472583829458
// 28178525553928963666413917477752412858886352396999
// 57545635726865674683797678579481878968159298917926
// 57944568656815567976792667818781377892989248891319
// 75698651748671976285978218739618932984172914319528
// 56475739656758684176786979528789718163989182927419
// 67554889357866599146897761125791887223681299833479
//
// Equipped with the full map, you can now find a path from the top left corner to the bottom right corner with the lowest total risk:
//
// 11637517422274862853338597396444961841755517295286
// 13813736722492484783351359589446246169155735727126
// 21365113283247622439435873354154698446526571955763
// 36949315694715142671582625378269373648937148475914
// 74634171118574528222968563933317967414442817852555
// 13191281372421239248353234135946434524615754563572
// 13599124212461123532357223464346833457545794456865
// 31254216394236532741534764385264587549637569865174
// 12931385212314249632342535174345364628545647573965
// 23119445813422155692453326671356443778246755488935
// 22748628533385973964449618417555172952866628316397
// 24924847833513595894462461691557357271266846838237
// 32476224394358733541546984465265719557637682166874
// 47151426715826253782693736489371484759148259586125
// 85745282229685639333179674144428178525553928963666
// 24212392483532341359464345246157545635726865674683
// 24611235323572234643468334575457944568656815567976
// 42365327415347643852645875496375698651748671976285
// 23142496323425351743453646285456475739656758684176
// 34221556924533266713564437782467554889357866599146
// 33859739644496184175551729528666283163977739427418
// 35135958944624616915573572712668468382377957949348
// 43587335415469844652657195576376821668748793277985
// 58262537826937364893714847591482595861259361697236
// 96856393331796741444281785255539289636664139174777
// 35323413594643452461575456357268656746837976785794
// 35722346434683345754579445686568155679767926678187
// 53476438526458754963756986517486719762859782187396
// 34253517434536462854564757396567586841767869795287
// 45332667135644377824675548893578665991468977611257
// 44961841755517295286662831639777394274188841538529
// 46246169155735727126684683823779579493488168151459
// 54698446526571955763768216687487932779859814388196
// 69373648937148475914825958612593616972361472718347
// 17967414442817852555392896366641391747775241285888
// 46434524615754563572686567468379767857948187896815
// 46833457545794456865681556797679266781878137789298
// 64587549637569865174867197628597821873961893298417
// 45364628545647573965675868417678697952878971816398
// 56443778246755488935786659914689776112579188722368
// 55172952866628316397773942741888415385299952649631
// 57357271266846838237795794934881681514599279262561
// 65719557637682166874879327798598143881961925499217
// 71484759148259586125936169723614727183472583829458
// 28178525553928963666413917477752412858886352396999
// 57545635726865674683797678579481878968159298917926
// 57944568656815567976792667818781377892989248891319
// 75698651748671976285978218739618932984172914319528
// 56475739656758684176786979528789718163989182927419
// 67554889357866599146897761125791887223681299833479
//
// The total risk of this path is 315 (the starting position is still never entered, so its risk is not counted).
//
// Using the full map, what is the lowest total risk of any path from the top left to the bottom right?
//
// Your puzzle answer was 2821.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

use anyhow::Result;

use crate::chargrid::Grid;
use crate::geom::{Direction, Vector2};

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2021/day15")?;
    let grid = parse(&input);
    println!("{:?}", solve(&grid, false));
    println!("{:?}", solve(&grid, true));
    Ok(())
}

fn parse(s: &str) -> Grid {
    let mut grid = Grid::parse(s).unwrap();
    for (_, _, c) in grid.iter_mut() {
        *c -= b'0';
    }
    grid
}

fn solve(grid: &Grid, big_grid: bool) -> u32 {
    let mut visited = HashSet::new();
    let mut min_heap = BinaryHeap::new();
    min_heap.push((Reverse(0), Vector2::new(0, 0)));
    let target = match big_grid {
        true => Vector2::new((5 * grid.num_cols) - 1, (5 * grid.num_rows) - 1),
        false => Vector2::new(grid.num_cols - 1, grid.num_rows - 1),
    };

    loop {
        let (Reverse(cost), pos) = min_heap.pop().unwrap();
        if visited.contains(&pos) {
            continue;
        }
        if pos == target {
            break cost;
        }
        visited.insert(pos);
        for np in Direction::VALUES_4D.iter().map(|dir| pos + dir.dxdy())
        {
            // @formatter:off
            if !visited.contains(&np) && np.x >= 0 && np.y >= 0 &&
                ((!big_grid &&
                    (np.x as usize) < grid.num_cols &&
                    (np.y as usize) < grid.num_rows) ||
                (big_grid &&
                    (np.x as usize) < (5 * grid.num_cols) &&
                    (np.y as usize) < (5 * grid.num_rows)))
            {
                min_heap.push((Reverse(cost + get_cost(&grid, np, big_grid)), np));
            }
            // @formatter:on
        }
    }
}

fn get_cost(grid: &Grid, p: Vector2, big_grid: bool) -> u32 {
    if !big_grid {
        grid[p] as u32
    } else {
        let (x, y) = (p.x as usize, p.y as usize);
        (((grid[y % grid.num_rows][x % grid.num_cols]) as u32
            + (x / grid.num_cols) as u32
            + (y / grid.num_rows) as u32 - 1) % 9) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
        ";
        let grid = parse(&s);
        assert_eq!(40, solve(&grid, false));
        assert_eq!(315, solve(&grid, true));
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

    #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Default, Debug)]
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
