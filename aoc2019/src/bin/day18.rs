// https://adventofcode.com/2019/day/18
//
// --- Day 18: Many-Worlds Interpretation ---
//
// As you approach Neptune, a planetary security system detects you and activates a giant tractor beam on Triton! You have no choice but to land.
//
// A scan of the local area reveals only one interesting feature: a massive underground vault. You generate a map of the tunnels (your puzzle input). The tunnels are too narrow to move diagonally.
//
// Only one entrance (marked @) is present among the open passages (marked .) and stone walls (#), but you also detect an assortment of keys (shown as lowercase letters) and doors (shown as uppercase letters). Keys of a given letter open the door of the same letter: a opens A, b opens B, and so on. You aren't sure which key you need to disable the tractor beam, so you'll need to collect all of them.
//
// For example, suppose you have the following map:
//
// #########
// #b.A.@.a#
// #########
//
// Starting from the entrance (@), you can only access a large door (A) and a key (a). Moving toward the door doesn't help you, but you can move 2 steps to collect the key, unlocking A in the process:
//
// #########
// #b.....@#
// #########
//
// Then, you can move 6 steps to collect the only other key, b:
//
// #########
// #@......#
// #########
//
// So, collecting every key took a total of 8 steps.
//
// Here is a larger example:
//
// ########################
// #f.D.E.e.C.b.A.@.a.B.c.#
// ######################.#
// #d.....................#
// ########################
//
// The only reasonable move is to take key a and unlock door A:
//
// ########################
// #f.D.E.e.C.b.....@.B.c.#
// ######################.#
// #d.....................#
// ########################
//
// Then, do the same with key b:
//
// ########################
// #f.D.E.e.C.@.........c.#
// ######################.#
// #d.....................#
// ########################
//
// ...and the same with key c:
//
// ########################
// #f.D.E.e.............@.#
// ######################.#
// #d.....................#
// ########################
//
// Now, you have a choice between keys d and e. While key e is closer, collecting it now would be slower in the long run than collecting key d first, so that's the best choice:
//
// ########################
// #f...E.e...............#
// ######################.#
// #@.....................#
// ########################
//
// Finally, collect key e to unlock door E, then collect key f, taking a grand total of 86 steps.
//
// Here are a few more examples:
//
//     ########################
//     #...............b.C.D.f#
//     #.######################
//     #.....@.a.B.c.d.A.e.F.g#
//     ########################
//
//     Shortest path is 132 steps: b, a, c, d, f, e, g
//
//     #################
//     #i.G..c...e..H.p#
//     ########.########
//     #j.A..b...f..D.o#
//     ########@########
//     #k.E..a...g..B.n#
//     ########.########
//     #l.F..d...h..C.m#
//     #################
//
//     Shortest paths are 136 steps;
//     one is: a, f, b, j, g, n, h, d, l, o, e, p, c, i, k, m
//
//     ########################
//     #@..............ac.GI.b#
//     ###d#e#f################
//     ###A#B#C################
//     ###g#h#i################
//     ########################
//
//     Shortest paths are 81 steps; one is: a, c, f, i, d, g, b, e, h
//
// How many steps is the shortest path that collects all of the keys?
//
// Your puzzle answer was 3512.
// --- Part Two ---
//
// You arrive at the vault only to discover that there is not one vault, but four - each with its own entrance.
//
// On your map, find the area in the middle that looks like this:
//
// ...
// .@.
// ...
//
// Update your map to instead use the correct data:
//
// @#@
// ###
// @#@
//
// This change will split your map into four separate sections, each with its own entrance:
//
// #######       #######
// #a.#Cd#       #a.#Cd#
// ##...##       ##@#@##
// ##.@.##  -->  #######
// ##...##       ##@#@##
// #cB#Ab#       #cB#Ab#
// #######       #######
//
// Because some of the keys are for doors in other vaults, it would take much too long to collect all of the keys by yourself. Instead, you deploy four remote-controlled robots. Each starts at one of the entrances (@).
//
// Your goal is still to collect all of the keys in the fewest steps, but now, each robot has its own position and can move independently. You can only remotely control a single robot at a time. Collecting a key instantly unlocks any corresponding doors, regardless of the vault in which the key or door is found.
//
// For example, in the map above, the top-left robot first collects key a, unlocking door A in the bottom-right vault:
//
// #######
// #@.#Cd#
// ##.#@##
// #######
// ##@#@##
// #cB#.b#
// #######
//
// Then, the bottom-right robot collects key b, unlocking door B in the bottom-left vault:
//
// #######
// #@.#Cd#
// ##.#@##
// #######
// ##@#.##
// #c.#.@#
// #######
//
// Then, the bottom-left robot collects key c:
//
// #######
// #@.#.d#
// ##.#@##
// #######
// ##.#.##
// #@.#.@#
// #######
//
// Finally, the top-right robot collects key d:
//
// #######
// #@.#.@#
// ##.#.##
// #######
// ##.#.##
// #@.#.@#
// #######
//
// In this example, it only took 8 steps to collect all of the keys.
//
// Sometimes, multiple robots might have keys available, or a robot might have to wait for multiple keys to be collected:
//
// ###############
// #d.ABC.#.....a#
// ######@#@######
// ###############
// ######@#@######
// #b.....#.....c#
// ###############
//
// First, the top-right, bottom-left, and bottom-right robots take turns collecting keys a, b, and c, a total of 6 + 6 + 6 = 18 steps. Then, the top-left robot can access key d, spending another 6 steps; collecting all of the keys here takes a minimum of 24 steps.
//
// Here's a more complex example:
//
// #############
// #DcBa.#.GhKl#
// #.###@#@#I###
// #e#d#####j#k#
// ###C#@#@###J#
// #fEbA.#.FgHi#
// #############
//
//     Top-left robot collects key a.
//     Bottom-left robot collects key b.
//     Top-left robot collects key c.
//     Bottom-left robot collects key d.
//     Top-left robot collects key e.
//     Bottom-left robot collects key f.
//     Bottom-right robot collects key g.
//     Top-right robot collects key h.
//     Bottom-right robot collects key i.
//     Top-right robot collects key j.
//     Bottom-right robot collects key k.
//     Top-right robot collects key l.
//
// In the above example, the fewest steps to collect all of the keys is 32.
//
// Here's an example with more choices:
//
// #############
// #g#f.D#..h#l#
// #F###e#E###.#
// #dCba@#@BcIJ#
// #############
// #nK.L@#@G...#
// #M###N#H###.#
// #o#m..#i#jk.#
// #############
//
// One solution with the fewest steps is:
//
//     Top-left robot collects key e.
//     Top-right robot collects key h.
//     Bottom-right robot collects key i.
//     Top-left robot collects key a.
//     Top-left robot collects key b.
//     Top-right robot collects key c.
//     Top-left robot collects key d.
//     Top-left robot collects key f.
//     Top-left robot collects key g.
//     Bottom-right robot collects key k.
//     Bottom-right robot collects key j.
//     Top-right robot collects key l.
//     Bottom-left robot collects key n.
//     Bottom-left robot collects key m.
//     Bottom-left robot collects key o.
//
// This example requires at least 72 steps to collect all keys.
//
// After updating your map and using the remote-controlled robots, what is the fewest steps necessary to collect all of the keys?
//
// Your puzzle answer was 1514.

use std::{fmt, fs};
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::collections::hash_map::Entry;

use crate::geom::{Direction, Vector2};

type Result<T> = std::result::Result<T, anyhow::Error>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2019/day18")?;
    println!("{}", explore(&input)?);
    let input = fs::read_to_string("input/aoc2019/day18b")?;
    println!("{}", explore(&input)?);
    Ok(())
}

fn explore(s: &str) -> Result<i32> {
    let mut id = 0;
    let grid = s.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.as_bytes().iter()
            .map(|&c| {
                let tile = Tile::new(c, id);
                if let Ok(Tile::Node(Node::Hero(_))) = tile {
                    id += 1;
                }
                tile
            })
            .collect::<Result<Vec<_>>>())
        .collect::<Result<Vec<_>>>()?;

    let mut dists = HashMap::new();
    let mut keys = BTreeSet::new();
    let mut start_nodes = BTreeSet::new();

    for (y, line) in grid.iter().enumerate() {
        for (x, elem) in line.iter().enumerate() {
            match elem {
                Tile::Node(n) => {
                    let pos = Vector2::new(x, y);
                    let adj_nodes = get_adj_dists(&grid, pos);
                    dists.insert(*n, adj_nodes);
                    if let Node::Key(k) = n { keys.insert(*k); }
                    if let Node::Hero(_) = n { start_nodes.insert(*n); }
                }
                _ => ()
            }
        }
    }
    Ok(min_steps(start_nodes, keys, dists))
}


fn get_adj_dists(grid: &Vec<Vec<Tile>>,
                 initial_pos: Vector2) -> HashSet<(Node, i32)> {
    let mut to_process = VecDeque::new();
    let mut visited = HashSet::new();
    let mut res = HashSet::new();
    to_process.push_back((initial_pos, 0));
    visited.insert(initial_pos);

    while !to_process.is_empty() {
        let (pos, depth) = to_process.pop_front().unwrap();
        for dir in Direction::VALUES.iter() {
            let new_pos = pos + dir.dxdy();
            if !visited.contains(&new_pos) {
                let tile = grid[new_pos.y as usize][new_pos.x as usize];
                match tile {
                    Tile::Floor |
                    Tile::Node(Node::Hero(_)) => to_process.push_back((new_pos, depth + 1)),
                    Tile::Node(n) => { res.insert((n, depth + 1)); }
                    _ => ()
                }
                visited.insert(new_pos);
            }
        }
    }
    res
}

fn min_steps(nodes: BTreeSet<Node>,
             keys: BTreeSet<Key>,
             dists: HashMap<Node, HashSet<(Node, i32)>>) -> i32 {
    let mut to_process = VecDeque::new();
    let mut min_dists = HashMap::new();

    let initial_state = (BTreeSet::new(), nodes);
    to_process.push_back(initial_state.clone());
    min_dists.insert(initial_state, 0);
    while !to_process.is_empty() {
        let state = to_process.pop_front().unwrap();
        let acc_dist = *min_dists.get(&state).unwrap();
        let (collected_keys, current_nodes) = state;

        for current_node in current_nodes.iter() {
            for (key, dist) in reachable_keys(*current_node, &collected_keys, &dists) {
                let mut next_collected_keys = collected_keys.clone();
                next_collected_keys.insert(key);

                let mut next_nodes = current_nodes.clone();
                next_nodes.remove(&current_node);
                next_nodes.insert(Node::Key(key));

                let next_state = (next_collected_keys, next_nodes);
                let new_dist = acc_dist + dist;
                match min_dists.entry(next_state) {
                    Entry::Occupied(mut e) =>
                        { e.insert(std::cmp::min(*e.get(), new_dist)); }
                    Entry::Vacant(e) => {
                        to_process.push_back(e.key().clone());
                        e.insert(new_dist);
                    }
                }
            }
        }
    }
    *min_dists.iter()
        .filter_map(|((ks, _), d)| if *ks == keys { Some(d) } else { None })
        .min().unwrap()
}

fn reachable_keys(node: Node,
                  keys: &BTreeSet<Key>,
                  dists: &HashMap<Node, HashSet<(Node, i32)>>) -> HashSet<(Key, i32)> {
    let mut res = HashSet::new();
    let mut visited = HashSet::new();
    visited.insert(node);
    let mut to_process = VecDeque::new();
    to_process.push_back((node, 0));

    while !to_process.is_empty() {
        let (node, acc_dist) = to_process.pop_front().unwrap();
        let adj = dists.get(&node).unwrap();
        for (next_node, dist) in adj.iter() {
            if !visited.contains(next_node) {
                let new_dist = acc_dist + dist;
                match next_node {
                    Node::Key(k) => {
                        if !keys.contains(k) {
                            res.insert((*k, new_dist));
                        } else {
                            to_process.push_back((*next_node, new_dist));
                        }
                    }
                    Node::Door(d) => {
                        if keys.iter().any(|k| k.unlocks() == *d) {
                            to_process.push_back((*next_node, new_dist));
                        }
                    }
                    _ => ()
                }
                visited.insert(*next_node);
            }
        }
    }
    res
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Tile {
    Floor,
    Wall,
    Node(Node),
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum Node {
    Hero(u8),
    Key(Key),
    Door(Door),
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Key(u8);

impl Key {
    fn unlocks(&self) -> Door {
        Door(self.0.to_ascii_uppercase())
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Door(u8);

impl fmt::Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

impl fmt::Debug for Door {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

impl Tile {
    fn new(c: u8, id: u8) -> Result<Tile> {
        match c {
            b'.' => Ok(Tile::Floor),
            b'#' => Ok(Tile::Wall),
            b'@' => Ok(Tile::Node(Node::Hero(id))),
            b'a'..=b'z' => Ok(Tile::Node(Node::Key(Key(c)))),
            b'A'..=b'Z' => Ok(Tile::Node(Node::Door(Door(c)))),
            _ => Err(anyhow::anyhow!("invalid tile: {}", c)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        #########
        #b.A.@.a#
        #########
        ";
        assert_eq!(8, explore(s)?);

        let s = r"
        ########################
        #f.D.E.e.C.b.A.@.a.B.c.#
        ######################.#
        #d.....................#
        ########################
        ";
        assert_eq!(86, explore(s)?);

        let s = r"
        ########################
        #...............b.C.D.f#
        #.######################
        #.....@.a.B.c.d.A.e.F.g#
        ########################
        ";
        assert_eq!(132, explore(s)?);

        let s = r"
        #################
        #i.G..c...e..H.p#
        ########.########
        #j.A..b...f..D.o#
        ########@########
        #k.E..a...g..B.n#
        ########.########
        #l.F..d...h..C.m#
        #################
        ";
        assert_eq!(136, explore(s)?);

        let s = r"
        ########################
        #@..............ac.GI.b#
        ###d#e#f################
        ###A#B#C################
        ###g#h#i################
        ########################
        ";
        assert_eq!(81, explore(s)?);

        Ok(())
    }

    #[test]
    fn test2() -> Result<()> {
        let s = r"
        #######
        #a.#Cd#
        ##@#@##
        #######
        ##@#@##
        #cB#Ab#
        #######
        ";
        assert_eq!(8, explore(s)?);

        let s = r"
        ###############
        #d.ABC.#.....a#
        ######@#@######
        ###############
        ######@#@######
        #b.....#.....c#
        ###############
        ";
        assert_eq!(24, explore(s)?);

        let s = r"
        #############
        #DcBa.#.GhKl#
        #.###@#@#I###
        #e#d#####j#k#
        ###C#@#@###J#
        #fEbA.#.FgHi#
        #############
        ";
        assert_eq!(32, explore(s)?);

        let s = r"
        #############
        #g#f.D#..h#l#
        #F###e#E###.#
        #dCba@#@BcIJ#
        #############
        #nK.L@#@G...#
        #M###N#H###.#
        #o#m..#i#jk.#
        #############
        ";
        assert_eq!(72, explore(s)?);
        Ok(())
    }
}

mod geom {
    use std::ops::{Add, AddAssign, Neg, SubAssign};

    use num::{NumCast, ToPrimitive};

    #[derive(Eq, PartialEq, Hash, Copy, Clone, Default, Debug)]
    pub struct Vector2 {
        pub x: i64,
        pub y: i64,
    }

    impl Vector2 {
        #[allow(dead_code)]
        pub const ZERO: Vector2 = Vector2 { x: 0, y: 0 };

        pub fn new<T>(x: T, y: T) -> Vector2
            where T: ToPrimitive
        {
            let x = NumCast::from::<T>(x).unwrap();
            let y = NumCast::from::<T>(y).unwrap();
            Vector2 { x, y }
        }
    }

    impl Add for Vector2 {
        type Output = Self;

        fn add(self, other: Vector2) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl AddAssign for Vector2 {
        fn add_assign(&mut self, other: Vector2) {
            *self = Self {
                x: self.x + other.x,
                y: self.y + other.y,
            };
        }
    }

    impl SubAssign for Vector2 {
        fn sub_assign(&mut self, other: Vector2) {
            *self = Self {
                x: self.x - other.x,
                y: self.y - other.y,
            };
        }
    }

    impl Neg for Vector2 {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Vector2 {
                x: -self.x,
                y: -self.y,
            }
        }
    }

    pub enum Direction {
        N,
        E,
        S,
        W,
    }

    impl Direction {
        pub const VALUES: [Direction; 4] = [
            Direction::N,
            Direction::E,
            Direction::S,
            Direction::W,
        ];

        pub fn dxdy(&self) -> Vector2 {
            match self {
                Direction::N => Vector2::new(0, -1),
                Direction::E => Vector2::new(1, 0),
                Direction::S => Vector2::new(0, 1),
                Direction::W => Vector2::new(-1, 0),
            }
        }

        #[allow(dead_code)]
        pub fn cw(&self) -> Direction {
            match self {
                Direction::N => Direction::E,
                Direction::E => Direction::S,
                Direction::S => Direction::W,
                Direction::W => Direction::N,
            }
        }

        #[allow(dead_code)]
        pub fn ccw(&self) -> Direction {
            match self {
                Direction::N => Direction::W,
                Direction::E => Direction::N,
                Direction::S => Direction::E,
                Direction::W => Direction::S,
            }
        }
    }
}
