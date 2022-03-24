// https://adventofcode.com/2021/day/18
//
// --- Day 18: Snailfish ---
//
// You descend into the ocean trench and encounter some snailfish. They say they saw the sleigh keys! They'll even tell you which direction the keys went if you help one of the smaller snailfish with his math homework.
//
// Snailfish numbers aren't like regular numbers. Instead, every snailfish number is a pair - an ordered list of two elements. Each element of the pair can be either a regular number or another pair.
//
// Pairs are written as [x,y], where x and y are the elements within the pair. Here are some example snailfish numbers, one snailfish number per line:
//
// [1,2]
// [[1,2],3]
// [9,[8,7]]
// [[1,9],[8,5]]
// [[[[1,2],[3,4]],[[5,6],[7,8]]],9]
// [[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
// [[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]
//
// This snailfish homework is about addition. To add two snailfish numbers, form a pair from the left and right parameters of the addition operator. For example, [1,2] + [[3,4],5] becomes [[1,2],[[3,4],5]].
//
// There's only one problem: snailfish numbers must always be reduced, and the process of adding two snailfish numbers can result in snailfish numbers that need to be reduced.
//
// To reduce a snailfish number, you must repeatedly do the first action in this list that applies to the snailfish number:
//
//     If any pair is nested inside four pairs, the leftmost such pair explodes.
//     If any regular number is 10 or greater, the leftmost such regular number splits.
//
// Once no action in the above list applies, the snailfish number is reduced.
//
// During reduction, at most one action applies, after which the process returns to the top of the list of actions. For example, if split produces a pair that meets the explode criteria, that pair explodes before other splits occur.
//
// To explode a pair, the pair's left value is added to the first regular number to the left of the exploding pair (if any), and the pair's right value is added to the first regular number to the right of the exploding pair (if any). Exploding pairs will always consist of two regular numbers. Then, the entire exploding pair is replaced with the regular number 0.
//
// Here are some examples of a single explode action:
//
//     [[[[[9,8],1],2],3],4] becomes [[[[0,9],2],3],4] (the 9 has no regular number to its left, so it is not added to any regular number).
//     [7,[6,[5,[4,[3,2]]]]] becomes [7,[6,[5,[7,0]]]] (the 2 has no regular number to its right, and so it is not added to any regular number).
//     [[6,[5,[4,[3,2]]]],1] becomes [[6,[5,[7,0]]],3].
//     [[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]] becomes [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]] (the pair [3,2] is unaffected because the pair [7,3] is further to the left; [3,2] would explode on the next action).
//     [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]] becomes [[3,[2,[8,0]]],[9,[5,[7,0]]]].
//
// To split a regular number, replace it with a pair; the left element of the pair should be the regular number divided by two and rounded down, while the right element of the pair should be the regular number divided by two and rounded up. For example, 10 becomes [5,5], 11 becomes [5,6], 12 becomes [6,6], and so on.
//
// Here is the process of finding the reduced result of [[[[4,3],4],4],[7,[[8,4],9]]] + [1,1]:
//
// after addition: [[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]
// after explode:  [[[[0,7],4],[7,[[8,4],9]]],[1,1]]
// after explode:  [[[[0,7],4],[15,[0,13]]],[1,1]]
// after split:    [[[[0,7],4],[[7,8],[0,13]]],[1,1]]
// after split:    [[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]
// after explode:  [[[[0,7],4],[[7,8],[6,0]]],[8,1]]
//
// Once no reduce actions apply, the snailfish number that remains is the actual result of the addition operation: [[[[0,7],4],[[7,8],[6,0]]],[8,1]].
//
// The homework assignment involves adding up a list of snailfish numbers (your puzzle input). The snailfish numbers are each listed on a separate line. Add the first snailfish number and the second, then add that result and the third, then add that result and the fourth, and so on until all numbers in the list have been used once.
//
// For example, the final sum of this list is [[[[1,1],[2,2]],[3,3]],[4,4]]:
//
// [1,1]
// [2,2]
// [3,3]
// [4,4]
//
// The final sum of this list is [[[[3,0],[5,3]],[4,4]],[5,5]]:
//
// [1,1]
// [2,2]
// [3,3]
// [4,4]
// [5,5]
//
// The final sum of this list is [[[[5,0],[7,4]],[5,5]],[6,6]]:
//
// [1,1]
// [2,2]
// [3,3]
// [4,4]
// [5,5]
// [6,6]
//
// Here's a slightly larger example:
//
// [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
// [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
// [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
// [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
// [7,[5,[[3,8],[1,4]]]]
// [[2,[2,2]],[8,[8,1]]]
// [2,9]
// [1,[[[9,3],9],[[9,0],[0,7]]]]
// [[[5,[7,4]],7],1]
// [[[[4,2],2],6],[8,7]]
//
// The final sum [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]] is found after adding up the above snailfish numbers:
//
//   [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
// + [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
// = [[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]
//
//   [[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]
// + [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
// = [[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]
//
//   [[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]
// + [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
// = [[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]
//
//   [[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]
// + [7,[5,[[3,8],[1,4]]]]
// = [[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]
//
//   [[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]
// + [[2,[2,2]],[8,[8,1]]]
// = [[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]
//
//   [[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]
// + [2,9]
// = [[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]
//
//   [[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]
// + [1,[[[9,3],9],[[9,0],[0,7]]]]
// = [[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]
//
//   [[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]
// + [[[5,[7,4]],7],1]
// = [[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]
//
//   [[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]
// + [[[[4,2],2],6],[8,7]]
// = [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]
//
// To check whether it's the right answer, the snailfish teacher only checks the magnitude of the final sum. The magnitude of a pair is 3 times the magnitude of its left element plus 2 times the magnitude of its right element. The magnitude of a regular number is just that number.
//
// For example, the magnitude of [9,1] is 3*9 + 2*1 = 29; the magnitude of [1,9] is 3*1 + 2*9 = 21. Magnitude calculations are recursive: the magnitude of [[9,1],[1,9]] is 3*29 + 2*21 = 129.
//
// Here are a few more magnitude examples:
//
//     [[1,2],[[3,4],5]] becomes 143.
//     [[[[0,7],4],[[7,8],[6,0]]],[8,1]] becomes 1384.
//     [[[[1,1],[2,2]],[3,3]],[4,4]] becomes 445.
//     [[[[3,0],[5,3]],[4,4]],[5,5]] becomes 791.
//     [[[[5,0],[7,4]],[5,5]],[6,6]] becomes 1137.
//     [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]] becomes 3488.
//
// So, given this example homework assignment:
//
// [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
// [[[5,[2,8]],4],[5,[[9,9],0]]]
// [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
// [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
// [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
// [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
// [[[[5,4],[7,7]],8],[[8,3],8]]
// [[9,3],[[9,9],[6,[4,9]]]]
// [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
// [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
//
// The final sum is:
//
// [[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]
//
// The magnitude of this final sum is 4140.
//
// Add up all of the snailfish numbers from the homework assignment in the order they appear. What is the magnitude of the final sum?
//
// Your puzzle answer was 3647.
// --- Part Two ---
//
// You notice a second question on the back of the homework assignment:
//
// What is the largest magnitude you can get from adding only two of the snailfish numbers?
//
// Note that snailfish addition is not commutative - that is, x + y and y + x can produce different results.
//
// Again considering the last example homework assignment above:
//
// [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
// [[[5,[2,8]],4],[5,[[9,9],0]]]
// [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
// [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
// [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
// [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
// [[[[5,4],[7,7]],8],[[8,3],8]]
// [[9,3],[[9,9],[6,[4,9]]]]
// [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
// [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
//
// The largest magnitude of the sum of any two snailfish numbers in this list is 3993. This is the magnitude of [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]] + [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]], which reduces to [[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]].
//
// What is the largest magnitude of any sum of two different snailfish numbers from the homework assignment?
//
// Your puzzle answer was 4600.

use std::fs;
use std::ops::Range;

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

use aoc2021::TrimEmpty;

// optimisation:
// store as vec[(num, depth)]
// all input depth <= 3
// addition = concat(v1, v2), increase all depth by 1
// explosion = [.. (l, _), (n, d) (r, _) ..] => [.. (l+n, _), (0, d-1), (r+n, _) ..]
// continuous explosion will force all depth to < 4
// split = [.. (n, d) ..] => [.. (n/2, d+1), (n+1/2, d+1) ..]
// split will increase depth to maximum 4
// [.. (_, max depth), (_, max depth) ..] is guaranteed to be a pair
// leftmost pair = leftmost max_depth, max_depth will be <= 4
fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2021/day18")?;
    let v = parse(&input);
    println!("{:?}", part1(&v));
    println!("{:?}", part2(&v));
    Ok(())
}

fn parse(s: &str) -> Vec<String> {
    s.lines().trim_empty().map(|s| s.to_string()).collect_vec()
}

lazy_static::lazy_static! {
    static ref PAIR: Regex = Regex::new(r"\[(\d+),(\d+)\]").unwrap();
    static ref NUM: Regex = Regex::new(r"(\d+)").unwrap();
    static ref BIGNUM: Regex = Regex::new(r"(\d\d+)").unwrap();
}

fn part1(v: &Vec<String>) -> u32 {
    let mut s = v[1..].iter()
        .fold(v[0].clone(), |acc, e| combine(&acc, e));
    magnitude(&mut s)
}

fn part2(v: &Vec<String>) -> u32 {
    v.iter().cartesian_product(v.iter()).map(|(s1, s2)| {
        let mut s = combine(s1, s2);
        magnitude(&mut s)
    }).max().unwrap()
}

fn combine(s1: &str, s2: &str) -> String {
    let mut s = format!("[{},{}]", s1, s2);
    reduce(&mut s);
    s
}

fn magnitude(s: &mut String) -> u32 {
    while let Some(cap) = PAIR.captures(s) {
        let range = cap.get(0).unwrap().range();
        let n1 = cap[1].parse::<u32>().unwrap();
        let n2 = cap[2].parse::<u32>().unwrap();
        let n = (n1 * 3) + (n2 * 2);
        s.replace_range(range, &n.to_string());
    }
    s.parse().unwrap()
}

fn reduce(s: &mut String) {
    loop {
        if let Some(explosion) = find_explosion(s) {
            explode(s, explosion);
            continue;
        }
        if let Some(mat) = BIGNUM.find(s) {
            let n = mat.as_str().parse().unwrap();
            let range = mat.range();
            split(s, n, range);
            continue;
        }
        break;
    }
}

fn split(s: &mut String, n: u32, range: Range<usize>) {
    let (n1, n2) = (n / 2, (n + 1) / 2);
    let pair = format!("[{},{}]", n1, n2);
    s.replace_range(range, &pair);
}

fn explode(s: &mut String, explosion: (u32, u32, usize, usize)) {
    let (n1, n2, start, end) = explosion;
    if let Some(mat) = NUM.find_at(s, end) {
        let r_num = n2 + mat.as_str().parse::<u32>().unwrap();
        let range = mat.range();
        s.replace_range(range, &r_num.to_string());
    }

    s.replace_range(start..end, "0");

    if let Some(mat) = NUM.find_iter(&s[..start]).last() {
        let l_num = n1 + mat.as_str().parse::<u32>().unwrap();
        let range = mat.range();
        s.replace_range(range, &l_num.to_string());
    }
}

fn find_explosion(s: &str) -> Option<(u32, u32, usize, usize)> {
    for cap in PAIR.captures_iter(s) {
        let mat = cap.get(0).unwrap();
        let depth = s[..mat.start()].chars().fold(0, |acc, c|
            match c {
                '[' => acc + 1,
                ']' => acc - 1,
                _ => acc,
            });
        if depth == 4 {
            let n1 = cap[1].parse().unwrap();
            let n2 = cap[2].parse().unwrap();
            return Some((n1, n2, mat.start(), mat.end()));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
        ";
        let v = parse(s);
        assert_eq!(4140, part1(&v));
        assert_eq!(3993, part2(&v));
        Ok(())
    }
}
