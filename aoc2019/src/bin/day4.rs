// https://adventofcode.com/2019/day/4
//
// --- Day 4: Secure Container ---
//
// You arrive at the Venus fuel depot only to discover it's protected by a password. The Elves had written the password on a sticky note, but someone threw it out.
//
// However, they do remember a few key facts about the password:
//
//     It is a six-digit number.
//     The value is within the range given in your puzzle input.
//     Two adjacent digits are the same (like 22 in 122345).
//     Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
//
// Other than the range rule, the following are true:
//
//     111111 meets these criteria (double 11, never decreases).
//     223450 does not meet these criteria (decreasing pair of digits 50).
//     123789 does not meet these criteria (no double).
//
// How many different passwords within the range given in your puzzle input meet these criteria?
//
// Your puzzle answer was 2150.
// --- Part Two ---
//
// An Elf just remembered one more important detail: the two adjacent matching digits are not part of a larger group of matching digits.
//
// Given this additional criterion, but still ignoring the range rule, the following are now true:
//
//     112233 meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
//     123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444).
//     111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).
//
// How many different passwords within the range given in your puzzle input meet all of the criteria?
//
// Your puzzle answer was 1462.

fn main() {
    let p1 = (124075..=580769).filter(|&i| is_valid_part1(i)).count();
    let p2 = (124075..=580769).filter(|&i| is_valid_part2(i)).count();
    println!("{}", p1);
    println!("{}", p2);
}

fn is_valid_part1(num: i32) -> bool {
    if num < 100000 || num > 999999 {
        return false;
    }
    let mut num = num;
    let mut prev = i32::MAX;
    let mut has_repeat = false;

    while num > 0 {
        let next = num % 10;
        if next > prev {
            return false;
        }
        if next == prev {
            has_repeat = true;
        }
        prev = next;
        num = num / 10;
    }
    has_repeat
}

fn is_valid_part2(num: i32) -> bool {
    if num < 100000 || num > 999999 {
        return false;
    }
    let mut num = num;
    let mut prev = i32::MAX;
    let mut repeat_count = 0;
    let mut has_repeat_twice = false;

    while num > 0 {
        let next = num % 10;
        if next > prev {
            return false;
        }
        if next == prev {
            repeat_count += 1;
        } else {
            if repeat_count == 1 {
                has_repeat_twice = true;
            }
            repeat_count = 0;
        }
        prev = next;
        num = num / 10;
    }
    has_repeat_twice || repeat_count == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(is_valid_part1(111111), true);
        assert_eq!(is_valid_part1(122345), true);
        assert_eq!(is_valid_part1(223450), false);
        assert_eq!(is_valid_part1(123789), false);
    }

    #[test]
    fn test2() {
        assert_eq!(is_valid_part2(111111), false);
        assert_eq!(is_valid_part2(112233), true);
        assert_eq!(is_valid_part2(123444), false);
        assert_eq!(is_valid_part2(111122), true);
        assert_eq!(is_valid_part2(112222), true);
    }
}
