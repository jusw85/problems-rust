use aoc2019::eat_at_restaurant;

// get cookie via firefox -> web developer -> network -> copy cookie
// i=1;curl 'https://adventofcode.com/2019/day/$i/input' -H 'Cookie: session=53616c7465645f5f6fe08b32142bb3e5ce2616eeba44a4051dcb3ee6f38618f4a2b13058e2d4a02d35320bd5f77fb48a' > day$i

// run via
// $ cargo run --package aoc2019 --bin day1
fn main() {
    eat_at_restaurant();
    println!("aoc2019")
}
