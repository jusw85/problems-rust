// use std::ops::{Add, AddAssign};
//
// #[derive(Eq, PartialEq, Hash, Copy, Clone, Default, Debug)]
// pub struct Point {
//     pub y: i32,
//     pub x: i32,
// }
//
// impl Point {
//     pub fn new(y: i32, x: i32) -> Point {
//         Point { y, x }
//     }
//
//     pub fn add(&mut self, v: Vector) {
//         self.y += v.dy;
//         self.x += v.dx;
//     }
// }
//
// impl Add<Vector> for Point {
//     type Output = Self;
//
//     fn add(self, vector: Vector) -> Self {
//         Self {
//             y: self.y + vector.dy,
//             x: self.x + vector.dx,
//         }
//     }
// }
//
// impl AddAssign<Vector> for Point {
//     fn add_assign(&mut self, vector: Vector) {
//         *self = Self {
//             y: self.y + vector.dy,
//             x: self.x + vector.dx,
//         };
//     }
// }
//
// #[derive(Eq, PartialEq, Hash, Copy, Clone, Default, Debug)]
// pub struct Vector {
//     pub dy: i32,
//     pub dx: i32,
// }
//
// impl Vector {
//     pub fn new(dy: i32, dx: i32) -> Vector {
//         Vector { dy, dx }
//     }
// }
//
// pub enum Direction {
//     N,
//     E,
//     S,
//     W,
// }
//
// impl Direction {
//     pub fn dydx(&self) -> Vector {
//         match self {
//             Direction::N => Vector::new(-1, 0),
//             Direction::E => Vector::new(0, 1),
//             Direction::S => Vector::new(1, 0),
//             Direction::W => Vector::new(0, -1),
//         }
//     }
// }
