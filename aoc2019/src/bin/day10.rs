// https://adventofcode.com/2019/day/10
//
//

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::collections::{HashMap, VecDeque};
use std::fs;

use anyhow::Context;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2019/day10")?;
    let p1 = part1(&input);

    println!("{:?}", p1);

    Ok(())
}

fn part1(s: &str) -> ((i32, i32), u32) {
    let grid: Vec<Vec<u8>> =
        s.trim()
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.as_bytes().to_vec())
            .collect();

    let mut asteroids = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, elem) in row.iter().enumerate() {
            if *elem == b'#' {
                asteroids.push((y as i32, x as i32));
            }
        }
    }

    let mut map: HashMap<_, Vec<_>> = HashMap::new();

    let mut los = vec![vec![false; asteroids.len()]; asteroids.len()];
    for i in 0..asteroids.len() {
        for j in (i + 1)..asteroids.len() {
            let a1 = asteroids[i];
            let a2 = asteroids[j];
            let g_a1a2 = simplify(gradient(a1, a2));

            let mut has_los = true;
            for a3 in &asteroids[(i + 1)..j] {
                let g_a1a3 = simplify(gradient(a1, *a3));
                if g_a1a2 == g_a1a3 {
                    has_los = false;
                    break;
                }
            }

            if has_los {
                los[i][j] = true;
                los[j][i] = true;

                map.entry(a1).or_default().push(a2);
                map.entry(a2).or_default().push(a1);
            }
        }
    }

    // map.iter().map(|(key, val)| (key, val.len())).max_by_key(|(_, len)| *len).unwrap();
    
    let los_count: Vec<_> = los.iter().map(|row| row.iter().filter(|&&e| e).count()).collect();
    let (max_los_idx, max_los) = los_count.iter().enumerate().max_by_key(|(_, &val)| val).unwrap();
    let (y, x) = asteroids[max_los_idx];
    ((x, y), *max_los as u32)
}

fn gradient((y1, x1): (i32, i32), (y2, x2): (i32, i32)) -> (i32, i32) {
    ((y1 - y2), (x1 - x2))
}

fn simplify((numerator, denominator): (i32, i32)) -> (i32, i32) {
    let gcd = gcd(numerator, denominator);
    (numerator / gcd, denominator / gcd)
}

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a.abs();
    let mut b = b.abs();
    let mut t;
    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(1, 0), 1);
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(3, 7), 1);
        assert_eq!(gcd(5, 15), 5);
        assert_eq!(gcd(-5, 15), 5);

        assert_eq!(simplify((-2, 10)), (-1, 5));
        assert_eq!(simplify((6, 8)), (3, 4));
    }

    #[test]
    fn test_part1() -> Result<()> {
        let s = "
        .#..#
        .....
        #####
        ....#
        ...##";
        assert_eq!(part1(s), ((3, 4), 8));

        let s = "
        ......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####";
        assert_eq!(part1(s), ((5, 8), 33));

        let s = "
        .#..#..###
        ####.###.#
        ....###.#.
        ..###.##.#
        ##.##.#.#.
        ....###..#
        ..#.#..#.#
        #..#.#.###
        .##...##.#
        .....#.#..";
        assert_eq!(part1(s), ((6, 3), 41));

        let s = "
        .#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##";
        assert_eq!(part1(s), ((11, 13), 210));
        Ok(())
    }
}
