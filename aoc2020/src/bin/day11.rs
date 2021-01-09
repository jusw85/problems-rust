// https://adventofcode.com/2020/day/11
//
// --- Day 11: Seating System ---
//
// Your plane lands with plenty of time to spare. The final leg of your journey is a ferry that goes directly to the tropical island where you can finally start your vacation. As you reach the waiting area to board the ferry, you realize you're so early, nobody else has even arrived yet!
//
// By modeling the process people use to choose (or abandon) their seat in the waiting area, you're pretty sure you can predict the best place to sit. You make a quick map of the seat layout (your puzzle input).
//
// The seat layout fits neatly on a grid. Each position is either floor (.), an empty seat (L), or an occupied seat (#). For example, the initial seat layout might look like this:
//
// L.LL.LL.LL
// LLLLLLL.LL
// L.L.L..L..
// LLLL.LL.LL
// L.LL.LL.LL
// L.LLLLL.LL
// ..L.L.....
// LLLLLLLLLL
// L.LLLLLL.L
// L.LLLLL.LL
//
// Now, you just need to model the people who will be arriving shortly. Fortunately, people are entirely predictable and always follow a simple set of rules. All decisions are based on the number of occupied seats adjacent to a given seat (one of the eight positions immediately up, down, left, right, or diagonal from the seat). The following rules are applied to every seat simultaneously:
//
//     If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
//     If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
//     Otherwise, the seat's state does not change.
//
// Floor (.) never changes; seats don't move, and nobody sits on the floor.
//
// After one round of these rules, every seat in the example layout becomes occupied:
//
// #.##.##.##
// #######.##
// #.#.#..#..
// ####.##.##
// #.##.##.##
// #.#####.##
// ..#.#.....
// ##########
// #.######.#
// #.#####.##
//
// After a second round, the seats with four or more occupied adjacent seats become empty again:
//
// #.LL.L#.##
// #LLLLLL.L#
// L.L.L..L..
// #LLL.LL.L#
// #.LL.LL.LL
// #.LLLL#.##
// ..L.L.....
// #LLLLLLLL#
// #.LLLLLL.L
// #.#LLLL.##
//
// This process continues for three more rounds:
//
// #.##.L#.##
// #L###LL.L#
// L.#.#..#..
// #L##.##.L#
// #.##.LL.LL
// #.###L#.##
// ..#.#.....
// #L######L#
// #.LL###L.L
// #.#L###.##
//
// #.#L.L#.##
// #LLL#LL.L#
// L.L.L..#..
// #LLL.##.L#
// #.LL.LL.LL
// #.LL#L#.##
// ..L.L.....
// #L#LLLL#L#
// #.LLLLLL.L
// #.#L#L#.##
//
// #.#L.L#.##
// #LLL#LL.L#
// L.#.L..#..
// #L##.##.L#
// #.#L.LL.LL
// #.#L#L#.##
// ..L.L.....
// #L#L##L#L#
// #.LLLLLL.L
// #.#L#L#.##
//
// At this point, something interesting happens: the chaos stabilizes and further applications of these rules cause no seats to change state! Once people stop moving around, you count 37 occupied seats.
//
// Simulate your seating area by applying the seating rules repeatedly until no seats change state. How many seats end up occupied?
//
// Your puzzle answer was 2178.
// --- Part Two ---
//
// As soon as people start to arrive, you realize your mistake. People don't just care about adjacent seats - they care about the first seat they can see in each of those eight directions!
//
// Now, instead of considering just the eight immediately adjacent seats, consider the first seat in each of those eight directions. For example, the empty seat below would see eight occupied seats:
//
// .......#.
// ...#.....
// .#.......
// .........
// ..#L....#
// ....#....
// .........
// #........
// ...#.....
//
// The leftmost empty seat below would only see one empty seat, but cannot see any of the occupied ones:
//
// .............
// .L.L.#.#.#.#.
// .............
//
// The empty seat below would see no occupied seats:
//
// .##.##.
// #.#.#.#
// ##...##
// ...L...
// ##...##
// #.#.#.#
// .##.##.
//
// Also, people seem to be more tolerant than you expected: it now takes five or more visible occupied seats for an occupied seat to become empty (rather than four or more from the previous rules). The other rules still apply: empty seats that see no occupied seats become occupied, seats matching no rule don't change, and floor never changes.
//
// Given the same starting layout as above, these new rules cause the seating area to shift around as follows:
//
// L.LL.LL.LL
// LLLLLLL.LL
// L.L.L..L..
// LLLL.LL.LL
// L.LL.LL.LL
// L.LLLLL.LL
// ..L.L.....
// LLLLLLLLLL
// L.LLLLLL.L
// L.LLLLL.LL
//
// #.##.##.##
// #######.##
// #.#.#..#..
// ####.##.##
// #.##.##.##
// #.#####.##
// ..#.#.....
// ##########
// #.######.#
// #.#####.##
//
// #.LL.LL.L#
// #LLLLLL.LL
// L.L.L..L..
// LLLL.LL.LL
// L.LL.LL.LL
// L.LLLLL.LL
// ..L.L.....
// LLLLLLLLL#
// #.LLLLLL.L
// #.LLLLL.L#
//
// #.L#.##.L#
// #L#####.LL
// L.#.#..#..
// ##L#.##.##
// #.##.#L.##
// #.#####.#L
// ..#.#.....
// LLL####LL#
// #.L#####.L
// #.L####.L#
//
// #.L#.L#.L#
// #LLLLLL.LL
// L.L.L..#..
// ##LL.LL.L#
// L.LL.LL.L#
// #.LLLLL.LL
// ..L.L.....
// LLLLLLLLL#
// #.LLLLL#.L
// #.L#LL#.L#
//
// #.L#.L#.L#
// #LLLLLL.LL
// L.L.L..#..
// ##L#.#L.L#
// L.L#.#L.L#
// #.L####.LL
// ..#.#.....
// LLL###LLL#
// #.LLLLL#.L
// #.L#LL#.L#
//
// #.L#.L#.L#
// #LLLLLL.LL
// L.L.L..#..
// ##L#.#L.L#
// L.L#.LL.L#
// #.LLLL#.LL
// ..#.L.....
// LLL###LLL#
// #.LLLLL#.L
// #.L#LL#.L#
//
// Again, at this point, people stop shifting around and the seating area reaches equilibrium. Once this occurs, you count 26 occupied seats.
//
// Given the new visibility method and the rule change for occupied seats becoming empty, once equilibrium is reached, how many seats end up occupied?
//
// Your puzzle answer was 1978.

use std::fs;

use anyhow::Result;
use itertools::Either;

use crate::chargrid::Grid;
use crate::geom::{Direction, Vector2};

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2020/day11")?;
    let seats = Grid::parse(&input)?;
    println!("{}", num_occupied(&evolve(&seats, true)));
    println!("{}", num_occupied(&evolve(&seats, false)));
    Ok(())
}

fn evolve(seats: &Grid, directly_adj: bool) -> Grid {
    let mut seats = seats.clone();
    let mut changed = true;
    while changed {
        changed = false;
        let mut new_seats = seats.clone();
        for (y, x, &c) in seats.iter() {
            if c == b'.' { continue; }

            let num_adj = count_visibly_occupied(&seats, y, x, directly_adj);
            if c == b'L' && num_adj == 0 {
                new_seats[y][x] = b'#';
                changed = true;
            } else if c == b'#'
                && (num_adj >= 5 || (directly_adj && num_adj >= 4)) {
                new_seats[y][x] = b'L';
                changed = true;
            }
        }
        seats = new_seats;
    }
    seats
}

fn count_visibly_occupied(seats: &Grid,
                          y: usize, x: usize,
                          directly_adj: bool) -> usize {
    Direction::VALUES_8D.iter()
        .map(|dir| {
            let iter = std::iter::successors(
                Some(Vector2::new(x, y)), move |pos| {
                    let pos = *pos + dir.dxdy();
                    if pos.y < 0 || pos.x < 0 ||
                        pos.y as usize >= seats.num_rows ||
                        pos.x as usize >= seats.num_cols {
                        None
                    } else {
                        Some(pos)
                    }
                }).skip(1);
            let iter = if directly_adj {
                Either::Left(iter.take(1))
            } else {
                Either::Right(iter)
            };
            iter.map(|pos| seats[pos])
                .skip_while(|&c| c == b'.')
                .take(1)
                .filter(|&c| c == b'#')
                .count()
        })
        .sum()
}

fn num_occupied(seats: &Grid) -> usize {
    seats.iter()
        .filter(|(_, _, &c)| c == b'#')
        .count()
}

#[cfg(test)]
mod tests {
    use crate::chargrid::Grid;

    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL
        ";
        let seats = Grid::parse(s)?;
        assert_eq!(num_occupied(&evolve(&seats, true)), 37);
        assert_eq!(num_occupied(&evolve(&seats, false)), 26);

        Ok(())
    }
}

mod chargrid {
    use std::fmt::{Display, Formatter};
    use std::fmt;
    use std::ops::{Index, IndexMut};

    use anyhow::Result;
    use itertools::Itertools;

    use crate::geom::Vector2;

    #[derive(Clone, Hash, Eq, PartialEq, Debug)]
    pub struct Grid {
        grid: Vec<Vec<u8>>,
        pub num_rows: usize,
        pub num_cols: usize,
    }

    impl Grid {
        pub fn parse(s: &str) -> Result<Grid> {
            let grid = s.lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
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

        pub fn iter(&self) -> impl Iterator<Item=(usize, usize, &u8)> {
            self.grid.iter().enumerate()
                .flat_map(|(y, row)| {
                    row.iter().enumerate()
                        .map(move |(x, c)| (y, x, c))
                })
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
    use std::convert::{TryFrom, TryInto};
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
