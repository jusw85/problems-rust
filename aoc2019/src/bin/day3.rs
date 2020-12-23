// https://adventofcode.com/2019/day/3
//
// --- Day 3: Crossed Wires ---
//
// The gravity assist was successful, and you're well on your way to the Venus refuelling station. During the rush back on Earth, the fuel management system wasn't completely installed, so that's next on the priority list.
//
// Opening the front panel reveals a jumble of wires. Specifically, two wires are connected to a central port and extend outward on a grid. You trace the path each wire takes as it leaves the central port, one wire per line of text (your puzzle input).
//
// The wires twist and turn, but the two wires occasionally cross paths. To fix the circuit, you need to find the intersection point closest to the central port. Because the wires are on a grid, use the Manhattan distance for this measurement. While the wires do technically cross right at the central port where they both start, this point does not count, nor does a wire count as crossing with itself.
//
// For example, if the first wire's path is R8,U5,L5,D3, then starting from the central port (o), it goes right 8, up 5, left 5, and finally down 3:
//
// ...........
// ...........
// ...........
// ....+----+.
// ....|....|.
// ....|....|.
// ....|....|.
// .........|.
// .o-------+.
// ...........
//
// Then, if the second wire's path is U7,R6,D4,L4, it goes up 7, right 6, down 4, and left 4:
//
// ...........
// .+-----+...
// .|.....|...
// .|..+--X-+.
// .|..|..|.|.
// .|.-X--+.|.
// .|..|....|.
// .|.......|.
// .o-------+.
// ...........
//
// These wires cross at two locations (marked X), but the lower-left one is closer to the central port: its distance is 3 + 3 = 6.
//
// Here are a few more examples:
//
//     R75,D30,R83,U83,L12,D49,R71,U7,L72
//     U62,R66,U55,R34,D71,R55,D58,R83 = distance 159
//     R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
//     U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = distance 135
//
// What is the Manhattan distance from the central port to the closest intersection?
//
// Your puzzle answer was 403.
// --- Part Two ---
//
// It turns out that this circuit is very timing-sensitive; you actually need to minimize the signal delay.
//
// To do this, calculate the number of steps each wire takes to reach each intersection; choose the intersection where the sum of both wires' steps is lowest. If a wire visits a position on the grid multiple times, use the steps value from the first time it visits that position when calculating the total value of a specific intersection.
//
// The number of steps a wire takes is the total number of grid squares the wire has entered to get to that location, including the intersection being considered. Again consider the example from above:
//
// ...........
// .+-----+...
// .|.....|...
// .|..+--X-+.
// .|..|..|.|.
// .|.-X--+.|.
// .|..|....|.
// .|.......|.
// .o-------+.
// ...........
//
// In the above example, the intersection closest to the central port is reached after 8+5+5+2 = 20 steps by the first wire and 7+6+4+3 = 20 steps by the second wire for a total of 20+20 = 40 steps.
//
// However, the top-right intersection is better: the first wire takes only 8+5+2 = 15 and the second wire takes only 7+6+2 = 15, a total of 15+15 = 30 steps.
//
// Here are the best steps for the extra examples from above:
//
//     R75,D30,R83,U83,L12,D49,R71,U7,L72
//     U62,R66,U55,R34,D71,R55,D58,R83 = 610 steps
//     R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
//     U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = 410 steps
//
// What is the fewest combined steps the wires must take to reach an intersection?
//
// Your puzzle answer was 4158.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::FromStr;

use anyhow;
use anyhow::Context;

enum Direction {
    U,
    R,
    D,
    L,
}

impl Direction {
    fn dxdy(&self) -> (i32, i32) {
        match self {
            Direction::U => (0, -1),
            Direction::R => (1, 0),
            Direction::D => (0, 1),
            Direction::L => (-1, 0),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

struct Move {
    dir: Direction,
    len: i32,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, len) = (&s[0..1], &s[1..]);
        let len = len.parse::<i32>()
            .with_context(|| format!("Failed to parse {}", s))?;

        match dir {
            "U" => Ok(Move { dir: Direction::U, len }),
            "R" => Ok(Move { dir: Direction::R, len }),
            "D" => Ok(Move { dir: Direction::D, len }),
            "L" => Ok(Move { dir: Direction::L, len }),
            _ => anyhow::bail!("Invalid move {}", s),
        }
    }
}

fn parse_wire(s: &str) -> Result<Vec<Move>, anyhow::Error> {
    s.trim()
        .split(',')
        .map(|x| x.parse::<Move>())
        .collect()
}

fn lay_wire(wire: &[Move]) -> HashMap<Point, u32> {
    let mut points = HashMap::new();
    let mut pos = Point { x: 0, y: 0 };
    let mut steps = 0;

    for mov in wire {
        let (dx, dy) = mov.dir.dxdy();
        for _ in 0..mov.len {
            pos.x += dx;
            pos.y += dy;
            steps += 1;
            points.entry(pos).or_insert(steps);
        }
    }
    points
}

fn main() -> Result<(), anyhow::Error> {
    let input = fs::read_to_string("input/aoc2019/day3")?;

    let wires =
        input.lines().take(2)
            .map(|line| parse_wire(line))
            .collect::<Result<Vec<_>, _>>()?;

    anyhow::ensure!(wires.len() >= 2, "insufficient lines");

    let (p1, p2) = part1_part2(&wires[0], &wires[1]);
    let p1 = p1.ok_or_else(|| anyhow::anyhow!("no intersections"))?;
    let p2 = p2.ok_or_else(|| anyhow::anyhow!("no intersections"))?;
    println!("{}", p1);
    println!("{}", p2);
    Ok(())
}

fn part1_part2(wire0: &[Move], wire1: &[Move]) -> (Option<u32>, Option<u32>) {
    let wire0 = lay_wire(wire0);
    let wire1 = lay_wire(wire1);
    let wire0_points: HashSet<&Point> = wire0.keys().collect();
    let wire1_points: HashSet<&Point> = wire1.keys().collect();
    let intersects: Vec<&&Point> = wire0_points.intersection(&wire1_points).collect();

    let min_md = intersects.iter()
        .map(|x| x.manhattan_distance())
        .min();

    let min_steps = intersects.iter()
        .map(|x| wire0[x] + wire1[x])
        .min();

    (min_md, min_steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() -> Result<(), anyhow::Error> {
        let wire0 = parse_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72")?;
        let wire1 = parse_wire("U62,R66,U55,R34,D71,R55,D58,R83")?;
        let (p1, p2) = part1_part2(&wire0, &wire1);
        assert_eq!((p1.unwrap(), p2.unwrap()), (159, 610));
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), anyhow::Error> {
        let wire0 = parse_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51")?;
        let wire1 = parse_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")?;
        let (p1, p2) = part1_part2(&wire0, &wire1);
        assert_eq!((p1.unwrap(), p2.unwrap()), (135, 410));
        Ok(())
    }
}
