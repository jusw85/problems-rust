// https://adventofcode.com/2021/day/12
//
// --- Day 12: Passage Pathing ---
//
// With your submarine's subterranean subsystems subsisting suboptimally, the only way you're getting out of this cave anytime soon is by finding a path yourself. Not just a path - the only way to know if you've found the best path is to find all of them.
//
// Fortunately, the sensors are still mostly working, and so you build a rough map of the remaining caves (your puzzle input). For example:
//
// start-A
// start-b
// A-c
// A-b
// b-d
// A-end
// b-end
//
// This is a list of how all of the caves are connected. You start in the cave named start, and your destination is the cave named end. An entry like b-d means that cave b is connected to cave d - that is, you can move between them.
//
// So, the above cave system looks roughly like this:
//
//     start
//     /   \
// c--A-----b--d
//     \   /
//      end
//
// Your goal is to find the number of distinct paths that start at start, end at end, and don't visit small caves more than once. There are two types of caves: big caves (written in uppercase, like A) and small caves (written in lowercase, like b). It would be a waste of time to visit any small cave more than once, but big caves are large enough that it might be worth visiting them multiple times. So, all paths you find should visit small caves at most once, and can visit big caves any number of times.
//
// Given these rules, there are 10 paths through this example cave system:
//
// start,A,b,A,c,A,end
// start,A,b,A,end
// start,A,b,end
// start,A,c,A,b,A,end
// start,A,c,A,b,end
// start,A,c,A,end
// start,A,end
// start,b,A,c,A,end
// start,b,A,end
// start,b,end
//
// (Each line in the above list corresponds to a single path; the caves visited by that path are listed in the order they are visited and separated by commas.)
//
// Note that in this cave system, cave d is never visited by any path: to do so, cave b would need to be visited twice (once on the way to cave d and a second time when returning from cave d), and since cave b is small, this is not allowed.
//
// Here is a slightly larger example:
//
// dc-end
// HN-start
// start-kj
// dc-start
// dc-HN
// LN-dc
// HN-end
// kj-sa
// kj-HN
// kj-dc
//
// The 19 paths through it are as follows:
//
// start,HN,dc,HN,end
// start,HN,dc,HN,kj,HN,end
// start,HN,dc,end
// start,HN,dc,kj,HN,end
// start,HN,end
// start,HN,kj,HN,dc,HN,end
// start,HN,kj,HN,dc,end
// start,HN,kj,HN,end
// start,HN,kj,dc,HN,end
// start,HN,kj,dc,end
// start,dc,HN,end
// start,dc,HN,kj,HN,end
// start,dc,end
// start,dc,kj,HN,end
// start,kj,HN,dc,HN,end
// start,kj,HN,dc,end
// start,kj,HN,end
// start,kj,dc,HN,end
// start,kj,dc,end
//
// Finally, this even larger example has 226 paths through it:
//
// fs-end
// he-DX
// fs-he
// start-DX
// pj-DX
// end-zg
// zg-sl
// zg-pj
// pj-he
// RW-he
// fs-DX
// pj-RW
// zg-RW
// start-pj
// he-WI
// zg-he
// pj-fs
// start-RW
//
// How many paths through this cave system are there that visit small caves at most once?
//
// Your puzzle answer was 4775.
// --- Part Two ---
//
// After reviewing the available paths, you realize you might have time to visit a single small cave twice. Specifically, big caves can be visited any number of times, a single small cave can be visited at most twice, and the remaining small caves can be visited at most once. However, the caves named start and end can only be visited exactly once each: once you leave the start cave, you may not return to it, and once you reach the end cave, the path must end immediately.
//
// Now, the 36 possible paths through the first example above are:
//
// start,A,b,A,b,A,c,A,end
// start,A,b,A,b,A,end
// start,A,b,A,b,end
// start,A,b,A,c,A,b,A,end
// start,A,b,A,c,A,b,end
// start,A,b,A,c,A,c,A,end
// start,A,b,A,c,A,end
// start,A,b,A,end
// start,A,b,d,b,A,c,A,end
// start,A,b,d,b,A,end
// start,A,b,d,b,end
// start,A,b,end
// start,A,c,A,b,A,b,A,end
// start,A,c,A,b,A,b,end
// start,A,c,A,b,A,c,A,end
// start,A,c,A,b,A,end
// start,A,c,A,b,d,b,A,end
// start,A,c,A,b,d,b,end
// start,A,c,A,b,end
// start,A,c,A,c,A,b,A,end
// start,A,c,A,c,A,b,end
// start,A,c,A,c,A,end
// start,A,c,A,end
// start,A,end
// start,b,A,b,A,c,A,end
// start,b,A,b,A,end
// start,b,A,b,end
// start,b,A,c,A,b,A,end
// start,b,A,c,A,b,end
// start,b,A,c,A,c,A,end
// start,b,A,c,A,end
// start,b,A,end
// start,b,d,b,A,c,A,end
// start,b,d,b,A,end
// start,b,d,b,end
// start,b,end
//
// The slightly larger example above now has 103 paths through it, and the even larger example now has 3509 paths through it.
//
// Given these new rules, how many paths through this cave system are there?
//
// Your puzzle answer was 152480.

use std::collections::{HashMap, HashSet};
use std::fs;

use anyhow::Result;

use aoc2021::TrimEmpty;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2021/day12")?;
    let paths = parse(&input);
    let count1 = count_paths(&paths, false);
    let count2 = count_paths(&paths, true);
    println!("{:?}", (count1, count2));
    Ok(())
}

fn parse(s: &str) -> HashMap<&str, Vec<&str>> {
    let mut hm = HashMap::new();
    for line in s.lines().trim_empty() {
        let (n1, n2) = line.split_once('-').unwrap();
        hm.entry(n1).or_insert(vec![]).push(n2);
        hm.entry(n2).or_insert(vec![]).push(n1);
    }
    hm
}

fn count_paths(paths: &HashMap<&str, Vec<&str>>, can_revisit: bool) -> u32 {
    struct State<'a, 'b> {
        node: &'a str,
        visited_nodes: &'b mut HashSet<&'a str>,
        used_revisit: bool,
    }

    return count_paths_rec(
        State { node: "start", visited_nodes: &mut HashSet::new(), used_revisit: false },
        paths, can_revisit);

    fn count_paths_rec<'a, 'b>(state: State<'a, 'b>,
                               paths: &HashMap<&str, Vec<&'a str>>,
                               can_revisit: bool) -> u32
    {
        if state.node == "end" {
            return 1;
        }

        let mut count = 0;
        let adj_nodes = paths.get(state.node).unwrap();
        for &node in adj_nodes {
            let visited_node_before = state.visited_nodes.contains(node);
            if node != "start" &&
                ((!can_revisit && !visited_node_before) ||
                    (can_revisit && !state.used_revisit) ||
                    (can_revisit && state.used_revisit && !visited_node_before))
            {
                if is_small_cave(node) {
                    state.visited_nodes.insert(node);
                }
                let (using_revisit, used_revisit) =
                    if can_revisit && !state.used_revisit && visited_node_before {
                        (true, true)
                    } else {
                        (false, state.used_revisit)
                    };

                count += count_paths_rec(
                    State { node, visited_nodes: state.visited_nodes, used_revisit },
                    paths, can_revisit);

                if !using_revisit {
                    state.visited_nodes.remove(node);
                }
            }
        }
        count
    }
}

fn is_small_cave(node: &str) -> bool {
    node.chars().next().unwrap().is_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
        ";
        let paths = parse(&s);
        assert_eq!(226, count_paths(&paths, false));
        assert_eq!(3509, count_paths(&paths, true));
        Ok(())
    }
}
