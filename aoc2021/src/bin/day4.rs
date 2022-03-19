// https://adventofcode.com/2021/day/4
//
// --- Day 4: Giant Squid ---
// 
// You're already almost 1.5km (almost a mile) below the surface of the ocean, already so deep that you can't see any sunlight. What you can see, however, is a giant squid that has attached itself to the outside of your submarine.
// 
// Maybe it wants to play bingo?
// 
// Bingo is played on a set of boards each consisting of a 5x5 grid of numbers. Numbers are chosen at random, and the chosen number is marked on all boards on which it appears. (Numbers may not appear on all boards.) If all numbers in any row or any column of a board are marked, that board wins. (Diagonals don't count.)
// 
// The submarine has a bingo subsystem to help passengers (currently, you and the giant squid) pass the time. It automatically generates a random order in which to draw numbers and a random set of boards (your puzzle input). For example:
// 
// 7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
// 
// 22 13 17 11  0
//  8  2 23  4 24
// 21  9 14 16  7
//  6 10  3 18  5
//  1 12 20 15 19
// 
//  3 15  0  2 22
//  9 18 13 17  5
// 19  8  7 25 23
// 20 11 10 24  4
// 14 21 16 12  6
// 
// 14 21 17 24  4
// 10 16 15  9 19
// 18  8 23 26 20
// 22 11 13  6  5
//  2  0 12  3  7
// 
// After the first five numbers are drawn (7, 4, 9, 5, and 11), there are no winners, but the boards are marked as follows (shown here adjacent to each other to save space):
// 
// 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
// 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
// 
// After the next six numbers are drawn (17, 23, 2, 0, 14, and 21), there are still no winners:
// 
// 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
// 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
// 
// Finally, 24 is drawn:
// 
// 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
// 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
// 
// At this point, the third board wins because it has at least one complete row or column of marked numbers (in this case, the entire top row is marked: 14 21 17 24 4).
// 
// The score of the winning board can now be calculated. Start by finding the sum of all unmarked numbers on that board; in this case, the sum is 188. Then, multiply that sum by the number that was just called when the board won, 24, to get the final score, 188 * 24 = 4512.
// 
// To guarantee victory against the giant squid, figure out which board will win first. What will your final score be if you choose that board?
// 
// Your puzzle answer was 34506.
// --- Part Two ---
// 
// On the other hand, it might be wise to try a different strategy: let the giant squid win.
// 
// You aren't sure how many bingo boards a giant squid could play at once, so rather than waste time counting its arms, the safe thing to do is to figure out which board will win last and choose that one. That way, no matter which boards it picks, it will win for sure.
// 
// In the above example, the second board is the last to win, which happens after 13 is eventually called and its middle column is completely marked. If you were to keep playing until this point, the second board would have a sum of unmarked numbers equal to 148 for a final score of 148 * 13 = 1924.
// 
// Figure out which board will win last. Once it wins, what would its final score be?
// 
// Your puzzle answer was 7686.

use std::collections::{HashMap, HashSet};
use std::fs;

use anyhow::Result;
use itertools::Itertools;

use aoc2021::TrimEmpty;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2021/day4")?;
    let (nums, mut boards) = parse(&input);
    println!("{:?}", solve(&nums, &mut boards));
    Ok(())
}

fn parse(s: &str) -> (Vec<u8>, Vec<Board>) {
    let mut iter = s.split("\n\n");
    let nums = iter.next().unwrap()
        .split(',').trim_empty()
        .map(|s| s.parse::<u8>().unwrap())
        .collect_vec();

    let boards = iter.map(|vals|
        Board::new(vals.split(char::is_whitespace).trim_empty()
            .enumerate()
            .map(|(idx, val)| (val.parse::<u8>().unwrap(), idx))
            .collect()))
        .collect_vec();
    (nums, boards)
}

fn solve(nums: &Vec<u8>, boards: &mut Vec<Board>) -> (u32, u32) {
    let mut first = None;
    let mut prev = None;

    for num in nums {
        for board in boards.iter_mut()
            .filter(|b| b.score.is_none())
        {
            board.mark(*num);

            if board.score.is_some() {
                if first.is_none() {
                    first = board.score;
                }
                prev = board.score;
            }
        }
    }
    (first.unwrap(), prev.unwrap())
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Board {
    vals: HashMap<u8, usize>,
    marked_positions: HashSet<usize>,
    score: Option<u32>,
}

impl Board {
    fn new(vals: HashMap<u8, usize>) -> Board {
        let marked_positions = HashSet::with_capacity(vals.len());
        Board { vals, marked_positions, score: None }
    }

    fn mark(&mut self, val: u8) {
        lazy_static::lazy_static! {
            static ref WINNING_POSITIONS: Vec<HashSet<usize>> =
                vec![
                    vec![0, 1, 2, 3, 4],
                    vec![5, 6, 7, 8, 9],
                    vec![10, 11, 12, 13, 14],
                    vec![15, 16, 17, 18, 19],
                    vec![20, 21, 22, 23, 24],
                    vec![0, 5, 10, 15, 20],
                    vec![1, 6, 11, 16, 21],
                    vec![2, 7, 12, 17, 22],
                    vec![3, 8, 13, 18, 23],
                    vec![4, 9, 14, 19, 24],
                ].into_iter().map(|c| {
                    c.into_iter().collect::<HashSet<usize>>()
                }).collect_vec();
        }

        if let (None, Some(pos)) = (self.score, self.vals.get(&val)) {
            self.marked_positions.insert(*pos);

            if WINNING_POSITIONS.iter()
                .any(|wp| wp.is_subset(&self.marked_positions))
            {
                let sum = self.vals.iter()
                    .filter_map(|(val, pos)|
                        (!self.marked_positions.contains(pos)).then(|| *val as u32))
                    .sum::<u32>();
                self.score = Some(sum * val as u32)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
        ";
        let (nums, mut boards) = parse(s);
        assert_eq!((4512, 1924), solve(&nums, &mut boards));
        Ok(())
    }
}
