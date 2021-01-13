// https://adventofcode.com/2020/day/17
//
// --- Day 17: Conway Cubes ---
//
// As your flight slowly drifts through the sky, the Elves at the Mythical Information Bureau at the North Pole contact you. They'd like some help debugging a malfunctioning experimental energy source aboard one of their super-secret imaging satellites.
//
// The experimental energy source is based on cutting-edge technology: a set of Conway Cubes contained in a pocket dimension! When you hear it's having problems, you can't help but agree to take a look.
//
// The pocket dimension contains an infinite 3-dimensional grid. At every integer 3-dimensional coordinate (x,y,z), there exists a single cube which is either active or inactive.
//
// In the initial state of the pocket dimension, almost all cubes start inactive. The only exception to this is a small flat region of cubes (your puzzle input); the cubes in this region start in the specified active (#) or inactive (.) state.
//
// The energy source then proceeds to boot up by executing six cycles.
//
// Each cube only ever considers its neighbors: any of the 26 other cubes where any of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3, its neighbors include the cube at x=2,y=2,z=2, the cube at x=0,y=2,z=3, and so on.
//
// During a cycle, all cubes simultaneously change their state according to the following rules:
//
//     If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
//     If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
//
// The engineers responsible for this experimental energy source would like you to simulate the pocket dimension and determine what the configuration of cubes should be at the end of the six-cycle boot process.
//
// For example, consider the following initial state:
//
// .#.
// ..#
// ###
//
// Even though the pocket dimension is 3-dimensional, this initial state represents a small 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1 region of the 3-dimensional space.)
//
// Simulating a few cycles from this initial state produces the following configurations, where the result of each cycle is shown layer-by-layer at each given z coordinate (and the frame of view follows the active cells in each cycle):
//
// Before any cycles:
//
// z=0
// .#.
// ..#
// ###
//
//
// After 1 cycle:
//
// z=-1
// #..
// ..#
// .#.
//
// z=0
// #.#
// .##
// .#.
//
// z=1
// #..
// ..#
// .#.
//
//
// After 2 cycles:
//
// z=-2
// .....
// .....
// ..#..
// .....
// .....
//
// z=-1
// ..#..
// .#..#
// ....#
// .#...
// .....
//
// z=0
// ##...
// ##...
// #....
// ....#
// .###.
//
// z=1
// ..#..
// .#..#
// ....#
// .#...
// .....
//
// z=2
// .....
// .....
// ..#..
// .....
// .....
//
//
// After 3 cycles:
//
// z=-2
// .......
// .......
// ..##...
// ..###..
// .......
// .......
// .......
//
// z=-1
// ..#....
// ...#...
// #......
// .....##
// .#...#.
// ..#.#..
// ...#...
//
// z=0
// ...#...
// .......
// #......
// .......
// .....##
// .##.#..
// ...#...
//
// z=1
// ..#....
// ...#...
// #......
// .....##
// .#...#.
// ..#.#..
// ...#...
//
// z=2
// .......
// .......
// ..##...
// ..###..
// .......
// .......
// .......
//
// After the full six-cycle boot process completes, 112 cubes are left in the active state.
//
// Starting with your given initial configuration, simulate six cycles. How many cubes are left in the active state after the sixth cycle?
//
// Your puzzle answer was 338.
// --- Part Two ---
//
// For some reason, your simulated results don't match what the experimental energy source engineers expected. Apparently, the pocket dimension actually has four spatial dimensions, not three.
//
// The pocket dimension contains an infinite 4-dimensional grid. At every integer 4-dimensional coordinate (x,y,z,w), there exists a single cube (really, a hypercube) which is still either active or inactive.
//
// Each cube only ever considers its neighbors: any of the 80 other cubes where any of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3,w=4, its neighbors include the cube at x=2,y=2,z=3,w=3, the cube at x=0,y=2,z=3,w=4, and so on.
//
// The initial state of the pocket dimension still consists of a small flat region of cubes. Furthermore, the same rules for cycle updating still apply: during each cycle, consider the number of active neighbors of each cube.
//
// For example, consider the same initial state as in the example above. Even though the pocket dimension is 4-dimensional, this initial state represents a small 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1x1 region of the 4-dimensional space.)
//
// Simulating a few cycles from this initial state produces the following configurations, where the result of each cycle is shown layer-by-layer at each given z and w coordinate:
//
// Before any cycles:
//
// z=0, w=0
// .#.
// ..#
// ###
//
//
// After 1 cycle:
//
// z=-1, w=-1
// #..
// ..#
// .#.
//
// z=0, w=-1
// #..
// ..#
// .#.
//
// z=1, w=-1
// #..
// ..#
// .#.
//
// z=-1, w=0
// #..
// ..#
// .#.
//
// z=0, w=0
// #.#
// .##
// .#.
//
// z=1, w=0
// #..
// ..#
// .#.
//
// z=-1, w=1
// #..
// ..#
// .#.
//
// z=0, w=1
// #..
// ..#
// .#.
//
// z=1, w=1
// #..
// ..#
// .#.
//
//
// After 2 cycles:
//
// z=-2, w=-2
// .....
// .....
// ..#..
// .....
// .....
//
// z=-1, w=-2
// .....
// .....
// .....
// .....
// .....
//
// z=0, w=-2
// ###..
// ##.##
// #...#
// .#..#
// .###.
//
// z=1, w=-2
// .....
// .....
// .....
// .....
// .....
//
// z=2, w=-2
// .....
// .....
// ..#..
// .....
// .....
//
// z=-2, w=-1
// .....
// .....
// .....
// .....
// .....
//
// z=-1, w=-1
// .....
// .....
// .....
// .....
// .....
//
// z=0, w=-1
// .....
// .....
// .....
// .....
// .....
//
// z=1, w=-1
// .....
// .....
// .....
// .....
// .....
//
// z=2, w=-1
// .....
// .....
// .....
// .....
// .....
//
// z=-2, w=0
// ###..
// ##.##
// #...#
// .#..#
// .###.
//
// z=-1, w=0
// .....
// .....
// .....
// .....
// .....
//
// z=0, w=0
// .....
// .....
// .....
// .....
// .....
//
// z=1, w=0
// .....
// .....
// .....
// .....
// .....
//
// z=2, w=0
// ###..
// ##.##
// #...#
// .#..#
// .###.
//
// z=-2, w=1
// .....
// .....
// .....
// .....
// .....
//
// z=-1, w=1
// .....
// .....
// .....
// .....
// .....
//
// z=0, w=1
// .....
// .....
// .....
// .....
// .....
//
// z=1, w=1
// .....
// .....
// .....
// .....
// .....
//
// z=2, w=1
// .....
// .....
// .....
// .....
// .....
//
// z=-2, w=2
// .....
// .....
// ..#..
// .....
// .....
//
// z=-1, w=2
// .....
// .....
// .....
// .....
// .....
//
// z=0, w=2
// ###..
// ##.##
// #...#
// .#..#
// .###.
//
// z=1, w=2
// .....
// .....
// .....
// .....
// .....
//
// z=2, w=2
// .....
// .....
// ..#..
// .....
// .....
//
// After the full six-cycle boot process completes, 848 cubes are left in the active state.
//
// Starting with your given initial configuration, simulate six cycles in a 4-dimensional space. How many cubes are left in the active state after the sixth cycle?
//
// Your puzzle answer was 2440.

use std::collections::{HashMap, HashSet};
use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2020/day17")?;
    let pts = parse(&input);
    let mut pts3d = to_3d(&pts);
    for _ in 0..6 {
        evolve(&mut pts3d);
    }
    println!("{}", pts3d.len());

    let mut pts4d = to_4d(&pts);
    for _ in 0..6 {
        evolve(&mut pts4d);
    }
    println!("{}", pts4d.len());
    Ok(())
}

fn parse(s: &str) -> HashSet<(usize, usize)> {
    s.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(y, row)|
            row.bytes().enumerate()
                .filter_map(move |(x, c)| if c == b'#' { Some((x, y)) } else { None }))
        .collect::<HashSet<_>>()
}

fn to_3d(pts: &HashSet<(usize, usize)>) -> HashSet<Vec<i32>> {
    pts.iter().map(|&(x, y)| vec![x as i32, y as i32, 0]).collect()
}

fn to_4d(pts: &HashSet<(usize, usize)>) -> HashSet<Vec<i32>> {
    pts.iter().map(|&(x, y)| vec![x as i32, y as i32, 0, 0]).collect()
}

fn evolve(pts: &mut HashSet<Vec<i32>>) {
    let mut num_neighbours = HashMap::new();
    for pt in pts.iter() {
        for npt in neighbours(pt) {
            *num_neighbours.entry(npt).or_insert(0) += 1;
        }
    }
    // pts should be cloned for atomic operation, but ok since rule overlap (3 neighbours = active)
    pts.retain(|pt| {
        if let Some(&n) = num_neighbours.get(pt) {
            if n == 2 || n == 3 {
                return true;
            }
        }
        return false;
    });
    let iter = num_neighbours.iter().filter_map(|(pt, &n)|
        if n == 3 {
            Some(pt.clone())
        } else {
            None
        }
    );
    pts.extend(iter);
}

fn neighbours(pt: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![vec![]];
    let mut res2 = Vec::new();
    for &dim in pt.iter() {
        for mut x in res.drain(..) {
            let mut x1 = x.clone();
            x1.push(dim - 1);
            let mut x2 = x.clone();
            x2.push(dim);
            x.push(dim + 1);
            res2.push(x1);
            res2.push(x2);
            res2.push(x);
        }
        let t = res;
        res = res2;
        res2 = t;
    }
    res.retain(|v| v != pt);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        .#.
        ..#
        ###
        ";
        let pts = parse(s);
        let mut pts3d = to_3d(&pts);
        for _ in 0..6 {
            evolve(&mut pts3d);
        }
        assert_eq!(112, pts3d.len());

        let mut pts4d = to_4d(&pts);
        for _ in 0..6 {
            evolve(&mut pts4d);
        }
        assert_eq!(848, pts4d.len());
        Ok(())
    }
}
