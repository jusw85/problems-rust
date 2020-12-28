// https://adventofcode.com/2019/day/12
//
//

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::{fs, iter};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

use crate::geom::Vector3;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2019/day12")?;
    let mut bodies = parse(&input)?;
    let mut bodies_clone = bodies.clone();
    for _ in 0..1000 {
        sim(&mut bodies);
    }
    println!("{}", energy(bodies));
    println!("{}", seen_steps(bodies_clone));
    Ok(())
}

fn parse(s: &str) -> Result<Vec<Body>> {
    lazy_static! { static ref RE: Regex = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap(); }
    let mut res = Vec::new();
    for cap in RE.captures_iter(s) {
        let x = cap[1].parse::<i32>()?;
        let y = cap[2].parse::<i32>()?;
        let z = cap[3].parse::<i32>()?;
        let body = Body::new(Vector3::new(x, y, z));
        res.push(body);
    }
    Ok(res)
}

fn sim(bodies: &mut Vec<Body>) {
    for i in 1..bodies.len() {
        let (left, right) = bodies.split_at_mut(i);
        let b1 = left.get_mut(i - 1).unwrap();
        for b2 in right.iter_mut() {
            let del: Vector3 = delta_vec3(b1.pos, b2.pos);
            b1.vel += del;
            b2.vel -= del;
        }
    }
    for b in bodies.iter_mut() {
        b.pos += b.vel;
    }
}

fn energy(bodies: Vec<Body>) -> i32 {
    let pots = bodies.iter().map(|b|
        b.pos.x.abs() + b.pos.y.abs() + b.pos.z.abs());
    let kins = bodies.iter().map(|b|
        b.vel.x.abs() + b.vel.y.abs() + b.vel.z.abs());
    pots.zip(kins).map(|(p, k)| p * k).sum()
}

fn delta_vec3(from: Vector3, to: Vector3) -> Vector3 {
    Vector3 {
        x: delta_i32(from.x, to.x),
        y: delta_i32(from.y, to.y),
        z: delta_i32(from.z, to.z),
    }
}

fn delta_i32(from: i32, to: i32) -> i32 {
    match from.cmp(&to) {
        Ordering::Less => { 1 }
        Ordering::Equal => { 0 }
        Ordering::Greater => { -1 }
    }
}

fn seen_steps(bodies: Vec<Body>) -> u64 {
    fn count_steps(mut bodies: Vec<Body>) -> i32 {
        let mut seen = HashSet::new();
        let mut steps = 0;
        while !seen.contains(&bodies) {
            seen.insert(bodies.clone());
            sim(&mut bodies);
            steps += 1;
        }
        steps
    }
    let mut bodies_x: Vec<_> = bodies.iter().map(|body| {
        let mut pos = Vector3::ZERO;
        let vel = Vector3::ZERO;
        pos.x = body.pos.x;
        Body { pos, vel }
    }).collect();

    let mut bodies_y: Vec<_> = bodies.iter().map(|body| {
        let mut pos = Vector3::ZERO;
        let vel = Vector3::ZERO;
        pos.y = body.pos.y;
        Body { pos, vel }
    }).collect();

    let mut bodies_z: Vec<_> = bodies.iter().map(|body| {
        let mut pos = Vector3::ZERO;
        let vel = Vector3::ZERO;
        pos.z = body.pos.z;
        Body { pos, vel }
    }).collect();

    let steps_x = count_steps(bodies_x) as u64;
    let steps_y = count_steps(bodies_y) as u64;
    let steps_z = count_steps(bodies_z) as u64;

    lcm(lcm(steps_x, steps_y), steps_z)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    let mut t;
    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 && b == 0 {
        return 0;
    }
    (a * b) / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        <x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>
        ";
        let mut bodies = parse(s)?;
        assert_eq!(2772, seen_steps(bodies.clone()));
        for _ in 0..10 {
            sim(&mut bodies);
        }
        assert_eq!(179, energy(bodies));
        Ok(())
    }

    #[test]
    fn test2() -> Result<()> {
        let s = r"
        <x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>
        ";
        let mut bodies = parse(s)?;
        assert_eq!(4686774924, seen_steps(bodies));
        Ok(())
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Default, Debug)]
struct Body {
    pos: Vector3,
    vel: Vector3,
}

impl Body {
    pub fn new(pos: Vector3) -> Body {
        Body {
            pos,
            vel: Vector3::ZERO,
        }
    }
}

mod geom {
    use std::ops::{AddAssign, Neg, SubAssign};

    #[derive(Eq, PartialEq, Hash, Copy, Clone, Default, Debug)]
    pub struct Vector3 {
        pub x: i32,
        pub y: i32,
        pub z: i32,
    }

    impl Vector3 {
        pub const ZERO: Vector3 = Vector3 { x: 0, y: 0, z: 0 };

        pub fn new(x: i32, y: i32, z: i32) -> Vector3 {
            Vector3 { x, y, z }
        }
    }

    impl AddAssign for Vector3 {
        fn add_assign(&mut self, other: Vector3) {
            *self = Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            };
        }
    }

    impl SubAssign for Vector3 {
        fn sub_assign(&mut self, other: Vector3) {
            *self = Self {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            };
        }
    }

    impl Neg for Vector3 {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Vector3 {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            }
        }
    }
}
