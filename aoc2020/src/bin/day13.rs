// https://adventofcode.com/2020/day/13
//
// --- Day 13: Shuttle Search ---
//
// Your ferry can make it safely to a nearby port, but it won't get much further. When you call to book another ship, you discover that no ships embark from that port to your vacation island. You'll need to get from the port to the nearest airport.
//
// Fortunately, a shuttle bus service is available to bring you from the sea port to the airport! Each bus has an ID number that also indicates how often the bus leaves for the airport.
//
// Bus schedules are defined based on a timestamp that measures the number of minutes since some fixed reference point in the past. At timestamp 0, every bus simultaneously departed from the sea port. After that, each bus travels to the airport, then various other locations, and finally returns to the sea port to repeat its journey forever.
//
// The time this loop takes a particular bus is also its ID number: the bus with ID 5 departs from the sea port at timestamps 0, 5, 10, 15, and so on. The bus with ID 11 departs at 0, 11, 22, 33, and so on. If you are there when the bus departs, you can ride that bus to the airport!
//
// Your notes (your puzzle input) consist of two lines. The first line is your estimate of the earliest timestamp you could depart on a bus. The second line lists the bus IDs that are in service according to the shuttle company; entries that show x must be out of service, so you decide to ignore them.
//
// To save time once you arrive, your goal is to figure out the earliest bus you can take to the airport. (There will be exactly one such bus.)
//
// For example, suppose you have the following notes:
//
// 939
// 7,13,x,x,59,x,31,19
//
// Here, the earliest timestamp you could depart is 939, and the bus IDs in service are 7, 13, 59, 31, and 19. Near timestamp 939, these bus IDs depart at the times marked D:
//
// time   bus 7   bus 13  bus 59  bus 31  bus 19
// 929      .       .       .       .       .
// 930      .       .       .       D       .
// 931      D       .       .       .       D
// 932      .       .       .       .       .
// 933      .       .       .       .       .
// 934      .       .       .       .       .
// 935      .       .       .       .       .
// 936      .       D       .       .       .
// 937      .       .       .       .       .
// 938      D       .       .       .       .
// 939      .       .       .       .       .
// 940      .       .       .       .       .
// 941      .       .       .       .       .
// 942      .       .       .       .       .
// 943      .       .       .       .       .
// 944      .       .       D       .       .
// 945      D       .       .       .       .
// 946      .       .       .       .       .
// 947      .       .       .       .       .
// 948      .       .       .       .       .
// 949      .       D       .       .       .
//
// The earliest bus you could take is bus ID 59. It doesn't depart until timestamp 944, so you would need to wait 944 - 939 = 5 minutes before it departs. Multiplying the bus ID by the number of minutes you'd need to wait gives 295.
//
// What is the ID of the earliest bus you can take to the airport multiplied by the number of minutes you'll need to wait for that bus?
//
// Your puzzle answer was 3997.
// --- Part Two ---
//
// The shuttle company is running a contest: one gold coin for anyone that can find the earliest timestamp such that the first bus ID departs at that time and each subsequent listed bus ID departs at that subsequent minute. (The first line in your input is no longer relevant.)
//
// For example, suppose you have the same list of bus IDs as above:
//
// 7,13,x,x,59,x,31,19
//
// An x in the schedule means there are no constraints on what bus IDs must depart at that time.
//
// This means you are looking for the earliest timestamp (called t) such that:
//
//     Bus ID 7 departs at timestamp t.
//     Bus ID 13 departs one minute after timestamp t.
//     There are no requirements or restrictions on departures at two or three minutes after timestamp t.
//     Bus ID 59 departs four minutes after timestamp t.
//     There are no requirements or restrictions on departures at five minutes after timestamp t.
//     Bus ID 31 departs six minutes after timestamp t.
//     Bus ID 19 departs seven minutes after timestamp t.
//
// The only bus departures that matter are the listed bus IDs at their specific offsets from t. Those bus IDs can depart at other times, and other bus IDs can depart at those times. For example, in the list above, because bus ID 19 must depart seven minutes after the timestamp at which bus ID 7 departs, bus ID 7 will always also be departing with bus ID 19 at seven minutes after timestamp t.
//
// In this example, the earliest timestamp at which this occurs is 1068781:
//
// time     bus 7   bus 13  bus 59  bus 31  bus 19
// 1068773    .       .       .       .       .
// 1068774    D       .       .       .       .
// 1068775    .       .       .       .       .
// 1068776    .       .       .       .       .
// 1068777    .       .       .       .       .
// 1068778    .       .       .       .       .
// 1068779    .       .       .       .       .
// 1068780    .       .       .       .       .
// 1068781    D       .       .       .       .
// 1068782    .       D       .       .       .
// 1068783    .       .       .       .       .
// 1068784    .       .       .       .       .
// 1068785    .       .       D       .       .
// 1068786    .       .       .       .       .
// 1068787    .       .       .       D       .
// 1068788    D       .       .       .       D
// 1068789    .       .       .       .       .
// 1068790    .       .       .       .       .
// 1068791    .       .       .       .       .
// 1068792    .       .       .       .       .
// 1068793    .       .       .       .       .
// 1068794    .       .       .       .       .
// 1068795    D       D       .       .       .
// 1068796    .       .       .       .       .
// 1068797    .       .       .       .       .
//
// In the above example, bus ID 7 departs at timestamp 1068788 (seven minutes after t). This is fine; the only requirement on that minute is that bus ID 19 departs then, and it does.
//
// Here are some other examples:
//
//     The earliest timestamp that matches the list 17,x,13,19 is 3417.
//     67,7,59,61 first occurs at timestamp 754018.
//     67,x,7,59,61 first occurs at timestamp 779210.
//     67,7,x,59,61 first occurs at timestamp 1261476.
//     1789,37,47,1889 first occurs at timestamp 1202161486.
//
// However, with so many bus IDs in your list, surely the actual earliest timestamp will be larger than 100000000000000!
//
// What is the earliest timestamp such that all of the listed bus IDs depart at offsets matching their positions in the list?
//
// Your puzzle answer was 500033211739354.

use std::fs;

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2020/day13")?;
    let (ts, schedule) = parse(&input);
    let (bus, wait) = earliest(ts, &schedule);
    println!("{}", bus * wait);
    let earliest_offset = earliest_offset(&schedule);
    println!("{}", earliest_offset);
    Ok(())
}

fn parse(s: &str) -> (i128, Vec<(usize, i128)>) {
    let mut lines = s.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    let ts = lines.next().unwrap()
        .parse::<i128>().unwrap();

    let schedule = lines.next().unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(i, s)| (i, s.parse::<i128>().unwrap()))
        .collect_vec();
    (ts, schedule)
}

fn earliest(ts: i128, schedule: &Vec<(usize, i128)>) -> (i128, i128) {
    schedule.iter()
        .map(|&(_, timing)| (timing, timing - (ts % timing)))
        .min_by_key(|&(_, timing)| timing)
        .unwrap()
}

// k = ax + offset_a
// k = by + offset_b
// => ax + offset_a = by + offset_b
// e.g. k = 7x + 0, k = 13y + 12
// k = 77, x = 11, y = 5
//
// (extended euclidean):
// 7x + 13y = 1
// 7(2) + 13(-1) = 1
// 7(2)(12) + 13(-1)(12) = 12
//
// k = 7x + 0 = 168
// lcm(7, 13) = 91
// solns = 168 + 91x = 77 + 91x
fn earliest_offset(schedule: &Vec<(usize, i128)>) -> i128 {
    let (offset_a, mut a) = *schedule.first().unwrap();
    let mut offset_a = offset_a as i128;
    for &(offset_b, b) in schedule[1..].iter() {
        let offset_b = b - offset_b as i128;
        let m = offset_b - offset_a;
        let lcm = lcm(a, b);
        let (arg0, arg1, ofs) = if m < 0 { (b, a, offset_b) } else { (a, b, offset_a) };
        let inv = mod_inv(arg0 as i128, arg1 as i128);
        let orig = (arg0 * inv * m.abs()) + ofs;
        offset_a = orig % lcm;
        a = lcm;
    }
    offset_a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        939
        7,13,x,x,59,x,31,19
        ";
        let (ts, schedule) = parse(s);
        assert_eq!((59, 5), earliest(ts, &schedule));
        assert_eq!(1068781, earliest_offset(&schedule));
        Ok(())
    }
}

fn lcm(a: i128, b: i128) -> i128 {
    (a * b) / gcd(a, b)
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    let mut t;
    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }
    a
}

fn mod_inv(mut a: i128, mut m: i128) -> i128 {
    assert_eq!(1, gcd(a.abs(), m.abs()));
    let orig_m = m;
    let mut y = 0;
    let mut x = 1;

    if m == 1 {
        return 0;
    }
    while a > 1 {
        let q = a / m;
        let r = a % m;

        a = m;
        m = r;

        let ny = x - (q * y);
        x = y;
        y = ny;
    }

    if x < 0 {
        x += orig_m;
    }
    return x;
}
