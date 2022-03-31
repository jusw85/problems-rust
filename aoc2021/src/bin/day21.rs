// https://adventofcode.com/2021/day/21
//
// --- Day 21: Dirac Dice ---
//
// There's not much to do as you slowly descend to the bottom of the ocean. The submarine computer challenges you to a nice game of Dirac Dice.
//
// This game consists of a single die, two pawns, and a game board with a circular track containing ten spaces marked 1 through 10 clockwise. Each player's starting space is chosen randomly (your puzzle input). Player 1 goes first.
//
// Players take turns moving. On each player's turn, the player rolls the die three times and adds up the results. Then, the player moves their pawn that many times forward around the track (that is, moving clockwise on spaces in order of increasing value, wrapping back around to 1 after 10). So, if a player is on space 7 and they roll 2, 2, and 1, they would move forward 5 times, to spaces 8, 9, 10, 1, and finally stopping on 2.
//
// After each player moves, they increase their score by the value of the space their pawn stopped on. Players' scores start at 0. So, if the first player starts on space 7 and rolls a total of 5, they would stop on space 2 and add 2 to their score (for a total score of 2). The game immediately ends as a win for any player whose score reaches at least 1000.
//
// Since the first game is a practice game, the submarine opens a compartment labeled deterministic dice and a 100-sided die falls out. This die always rolls 1 first, then 2, then 3, and so on up to 100, after which it starts over at 1 again. Play using this die.
//
// For example, given these starting positions:
//
// Player 1 starting position: 4
// Player 2 starting position: 8
//
// This is how the game would go:
//
//     Player 1 rolls 1+2+3 and moves to space 10 for a total score of 10.
//     Player 2 rolls 4+5+6 and moves to space 3 for a total score of 3.
//     Player 1 rolls 7+8+9 and moves to space 4 for a total score of 14.
//     Player 2 rolls 10+11+12 and moves to space 6 for a total score of 9.
//     Player 1 rolls 13+14+15 and moves to space 6 for a total score of 20.
//     Player 2 rolls 16+17+18 and moves to space 7 for a total score of 16.
//     Player 1 rolls 19+20+21 and moves to space 6 for a total score of 26.
//     Player 2 rolls 22+23+24 and moves to space 6 for a total score of 22.
//
// ...after many turns...
//
//     Player 2 rolls 82+83+84 and moves to space 6 for a total score of 742.
//     Player 1 rolls 85+86+87 and moves to space 4 for a total score of 990.
//     Player 2 rolls 88+89+90 and moves to space 3 for a total score of 745.
//     Player 1 rolls 91+92+93 and moves to space 10 for a final score, 1000.
//
// Since player 1 has at least 1000 points, player 1 wins and the game ends. At this point, the losing player had 745 points and the die had been rolled a total of 993 times; 745 * 993 = 739785.
//
// Play a practice game using the deterministic 100-sided die. The moment either player wins, what do you get if you multiply the score of the losing player by the number of times the die was rolled during the game?
//
// Your puzzle answer was 518418.
// --- Part Two ---
//
// Now that you're warmed up, it's time to play the real game.
//
// A second compartment opens, this time labeled Dirac dice. Out of it falls a single three-sided die.
//
// As you experiment with the die, you feel a little strange. An informational brochure in the compartment explains that this is a quantum die: when you roll it, the universe splits into multiple copies, one copy for each possible outcome of the die. In this case, rolling the die always splits the universe into three copies: one where the outcome of the roll was 1, one where it was 2, and one where it was 3.
//
// The game is played the same as before, although to prevent things from getting too far out of hand, the game now ends when either player's score reaches at least 21.
//
// Using the same starting positions as in the example above, player 1 wins in 444356092776315 universes, while player 2 merely wins in 341960390180808 universes.
//
// Using your given starting positions, determine every possible outcome. Find the player that wins in more universes; in how many universes does that player win?
//
// Your puzzle answer was 116741133558209.

use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;
use std::iter::successors;

use anyhow::Result;

fn main() -> Result<()> {
    println!("{:?}", part1(8, 1));
    println!("{:?}", part2(8, 1));
    Ok(())
}

fn part1(p1: i32, p2: i32) -> i32 {
    let mut it = successors(Some(1), |i| Some((i % 100) + 1));
    let mut pos = [p1, p2];
    let mut scores = [0, 0];
    let mut count = 0;

    for i in successors(Some(0), |i| Some((i + 1) % 2)) {
        count += 3;
        let die = it.by_ref().take(3).sum::<i32>();
        let npos = (((pos[i] + die) - 1) % 10) + 1;
        scores[i] += npos;
        pos[i] = npos;
        if scores[i] >= 1000 {
            break;
        }
    }
    min(scores[0], scores[1]) * count
}

fn part2(p1: i32, p2: i32) -> u64 {
    let (p1_win, p1_lose) = count_ways(p1);
    let (p2_win, p2_lose) = count_ways(p2);

    let p2_vs_p1_win = p2_win.iter().map(|(k, v)| v * p1_lose.get(k).unwrap_or(&0)).sum::<u64>();
    let p1_vs_p2_win = p1_win.iter().map(|(k, v)| v * p2_lose.get(&(*k - 1)).unwrap_or(&0)).sum::<u64>();
    max(p1_vs_p2_win, p2_vs_p1_win)
}

fn count_ways(pos: i32) -> (HashMap<i32, u64>, HashMap<i32, u64>) {
    let mut hm = HashMap::new(); // win at roll k
    let mut hm2 = HashMap::new(); // did not win by roll k
    count_ways_rec(0, pos, 0, 1, &mut hm, &mut hm2);
    return (hm, hm2);

    fn count_ways_rec(score: i32, pos: i32, num_rolls: i32, acc: u64,
                      hm: &mut HashMap<i32, u64>,
                      hm2: &mut HashMap<i32, u64>, )
    {
        if score >= 21 {
            *hm.entry(num_rolls).or_insert(0) += acc;
            return;
        }
        *hm2.entry(num_rolls).or_insert(0) += acc;

        for (die, ways) in (3..=9).zip([1, 3, 6, 7, 6, 3, 1]) {
            let npos = (((pos + die) - 1) % 10) + 1;
            count_ways_rec(score + npos, npos, num_rolls + 1, ways * acc, hm, hm2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        assert_eq!(739785, part1(4, 8));
        assert_eq!(444356092776315, part2(4, 8));
        Ok(())
    }
}
