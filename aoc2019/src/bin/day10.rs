// https://adventofcode.com/2019/day/10
//
//

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2019/day10")?;
    let p1p2 = part1part2(&input);
    println!("{:?}", p1p2);
    Ok(())
}

fn part1part2(s: &str) -> ((i32, i32), usize, Option<i32>) {
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

    let mut los_map: HashMap<_, Vec<_>> = HashMap::new();

    for (i, &a1) in asteroids.iter().enumerate() {
        for (j, &a2) in asteroids[i + 1..].iter().enumerate() {
            let g_a1a2 = simplify(dydx(a1, a2));

            let mut has_los = true;
            for &a3 in asteroids[(i + 1)..(i + 1 + j)].iter() {
                let g_a1a3 = simplify(dydx(a1, a3));
                if g_a1a2 == g_a1a3 {
                    has_los = false;
                    break;
                }
            }
            if has_los {
                los_map.entry(a1).or_default().push(a2);
                los_map.entry(a2).or_default().push(a1);
            }
        }
    }

    let (&origin, los_asteroids) = los_map.iter_mut()
        .max_by(|(_, v1), (_, v2)| {
            let l1 = v1.len();
            let l2 = v2.len();
            return l1.cmp(&l2);
        }).unwrap();

    los_asteroids.sort_unstable_by(|&p1, &p2| {
        let q1 = quadrant(origin, p1);
        let q2 = quadrant(origin, p2);
        if q1 != q2 {
            return q1.cmp(&q2);
        }
        let g1 = gradient(origin, p1);
        let g2 = gradient(origin, p2);
        assert_ne!(g1, g2);
        if g1 > g2 { Ordering::Less } else { Ordering::Greater }
    });

    let xy200 = if los_asteroids.len() >= 200 {
        let (y, x) = los_asteroids[199];
        Some(x * 100 + y)
    } else {
        None
    };

    let (y, x) = origin;
    ((x, y), los_asteroids.len(), xy200)
}

fn quadrant((origin_y, origin_x): (i32, i32), (y, x): (i32, i32)) -> u8 {
    if (x >= origin_x) && (y < origin_y) { return 0; }
    if (y >= origin_y) && (x > origin_x) { return 1; }
    if (x <= origin_x) && (y > origin_y) { return 2; }
    if (y <= origin_y) && (x < origin_x) { return 3; }
    unreachable!();
}

fn gradient(p1: (i32, i32), p2: (i32, i32)) -> f64 {
    let (y, x) = dydx(p1, p2);
    let mut res = -(y as f64 / x as f64); // flip y direction
    if res.is_infinite() && res.is_sign_negative() {
        res = f64::INFINITY;
    }
    res
}

fn dydx((y1, x1): (i32, i32), (y2, x2): (i32, i32)) -> (i32, i32) {
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
    fn test() -> Result<()> {
        let s = "
        .#..#
        .....
        #####
        ....#
        ...##";
        let (coord, len, _) = part1part2(s);
        assert_eq!((coord, len), ((3, 4), 8));

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
        let (coord, len, _) = part1part2(s);
        assert_eq!((coord, len), ((5, 8), 33));

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
        let (coord, len, _) = part1part2(s);
        assert_eq!((coord, len), ((6, 3), 41));

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
        assert_eq!(part1part2(s), ((11, 13), 210, Some(802)));
        Ok(())
    }
}
