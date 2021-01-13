// https://adventofcode.com/2020/day/18
//
// --- Day 18: Operation Order ---
//
// As you look out the window and notice a heavily-forested continent slowly appear over the horizon, you are interrupted by the child sitting next to you. They're curious if you could help them with their math homework.
//
// Unfortunately, it seems like this "math" follows different rules than you remember.
//
// The homework (your puzzle input) consists of a series of expressions that consist of addition (+), multiplication (*), and parentheses ((...)). Just like normal math, parentheses indicate that the expression inside must be evaluated before it can be used by the surrounding expression. Addition still finds the sum of the numbers on both sides of the operator, and multiplication still finds the product.
//
// However, the rules of operator precedence have changed. Rather than evaluating multiplication before addition, the operators have the same precedence, and are evaluated left-to-right regardless of the order in which they appear.
//
// For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are as follows:
//
// 1 + 2 * 3 + 4 * 5 + 6
//   3   * 3 + 4 * 5 + 6
//       9   + 4 * 5 + 6
//          13   * 5 + 6
//              65   + 6
//                  71
//
// Parentheses can override this order; for example, here is what happens if parentheses are added to form 1 + (2 * 3) + (4 * (5 + 6)):
//
// 1 + (2 * 3) + (4 * (5 + 6))
// 1 +    6    + (4 * (5 + 6))
//      7      + (4 * (5 + 6))
//      7      + (4 *   11   )
//      7      +     44
//             51
//
// Here are a few more examples:
//
//     2 * 3 + (4 * 5) becomes 26.
//     5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 437.
//     5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 12240.
//     ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 13632.
//
// Before you can help with the homework, you need to understand it yourself. Evaluate the expression on each line of the homework; what is the sum of the resulting values?
//
// Your puzzle answer was 98621258158412.
// --- Part Two ---
//
// You manage to answer the child's questions and they finish part 1 of their homework, but get stuck when they reach the next section: advanced math.
//
// Now, addition and multiplication have different precedence levels, but they're not the ones you're familiar with. Instead, addition is evaluated before multiplication.
//
// For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are now as follows:
//
// 1 + 2 * 3 + 4 * 5 + 6
//   3   * 3 + 4 * 5 + 6
//   3   *   7   * 5 + 6
//   3   *   7   *  11
//      21       *  11
//          231
//
// Here are the other examples from above:
//
//     1 + (2 * 3) + (4 * (5 + 6)) still becomes 51.
//     2 * 3 + (4 * 5) becomes 46.
//     5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 1445.
//     5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 669060.
//     ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 23340.
//
// What do you get if you add up the results of evaluating the homework problems using these new rules?
//
// Your puzzle answer was 241216538527890.

use std::fs;

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2020/day18")?;
    let exprs = parse(&input);
    let sum = exprs.iter().map(|expr| eval(expr, false)).sum::<u64>();
    println!("{}", sum);
    let sum = exprs.iter().map(|expr| eval(expr, true)).sum::<u64>();
    println!("{}", sum);
    Ok(())
}

fn parse(s: &str) -> Vec<String> {
    s.lines()
        .map(|line| {
            let mut s = line.to_string();
            s.retain(|c| !c.is_whitespace());
            s
        })
        .filter(|line| !line.is_empty())
        .collect_vec()
}

fn eval(expr: &str, add_precendence: bool) -> u64 {
    let mut operands = Vec::new();
    let mut operators = Vec::new();

    fn eval1(operands: &mut Vec<u64>, operators: &mut Vec<char>) {
        let arg1 = operands.pop().unwrap();
        let arg2 = operands.pop().unwrap();
        let res = match operators.pop().unwrap() {
            '+' => arg1 + arg2,
            '*' => arg1 * arg2,
            _ => panic!("unrecognized")
        };
        operands.push(res);
    }

    for c in expr.chars() {
        if let Some(d) = c.to_digit(10) {
            operands.push(d as u64);
        } else if operators.is_empty() || c == '(' {
            operators.push(c);
        } else if c == ')' {
            while *operators.last().unwrap() != '(' {
                eval1(&mut operands, &mut operators);
            }
            operators.pop();
        } else {
            let last = *operators.last().unwrap();
            if last != '('
                && !(add_precendence && last == '*' && c == '+')
            {
                eval1(&mut operands, &mut operators);
            }
            operators.push(c);
        }
    }

    while !operators.is_empty() {
        eval1(&mut operands, &mut operators);
    }
    assert_eq!(operands.len(), 1);
    operands.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        2 * 3 + (4 * 5)
        5 + (8 * 3 + 9 + 3 * 4 * 3)
        5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
        ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
        ";
        let exprs = parse(s);
        let vals = exprs.iter().map(|expr| eval(expr, false)).collect_vec();
        assert_eq!(vec![26, 437, 12240, 13632], vals);

        let s = r"
        1 + (2 * 3) + (4 * (5 + 6))
        2 * 3 + (4 * 5)
        5 + (8 * 3 + 9 + 3 * 4 * 3)
        5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
        ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
        ";
        let exprs = parse(s);
        let vals = exprs.iter().map(|expr| eval(expr, true)).collect_vec();
        assert_eq!(vec![51, 46, 1445, 669060, 23340], vals);
        Ok(())
    }
}
