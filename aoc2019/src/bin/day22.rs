// https://adventofcode.com/2019/day/22
//
// --- Day 22: Slam Shuffle ---
//
// There isn't much to do while you wait for the droids to repair your ship. At least you're drifting in the right direction. You decide to practice a new card shuffle you've been working on.
//
// Digging through the ship's storage, you find a deck of space cards! Just like any deck of space cards, there are 10007 cards in the deck numbered 0 through 10006. The deck must be new - they're still in factory order, with 0 on the top, then 1, then 2, and so on, all the way through to 10006 on the bottom.
//
// You've been practicing three different techniques that you use while shuffling. Suppose you have a deck of only 10 cards (numbered 0 through 9):
//
// To deal into new stack, create a new stack of cards by dealing the top card of the deck onto the top of the new stack repeatedly until you run out of cards:
//
// Top          Bottom
// 0 1 2 3 4 5 6 7 8 9   Your deck
//                       New stack
//
//   1 2 3 4 5 6 7 8 9   Your deck
//                   0   New stack
//
//     2 3 4 5 6 7 8 9   Your deck
//                 1 0   New stack
//
//       3 4 5 6 7 8 9   Your deck
//               2 1 0   New stack
//
// Several steps later...
//
//                   9   Your deck
//   8 7 6 5 4 3 2 1 0   New stack
//
//                       Your deck
// 9 8 7 6 5 4 3 2 1 0   New stack
//
// Finally, pick up the new stack you've just created and use it as the deck for the next technique.
//
// To cut N cards, take the top N cards off the top of the deck and move them as a single unit to the bottom of the deck, retaining their order. For example, to cut 3:
//
// Top          Bottom
// 0 1 2 3 4 5 6 7 8 9   Your deck
//
//       3 4 5 6 7 8 9   Your deck
// 0 1 2                 Cut cards
//
// 3 4 5 6 7 8 9         Your deck
//               0 1 2   Cut cards
//
// 3 4 5 6 7 8 9 0 1 2   Your deck
//
// You've also been getting pretty good at a version of this technique where N is negative! In that case, cut (the absolute value of) N cards from the bottom of the deck onto the top. For example, to cut -4:
//
// Top          Bottom
// 0 1 2 3 4 5 6 7 8 9   Your deck
//
// 0 1 2 3 4 5           Your deck
//             6 7 8 9   Cut cards
//
//         0 1 2 3 4 5   Your deck
// 6 7 8 9               Cut cards
//
// 6 7 8 9 0 1 2 3 4 5   Your deck
//
// To deal with increment N, start by clearing enough space on your table to lay out all of the cards individually in a long line. Deal the top card into the leftmost position. Then, move N positions to the right and deal the next card there. If you would move into a position past the end of the space on your table, wrap around and keep counting from the leftmost card again. Continue this process until you run out of cards.
//
// For example, to deal with increment 3:
//
//
// 0 1 2 3 4 5 6 7 8 9   Your deck
// . . . . . . . . . .   Space on table
// ^                     Current position
//
// Deal the top card to the current position:
//
//   1 2 3 4 5 6 7 8 9   Your deck
// 0 . . . . . . . . .   Space on table
// ^                     Current position
//
// Move the current position right 3:
//
//   1 2 3 4 5 6 7 8 9   Your deck
// 0 . . . . . . . . .   Space on table
//       ^               Current position
//
// Deal the top card:
//
//     2 3 4 5 6 7 8 9   Your deck
// 0 . . 1 . . . . . .   Space on table
//       ^               Current position
//
// Move right 3 and deal:
//
//       3 4 5 6 7 8 9   Your deck
// 0 . . 1 . . 2 . . .   Space on table
//             ^         Current position
//
// Move right 3 and deal:
//
//         4 5 6 7 8 9   Your deck
// 0 . . 1 . . 2 . . 3   Space on table
//                   ^   Current position
//
// Move right 3, wrapping around, and deal:
//
//           5 6 7 8 9   Your deck
// 0 . 4 1 . . 2 . . 3   Space on table
//     ^                 Current position
//
// And so on:
//
// 0 7 4 1 8 5 2 9 6 3   Space on table
//
// Positions on the table which already contain cards are still counted; they're not skipped. Of course, this technique is carefully designed so it will never put two cards in the same position or leave a position empty.
//
// Finally, collect the cards on the table so that the leftmost card ends up at the top of your deck, the card to its right ends up just below the top card, and so on, until the rightmost card ends up at the bottom of the deck.
//
// The complete shuffle process (your puzzle input) consists of applying many of these techniques. Here are some examples that combine techniques; they all start with a factory order deck of 10 cards:
//
// deal with increment 7
// deal into new stack
// deal into new stack
// Result: 0 3 6 9 2 5 8 1 4 7
//
// cut 6
// deal with increment 7
// deal into new stack
// Result: 3 0 7 4 1 8 5 2 9 6
//
// deal with increment 7
// deal with increment 9
// cut -2
// Result: 6 3 0 7 4 1 8 5 2 9
//
// deal into new stack
// cut -2
// deal with increment 7
// cut 8
// cut -4
// deal with increment 7
// cut 3
// deal with increment 9
// deal with increment 3
// cut -1
// Result: 9 2 5 8 1 4 7 0 3 6
//
// Positions within the deck count from 0 at the top, then 1 for the card immediately below the top card, and so on to the bottom. (That is, cards start in the position matching their number.)
//
// After shuffling your factory order deck of 10007 cards, what is the position of card 2019?
//
// Your puzzle answer was 2480.
// --- Part Two ---
//
// After a while, you realize your shuffling skill won't improve much more with merely a single deck of cards. You ask every 3D printer on the ship to make you some more cards while you check on the ship repairs. While reviewing the work the droids have finished so far, you think you see Halley's Comet fly past!
//
// When you get back, you discover that the 3D printers have combined their power to create for you a single, giant, brand new, factory order deck of 119315717514047 space cards.
//
// Finally, a deck of cards worthy of shuffling!
//
// You decide to apply your complete shuffle process (your puzzle input) to the deck 101741582076661 times in a row.
//
// You'll need to be careful, though - one wrong move with this many cards and you might overflow your entire ship!
//
// After shuffling your new, giant, factory order deck that many times, what number is on the card that ends up in position 2020?
//
// Your puzzle answer was 62416301438548.

use std::collections::{HashSet, VecDeque};
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2019/day22")?;
    let moves = parse(&input)?;
    let simp_moves = simplify(&moves, 10007);

    let deck = shuffle(&simp_moves, 10007);
    let res = deck.iter().enumerate()
        .find(|(_i, &c)| c == 2019 as usize);
    println!("{:?}", res);

    let n = 119315717514047_i128;
    let moves = simplify(&moves, n);
    println!("{:?}", moves);
    // [DealInc(34300472370265), Cut(-13035406070497)]

    let times = 101741582076661_i128;
    let fold = fold(34300472370265, -13035406070497, n, times);
    println!("{:?}", fold);
    // (5606965185395, -13585759979799)

    let cut = rev_cut(n, 2020, -13585759979799);

    // https://math.stackexchange.com/questions/3968831/closed-form-for-a-cdot-x-bmod-n-r
    let res = (mod_inv(5606965185395, n) * cut) % n;
    println!("{}", res);
    Ok(())
}

// deal(x) cut(y) deal(p) cut(q)
// = deal(x) deal(p) cut((-y * (n - p)) % n) cut(q)
// = deal((x * p) % n)  cut((((-y * (n - p)) % n) + q) % n)
fn fold(arg1: i128, arg2: i128, n: i128, mut target: i128) -> (i128, i128) {
    let combine = |(x, y): (i128, i128),
                   (p, q): (i128, i128)| -> (i128, i128) {
        let arg1 = (x * p) % n;
        let arg2 = (((-y * (n - p)) % n) + q) % n;
        (arg1, arg2)
    };
    assert!(target > 0);

    let mut res = None;
    let mut init = false;
    let mut args = (arg1, arg2);
    while target != 0 {
        if !init {
            init = true;
        } else {
            args = combine(args, args);
        }
        if target % 2 != 0 {
            res = if res.is_none() {
                Some(args)
            } else {
                Some(combine(res.unwrap(), args))
            }
        }
        target >>= 1;
    }
    res.unwrap()
}

fn simplify(moves: &VecDeque<Move>, n: i128) -> VecDeque<Move> {
    let mut moves = moves.clone();
    let mut res = VecDeque::new();
    let mut changed = true;

    while changed {
        changed = false;
        res.push_back(moves.pop_front().unwrap());
        while !moves.is_empty() {
            let next_move = moves.pop_front().unwrap();
            let prev_move = res.pop_back().unwrap();
            match (prev_move, next_move) {
                (Move::Cut(arg1), Move::DealInc(arg2)) => {
                    res.push_back(next_move);
                    let arg = (-arg1 * (n - arg2)) % n;
                    res.push_back(Move::Cut(arg));
                    changed = true;
                }
                (Move::DealInc(arg1), Move::DealInc(arg2)) => {
                    let arg = (arg1 * arg2) % n;
                    res.push_back(Move::DealInc(arg));
                    changed = true;
                }
                (Move::Cut(arg1), Move::Cut(arg2)) => {
                    let arg = (arg1 + arg2) % n;
                    res.push_back(Move::Cut(arg));
                    changed = true;
                }
                (Move::Cut(arg1), Move::DealNew) => {
                    res.push_back(Move::DealNew);
                    res.push_back(Move::Cut(-arg1));
                    changed = true;
                }
                (Move::DealInc(arg1), Move::DealNew) => {
                    res.push_back(Move::DealInc(n - arg1));
                    res.push_back(Move::Cut(1));
                    changed = true;
                }
                (Move::DealNew, Move::DealNew) => {
                    changed = true;
                }
                (_, _) => {
                    res.push_back(prev_move);
                    res.push_back(next_move);
                }
            }
        }
        let t = res;
        res = moves;
        moves = t;
    }
    moves
}

// doesn't work
#[allow(dead_code)]
fn find_loop(moves: &VecDeque<Move>) {
    let mut set = HashSet::new();
    let deck_size = 119315717514047_i128;
    let mut pos = 2020_i128;

    set.insert(pos);
    loop {
        let prev_pos = rev(&moves, deck_size, pos);
        if set.contains(&prev_pos) {
            println!("found! {}", set.len());
            break;
        }
        set.insert(prev_pos);
        pos = prev_pos;
        if set.len() % 100_000 == 0 {
            println!("{}", set.len());
        }
    }
}

fn shuffle(moves: &VecDeque<Move>, size: usize) -> Vec<usize> {
    let mut deck = vec![0usize; size];
    for (i, c) in deck.iter_mut().enumerate() {
        *c = i;
    }
    for mov in moves {
        match mov {
            Move::Cut(arg) => cut(&mut deck, *arg),
            Move::DealInc(arg) => deal_inc(&mut deck, *arg),
            Move::DealNew => deck.reverse()
        }
    }
    deck
}

fn cut(deck: &mut Vec<usize>, num: i128) {
    if num > 0 {
        deck.rotate_left(num as usize);
    } else {
        deck.rotate_right(-num as usize);
    }
}

fn deal_inc(deck: &mut Vec<usize>, num: i128) {
    let orig = deck.clone();
    let mut i = 0;
    for c in orig {
        deck[i] = c;
        i = (i + num as usize) % deck.len();
    }
}

#[allow(dead_code)]
fn rev(moves: &VecDeque<Move>, size: i128, mut pos: i128) -> i128 {
    for mov in moves.iter().rev() {
        pos = match mov {
            Move::Cut(arg) => rev_cut(size, pos, *arg as i128),
            Move::DealInc(arg) => rev_deal_inc(size, pos, *arg as i128),
            Move::DealNew => rev_deal_new(size, pos),
        }
    }
    pos
}

#[allow(dead_code)]
fn rev_deal_new(n: i128, i: i128) -> i128 {
    n - 1 - i
}

#[allow(dead_code)]
fn rev_cut(n: i128, i: i128, cut: i128) -> i128 {
    if cut > 0 {
        (cut + i) % n
    } else {
        (n + cut + i) % n
    }
}

#[allow(dead_code)]
fn rev_deal_inc(n: i128, mut i: i128, inc: i128) -> i128 {
    let mut res = 0;
    loop {
        res += i / inc;
        let r = i % inc;
        if r == 0 {
            break;
        }
        res += 1;
        i = n - (inc - r);
    }
    res
}

fn parse(s: &str) -> Result<VecDeque<Move>> {
    const P1: &str = r"cut (-?\d+)";
    const P2: &str = r"deal with increment (\d+)";
    const P3: &str = r"deal into new stack";
    lazy_static! {
        static ref P1_RE: Regex = Regex::new(P1).unwrap();
        static ref P2_RE: Regex = Regex::new(P2).unwrap();
        static ref P3_RE: Regex = Regex::new(P3).unwrap();
    }
    let mut moves = VecDeque::new();
    for line in s.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty()) {
        let mov = if P1_RE.is_match(line) {
            let arg = &P1_RE.captures(line).unwrap()[1];
            let arg = arg.parse::<i128>().unwrap();
            Move::Cut(arg)
        } else if P2_RE.is_match(line) {
            let arg = &P2_RE.captures(line).unwrap()[1];
            let arg = arg.parse::<i128>().unwrap();
            Move::DealInc(arg)
        } else if P3_RE.is_match(line) {
            Move::DealNew
        } else {
            anyhow::bail!("invalid line: {}", s);
        };
        moves.push_back(mov);
    }
    Ok(moves)
}

#[derive(Debug, Copy, Clone)]
enum Move {
    Cut(i128),
    DealInc(i128),
    DealNew,
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    let mut t;
    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }
    a
}

fn mod_inv(mut a: i128, mut m: i128) -> i128 {
    assert_eq!(1, gcd(a.abs() as u128, m.abs() as u128));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        deal with increment 7
        deal into new stack
        deal into new stack
        ";
        let moves = parse(s)?;
        let deck = shuffle(&moves, 10);
        assert_eq!(vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7], deck);

        let s = r"
        cut 6
        deal with increment 7
        deal into new stack
        ";
        let moves = parse(s)?;
        let deck = shuffle(&moves, 10);
        assert_eq!(vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6], deck);

        let s = r"
        deal with increment 7
        deal with increment 9
        cut -2
        ";
        let moves = parse(s)?;
        let deck = shuffle(&moves, 10);
        assert_eq!(vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9], deck);

        let s = r"
        deal into new stack
        cut -2
        deal with increment 7
        cut 8
        cut -4
        deal with increment 7
        cut 3
        deal with increment 9
        deal with increment 3
        cut -1
        ";
        let moves = parse(s)?;
        let deck = shuffle(&moves, 10);
        assert_eq!(vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6], deck);
        Ok(())
    }

    #[test]
    fn test2() -> Result<()> {
        let s = r"
        deal into new stack
        cut -2
        deal with increment 7
        cut 8
        cut -4
        deal with increment 7
        cut 3
        deal with increment 9
        deal with increment 3
        cut -1
        ";
        let moves = parse(s)?;
        let deck = (0..10).map(|i| rev(&moves, 10, i)).collect::<Vec<_>>();
        assert_eq!(vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6], deck);
        Ok(())
    }

    #[test]
    #[ignore]
    // cut a, cut b == cut (a + b)
    fn test3() -> Result<()> {
        let s = r"
        cut 1
        cut 1
        ";
        println!("{:?}", shuffle(&parse(s)?, 10));

        let s = r"
        cut 2
        ";
        println!("{:?}", shuffle(&parse(s)?, 10));

        let s = r"
        cut 1
        cut -1
        ";
        println!("{:?}", shuffle(&parse(s)?, 10));
        Ok(())
    }

    #[test]
    #[ignore]
    // cut 1, dealnew == dealnew, cut -1
    fn test4() -> Result<()> {
        let s = r"
        cut 1
        deal into new stack
        ";
        println!("{:?}", shuffle(&parse(s)?, 10));

        let s = r"
        deal into new stack
        cut -1
        ";
        println!("{:?}", shuffle(&parse(s)?, 10));
        Ok(())
    }

    #[test]
    #[ignore]
    // deal_inc x
    // ==
    // cut 1
    // deal_inc x
    // cut n - x
    // ==
    // cut k
    // deal_inc x
    // cut (k(n - x)) % n
    fn test5() -> Result<()> {
        let size = 13; // prime
        for i in 2..size { // coprime
            let s = format!(r"
            deal with increment {}", i);
            let deck1 = shuffle(&parse(&s)?, size);

            let s = format!(r"
            cut 1
            deal with increment {}
            cut {}", i, size - i);
            let deck2 = shuffle(&parse(&s)?, size);
            assert_eq!(deck1, deck2)
        }

        let s = r"
        deal with increment 7
        ";
        println!("{:?}", shuffle(&parse(s)?, 13));

        let s = r"
        cut 1
        deal with increment 7
        cut 6
        ";
        println!("{:?}", shuffle(&parse(s)?, 13));

        let s = r"
        cut 2
        deal with increment 7
        cut 12
        ";
        println!("{:?}", shuffle(&parse(s)?, 13));

        let s = r"
        cut 3
        deal with increment 7
        cut 5
        ";
        println!("{:?}", shuffle(&parse(s)?, 13));

        Ok(())
    }

    #[test]
    #[ignore]
    fn test6() -> Result<()> {
        let s = r"
        deal with increment 3
        ";
        println!("{:?}", shuffle(&parse(s)?, 13));

        let s = r"
        deal with increment 10
        deal into new stack
        cut -1
        ";
        println!("{:?}", shuffle(&parse(s)?, 13));
        Ok(())
    }

    #[test]
    #[ignore]
    fn test7() -> Result<()> {
        let s = r"
        deal with increment 3
        deal with increment 4
        ";
        println!("{:?}", shuffle(&parse(s)?, 13));

        let s = r"
        deal with increment 12
        ";
        println!("{:?}", shuffle(&parse(s)?, 13));

        let s = r"
        deal with increment 3
        deal with increment 5
        ";
        println!("{:?}", shuffle(&parse(s)?, 13));

        let s = r"
        deal with increment 2
        ";
        println!("{:?}", shuffle(&parse(s)?, 13));
        Ok(())
    }
}
