// get cookie via firefox -> web developer -> network -> copy cookie
// i=1;curl 'https://adventofcode.com/2019/day/$i/input' -H 'Cookie: session=53616c7465645f5f6fe08b32142bb3e5ce2616eeba44a4051dcb3ee6f38618f4a2b13058e2d4a02d35320bd5f77fb48a' > day$i

// run via
// $ cargo run --package aoc2019 --bin day1

// gdb:
// let nums: &mut [i32]
// ptype nums
//
// type = struct &mut [i32] {
// data_ptr: *mut i32,
// length: usize,
// }
//
// p *nums.data_ptr@nums.length
//
//
// let nums: &[i32]
// ptype nums
//
// type = struct &[i32] {
//   data_ptr: *mut i32,
//   length: usize,
// }
// p x[0]@nums.length
//
// $ display var
//
// #![allow(dead_code)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(unused_mut)]


// use std::time::Instant;
// let now = Instant::now();
//
// let elapsed = now.elapsed();
// println!("Elapsed: {:.2?}", elapsed);


fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
