// https://adventofcode.com/2020/day/16
//
// --- Day 16: Ticket Translation ---
//
// As you're walking to yet another connecting flight, you realize that one of the legs of your re-routed trip coming up is on a high-speed train. However, the train ticket you were given is in a language you don't understand. You should probably figure out what it says before you get to the train station after the next flight.
//
// Unfortunately, you can't actually read the words on the ticket. You can, however, read the numbers, and so you figure out the fields these tickets must have and the valid ranges for values in those fields.
//
// You collect the rules for ticket fields, the numbers on your ticket, and the numbers on other nearby tickets for the same train service (via the airport security cameras) together into a single document you can reference (your puzzle input).
//
// The rules for ticket fields specify a list of fields that exist somewhere on the ticket and the valid ranges of values for each field. For example, a rule like class: 1-3 or 5-7 means that one of the fields in every ticket is named class and can be any value in the ranges 1-3 or 5-7 (inclusive, such that 3 and 5 are both valid in this field, but 4 is not).
//
// Each ticket is represented by a single line of comma-separated values. The values are the numbers on the ticket in the order they appear; every ticket has the same format. For example, consider this ticket:
//
// .--------------------------------------------------------.
// | ????: 101    ?????: 102   ??????????: 103     ???: 104 |
// |                                                        |
// | ??: 301  ??: 302             ???????: 303      ??????? |
// | ??: 401  ??: 402           ???? ????: 403    ????????? |
// '--------------------------------------------------------'
//
// Here, ? represents text in a language you don't understand. This ticket might be represented as 101,102,103,104,301,302,303,401,402,403; of course, the actual train tickets you're looking at are much more complicated. In any case, you've extracted just the numbers in such a way that the first number is always the same specific field, the second number is always a different specific field, and so on - you just don't know what each position actually means!
//
// Start by determining which tickets are completely invalid; these are tickets that contain values which aren't valid for any field. Ignore your ticket for now.
//
// For example, suppose you have the following notes:
//
// class: 1-3 or 5-7
// row: 6-11 or 33-44
// seat: 13-40 or 45-50
//
// your ticket:
// 7,1,14
//
// nearby tickets:
// 7,3,47
// 40,4,50
// 55,2,20
// 38,6,12
//
// It doesn't matter which position corresponds to which field; you can identify invalid nearby tickets by considering only whether tickets contain values that are not valid for any field. In this example, the values on the first nearby ticket are all valid for at least one field. This is not true of the other three nearby tickets: the values 4, 55, and 12 are are not valid for any field. Adding together all of the invalid values produces your ticket scanning error rate: 4 + 55 + 12 = 71.
//
// Consider the validity of the nearby tickets you scanned. What is your ticket scanning error rate?
//
// Your puzzle answer was 21071.
// --- Part Two ---
//
// Now that you've identified which tickets contain invalid values, discard those tickets entirely. Use the remaining valid tickets to determine which field is which.
//
// Using the valid ranges for each field, determine what order the fields appear on the tickets. The order is consistent between all tickets: if seat is the third field, it is the third field on every ticket, including your ticket.
//
// For example, suppose you have the following notes:
//
// class: 0-1 or 4-19
// row: 0-5 or 8-19
// seat: 0-13 or 16-19
//
// your ticket:
// 11,12,13
//
// nearby tickets:
// 3,9,18
// 15,1,5
// 5,14,9
//
// Based on the nearby tickets in the above example, the first position must be row, the second position must be class, and the third position must be seat; you can conclude that in your ticket, class is 12, row is 11, and seat is 13.
//
// Once you work out which field is which, look for the six fields on your ticket that start with the word departure. What do you get if you multiply those six values together?
//
// Your puzzle answer was 3429967441937.

use std::collections::HashMap;
use std::fs;
use std::ops::Range;

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2020/day16")?;
    let (rules, my_ticket, mut all_tickets) = parse(&input);
    println!("{}", scanning_error_rate(&rules, &mut all_tickets));

    all_tickets.push(my_ticket.clone());
    let fields = solve_fields(&rules, &all_tickets);
    let product = fields.iter().enumerate()
        .filter(|(_, v)| v.starts_with("departure"))
        .map(|(i, _)| my_ticket[i] as u64)
        .product::<u64>();
    println!("{}", product);
    Ok(())
}

fn parse(s: &str) -> (HashMap<String, (Range<u32>, Range<u32>)>, Vec<u32>, Vec<Vec<u32>>) {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    }

    let mut sections = s.trim().split("\n\n");
    let rules = sections.next().unwrap();
    let rules = rules.lines()
        .map(|line| {
            let caps = RE.captures(line.trim()).unwrap();
            let category = caps[1].to_string();
            let from1 = caps[2].parse::<u32>().unwrap();
            let to1 = caps[3].parse::<u32>().unwrap();
            let from2 = caps[4].parse::<u32>().unwrap();
            let to2 = caps[5].parse::<u32>().unwrap();
            let range1 = from1..to1 + 1;
            let range2 = from2..to2 + 1;
            (category, (range1, range2))
        })
        .collect::<HashMap<_, _>>();

    fn parse_ticket(line: &str) -> Vec<u32> {
        line.trim().split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect_vec()
    }

    let mut my_ticket = sections.next().unwrap().lines();
    assert_eq!("your ticket:", my_ticket.next().unwrap().trim());
    let my_ticket = parse_ticket(my_ticket.next().unwrap());

    let mut all_tickets = sections.next().unwrap().lines();
    assert_eq!("nearby tickets:", all_tickets.next().unwrap().trim());
    let all_tickets = all_tickets.map(parse_ticket).collect_vec();

    (rules, my_ticket, all_tickets)
}

fn scanning_error_rate(rules: &HashMap<String, (Range<u32>, Range<u32>)>,
                       tickets: &mut Vec<Vec<u32>>) -> u32 {
    let ranges = rules.values()
        .fold(Vec::with_capacity(rules.len() * 2), |mut vec, ranges| {
            vec.push(ranges.0.clone());
            vec.push(ranges.1.clone());
            vec
        });

    let is_valid_field = |field| -> bool {
        ranges.iter().any(|range| range.contains(&field))
    };

    let sum = tickets.iter()
        .flat_map(|fields| fields.iter()
            .filter(|&&field| !is_valid_field(field)))
        .sum::<u32>();
    // nightly: drain_filter
    tickets.retain(|fields| !fields.iter()
        .any(|&field| !is_valid_field(field)));
    sum
}

fn solve_fields<'a>(rules: &'a HashMap<String, (Range<u32>, Range<u32>)>,
                    tickets: &Vec<Vec<u32>>) -> Vec<&'a String> {
    let possible_cats = |i| {
        rules.iter()
            .filter(|(_, (r1, r2))|
                tickets.iter()
                    .map(|fields| &fields[i])
                    .all(|v| { r1.contains(v) || r2.contains(v) }))
            .map(|(cat, _)| cat)
            .collect_vec()
    };

    let mut cats = (0..tickets[0].len())
        .map(possible_cats)
        .collect_vec();

    let mut to_process = cats.iter()
        .filter(|v| v.len() == 1)
        .map(|v| v[0])
        .collect_vec();
    while !to_process.is_empty() {
        let cat = to_process.pop().unwrap();
        for vec in cats.iter_mut().filter(|v| v.len() > 1) {
            if let Some(pos) = vec.iter().position(|x| *x == cat) {
                vec.remove(pos);
                if vec.len() == 1 {
                    to_process.push(vec[0]);
                }
            }
        }
    }
    cats.iter().map(|v| {
        assert_eq!(v.len(), 1);
        v[0]
    }).collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50

        your ticket:
        7,1,14

        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12
        ";
        let (rules, _, mut all_tickets) = parse(s);
        assert_eq!(71, scanning_error_rate(&rules, &mut all_tickets));

        let s = r"
        class: 0-1 or 4-19
        row: 0-5 or 8-19
        seat: 0-13 or 16-19

        your ticket:
        11,12,13

        nearby tickets:
        3,9,18
        15,1,5
        5,14,9
        ";
        let (rules, my_ticket, mut all_tickets) = parse(s);
        all_tickets.push(my_ticket.clone());
        let fields = solve_fields(&rules, &all_tickets);
        assert_eq!(vec!["row", "class", "seat"], fields);
        Ok(())
    }
}
