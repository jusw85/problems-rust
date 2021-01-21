// https://adventofcode.com/2020/day/20
//
// --- Day 20: Jurassic Jigsaw ---
//
// The high-speed train leaves the forest and quickly carries you south. You can even see a desert in the distance! Since you have some spare time, you might as well see if there was anything interesting in the image the Mythical Information Bureau satellite captured.
//
// After decoding the satellite messages, you discover that the data actually contains many small images created by the satellite's camera array. The camera array consists of many cameras; rather than produce a single square image, they produce many smaller square image tiles that need to be reassembled back into a single image.
//
// Each camera in the camera array returns a single monochrome image tile with a random unique ID number. The tiles (your puzzle input) arrived in a random order.
//
// Worse yet, the camera array appears to be malfunctioning: each image tile has been rotated and flipped to a random orientation. Your first task is to reassemble the original image by orienting the tiles so they fit together.
//
// To show how the tiles should be reassembled, each tile's image data includes a border that should line up exactly with its adjacent tiles. All tiles have this border, and the border lines up exactly when the tiles are both oriented correctly. Tiles at the edge of the image also have this border, but the outermost edges won't line up with any other tiles.
//
// For example, suppose you have the following nine tiles:
//
// Tile 2311:
// ..##.#..#.
// ##..#.....
// #...##..#.
// ####.#...#
// ##.##.###.
// ##...#.###
// .#.#.#..##
// ..#....#..
// ###...#.#.
// ..###..###
//
// Tile 1951:
// #.##...##.
// #.####...#
// .....#..##
// #...######
// .##.#....#
// .###.#####
// ###.##.##.
// .###....#.
// ..#.#..#.#
// #...##.#..
//
// Tile 1171:
// ####...##.
// #..##.#..#
// ##.#..#.#.
// .###.####.
// ..###.####
// .##....##.
// .#...####.
// #.##.####.
// ####..#...
// .....##...
//
// Tile 1427:
// ###.##.#..
// .#..#.##..
// .#.##.#..#
// #.#.#.##.#
// ....#...##
// ...##..##.
// ...#.#####
// .#.####.#.
// ..#..###.#
// ..##.#..#.
//
// Tile 1489:
// ##.#.#....
// ..##...#..
// .##..##...
// ..#...#...
// #####...#.
// #..#.#.#.#
// ...#.#.#..
// ##.#...##.
// ..##.##.##
// ###.##.#..
//
// Tile 2473:
// #....####.
// #..#.##...
// #.##..#...
// ######.#.#
// .#...#.#.#
// .#########
// .###.#..#.
// ########.#
// ##...##.#.
// ..###.#.#.
//
// Tile 2971:
// ..#.#....#
// #...###...
// #.#.###...
// ##.##..#..
// .#####..##
// .#..####.#
// #..#.#..#.
// ..####.###
// ..#.#.###.
// ...#.#.#.#
//
// Tile 2729:
// ...#.#.#.#
// ####.#....
// ..#.#.....
// ....#..#.#
// .##..##.#.
// .#.####...
// ####.#.#..
// ##.####...
// ##..#.##..
// #.##...##.
//
// Tile 3079:
// #.#.#####.
// .#..######
// ..#.......
// ######....
// ####.#..#.
// .#...#.##.
// #.#####.##
// ..#.###...
// ..#.......
// ..#.###...
//
// By rotating, flipping, and rearranging them, you can find a square arrangement that causes all adjacent borders to line up:
//
// #...##.#.. ..###..### #.#.#####.
// ..#.#..#.# ###...#.#. .#..######
// .###....#. ..#....#.. ..#.......
// ###.##.##. .#.#.#..## ######....
// .###.##### ##...#.### ####.#..#.
// .##.#....# ##.##.###. .#...#.##.
// #...###### ####.#...# #.#####.##
// .....#..## #...##..#. ..#.###...
// #.####...# ##..#..... ..#.......
// #.##...##. ..##.#..#. ..#.###...
//
// #.##...##. ..##.#..#. ..#.###...
// ##..#.##.. ..#..###.# ##.##....#
// ##.####... .#.####.#. ..#.###..#
// ####.#.#.. ...#.##### ###.#..###
// .#.####... ...##..##. .######.##
// .##..##.#. ....#...## #.#.#.#...
// ....#..#.# #.#.#.##.# #.###.###.
// ..#.#..... .#.##.#..# #.###.##..
// ####.#.... .#..#.##.. .######...
// ...#.#.#.# ###.##.#.. .##...####
//
// ...#.#.#.# ###.##.#.. .##...####
// ..#.#.###. ..##.##.## #..#.##..#
// ..####.### ##.#...##. .#.#..#.##
// #..#.#..#. ...#.#.#.. .####.###.
// .#..####.# #..#.#.#.# ####.###..
// .#####..## #####...#. .##....##.
// ##.##..#.. ..#...#... .####...#.
// #.#.###... .##..##... .####.##.#
// #...###... ..##...#.. ...#..####
// ..#.#....# ##.#.#.... ...##.....
//
// For reference, the IDs of the above tiles are:
//
// 1951    2311    3079
// 2729    1427    2473
// 2971    1489    1171
//
// To check that you've assembled the image correctly, multiply the IDs of the four corner tiles together. If you do this with the assembled tiles from the example above, you get 1951 * 3079 * 2971 * 1171 = 20899048083289.
//
// Assemble the tiles into an image. What do you get if you multiply together the IDs of the four corner tiles?
//
// Your puzzle answer was 63187742854073.
// --- Part Two ---
//
// Now, you're ready to check the image for sea monsters.
//
// The borders of each tile are not part of the actual image; start by removing them.
//
// In the example above, the tiles become:
//
// .#.#..#. ##...#.# #..#####
// ###....# .#....#. .#......
// ##.##.## #.#.#..# #####...
// ###.#### #...#.## ###.#..#
// ##.#.... #.##.### #...#.##
// ...##### ###.#... .#####.#
// ....#..# ...##..# .#.###..
// .####... #..#.... .#......
//
// #..#.##. .#..###. #.##....
// #.####.. #.####.# .#.###..
// ###.#.#. ..#.#### ##.#..##
// #.####.. ..##..## ######.#
// ##..##.# ...#...# .#.#.#..
// ...#..#. .#.#.##. .###.###
// .#.#.... #.##.#.. .###.##.
// ###.#... #..#.##. ######..
//
// .#.#.### .##.##.# ..#.##..
// .####.## #.#...## #.#..#.#
// ..#.#..# ..#.#.#. ####.###
// #..####. ..#.#.#. ###.###.
// #####..# ####...# ##....##
// #.##..#. .#...#.. ####...#
// .#.###.. ##..##.. ####.##.
// ...###.. .##...#. ..#..###
//
// Remove the gaps to form the actual image:
//
// .#.#..#.##...#.##..#####
// ###....#.#....#..#......
// ##.##.###.#.#..######...
// ###.#####...#.#####.#..#
// ##.#....#.##.####...#.##
// ...########.#....#####.#
// ....#..#...##..#.#.###..
// .####...#..#.....#......
// #..#.##..#..###.#.##....
// #.####..#.####.#.#.###..
// ###.#.#...#.######.#..##
// #.####....##..########.#
// ##..##.#...#...#.#.#.#..
// ...#..#..#.#.##..###.###
// .#.#....#.##.#...###.##.
// ###.#...#..#.##.######..
// .#.#.###.##.##.#..#.##..
// .####.###.#...###.#..#.#
// ..#.#..#..#.#.#.####.###
// #..####...#.#.#.###.###.
// #####..#####...###....##
// #.##..#..#...#..####...#
// .#.###..##..##..####.##.
// ...###...##...#...#..###
//
// Now, you're ready to search for sea monsters! Because your image is monochrome, a sea monster will look like this:
//
//                   #
// #    ##    ##    ###
//  #  #  #  #  #  #
//
// When looking for this pattern in the image, the spaces can be anything; only the # need to match. Also, you might need to rotate or flip your image before it's oriented correctly to find sea monsters. In the above image, after flipping and rotating it to the appropriate orientation, there are two sea monsters (marked with O):
//
// .####...#####..#...###..
// #####..#..#.#.####..#.#.
// .#.#...#.###...#.##.O#..
// #.O.##.OO#.#.OO.##.OOO##
// ..#O.#O#.O##O..O.#O##.##
// ...#.#..##.##...#..#..##
// #.##.#..#.#..#..##.#.#..
// .###.##.....#...###.#...
// #.####.#.#....##.#..#.#.
// ##...#..#....#..#...####
// ..#.##...###..#.#####..#
// ....#.##.#.#####....#...
// ..##.##.###.....#.##..#.
// #...#...###..####....##.
// .#.##...#.##.#.#.###...#
// #.###.#..####...##..#...
// #.###...#.##...#.##O###.
// .O##.#OO.###OO##..OOO##.
// ..O#.O..O..O.#O##O##.###
// #.#..##.########..#..##.
// #.#####..#.#...##..#....
// #....##..#.#########..##
// #...#.....#..##...###.##
// #..###....##.#...##.##.#
//
// Determine how rough the waters are in the sea monsters' habitat by counting the number of # that are not part of a sea monster. In the above example, the habitat's water roughness is 273.
//
// How many # are not part of a sea monster?
//
// Your puzzle answer was 2152.

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

use anyhow::Result;
use itertools::Itertools;
use num::integer::Roots;
use regex::Regex;

use crate::chargrid::Grid;
use crate::tileborder::SquareTileBorder;
use aoc2020::Enumerate2D;

const DRAGON_RAW: &str = r"
                  # 
#    ##    ##    ###
 #  #  #  #  #  #   
";

lazy_static::lazy_static! {
    static ref DRAGON: Vec<Vec<u8>> = DRAGON_RAW.lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.as_bytes().to_vec())
            .collect_vec();

    static ref DRAGON_PTS: HashSet<(usize, usize)> = DRAGON.iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter()
                .positions(|&c| c == b'#')
                .map(move |x| (y, x)))
            .collect();
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2020/day20")?;
    let tiles = parse(&input);
    let tileborders = solve(&tiles);
    println!("{}", corner_prod(&tileborders));
    let image = assemble_image(tileborders, tiles);
    println!("{:?}", calc_water_roughness(&image));
    Ok(())
}

fn parse(s: &str) -> HashMap<u64, Grid> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
    }
    s.trim().split("\n\n")
        .map(|tile| {
            let (first_line, rest) = tile.split_at(tile.find('\n').unwrap());
            let id = &RE.captures(first_line.trim()).unwrap()[1];
            let id = id.parse::<u64>().unwrap();
            let grid = Grid::parse(rest).unwrap();
            (id, grid)
        })
        .collect::<HashMap<_, _>>()
}

mod tileborder {
    use super::*;

    #[derive(Clone, Eq, PartialEq, Debug, Default)]
    pub struct SquareTileBorder {
        pub id: u64,
        pub n: u16,
        pub e: u16,
        pub s: u16,
        pub w: u16,
        pub num_cw_rotations: usize,
        pub is_flipped: bool,
        len: usize,
    }

    fn reverse(mut border: u16, len: usize) -> u16 {
        let mut res = 0;
        for _ in 0..len {
            res <<= 1;
            res |= border & 1;
            border >>= 1;
        }
        res
    }

    impl SquareTileBorder {
        pub fn new(id: u64, grid: &Grid) -> SquareTileBorder {
            assert_eq!(grid.num_rows, grid.num_cols);
            assert!(grid.num_rows < 16);

            fn to_int(s: &[u8]) -> u16 {
                let s = std::str::from_utf8(s).unwrap();
                let s = s.replace('.', "0").replace('#', "1");
                u16::from_str_radix(&s, 2).unwrap()
            }

            let n = to_int(&grid[0]);
            let s = to_int(&grid[grid.num_rows - 1]);
            let w = to_int(&grid.iter_row().map(|v| v[0]).collect_vec());
            let e = to_int(&grid.iter_row().map(|v| v[v.len() - 1]).collect_vec());

            SquareTileBorder { id, n, e, s, w, len: grid.num_rows, num_cw_rotations: 0, is_flipped: false }
        }

        pub fn flip_x(&self) -> SquareTileBorder {
            let mut res = self.clone();
            res.is_flipped = !res.is_flipped;
            std::mem::swap(&mut res.e, &mut res.w);
            res.n = reverse(res.n, res.len);
            res.s = reverse(res.s, res.len);
            res
        }

        pub fn rotate_cw(&self) -> SquareTileBorder {
            let mut res = self.clone();
            res.num_cw_rotations = (res.num_cw_rotations + 1) % 4;
            std::mem::swap(&mut res.n, &mut res.e);
            std::mem::swap(&mut res.n, &mut res.s);
            std::mem::swap(&mut res.n, &mut res.w);
            res.s = reverse(res.s, res.len);
            res.n = reverse(res.n, res.len);
            res
        }

        pub fn variations(&self) -> impl Iterator<Item=Self> {
            std::iter::successors(Some(self.clone()), |prev|
                if prev.num_cw_rotations < 3 {
                    Some(prev.rotate_cw())
                } else if !prev.is_flipped {
                    Some(prev.rotate_cw().flip_x())
                } else {
                    None
                },
            )
        }
    }
}

fn solve(tiles: &HashMap<u64, Grid>) -> Vec<Vec<SquareTileBorder>> {
    let mut tileborders = tiles.iter()
        .map(|(&id, grid)| SquareTileBorder::new(id, grid))
        .collect::<VecDeque<_>>();

    // testcase and input happen to have unique topleft without rotation / flip
    let (tl_idx, _) = tileborders.iter()
        .enumerate()
        .find(|(_, tb)|
            tileborders.iter()
                .filter(|tb2| tb != tb2)
                .flat_map(SquareTileBorder::variations)
                .all(|variation| variation.s != tb.n && variation.e != tb.w))
        .unwrap();

    let n = tileborders.len().sqrt();
    let mut grid = vec![vec![SquareTileBorder::default(); n]; n];
    grid[0][0] = tileborders.remove(tl_idx).unwrap();

    let found = solve_rec(&mut grid, n, 1, &mut tileborders);
    assert!(found);
    return grid;

    fn solve_rec(mut grid: &mut Vec<Vec<SquareTileBorder>>,
                 n: usize, i: usize,
                 mut tileborders: &mut VecDeque<SquareTileBorder>) -> bool {
        let (y, x) = (i / n, i % n);
        if y >= n {
            return true;
        }
        for _ in 0..tileborders.len() {
            let tileborder = tileborders.pop_front().unwrap();
            for variation in tileborder.variations() {
                if (x <= 0 || grid[y][x - 1].e == variation.w)
                    && (y <= 0 || grid[y - 1][x].s == variation.n)
                {
                    grid[y][x] = variation;
                    let found = solve_rec(&mut grid, n, i + 1, &mut tileborders);
                    if found { return true; }
                }
            }
            tileborders.push_back(tileborder);
        }
        false
    }
}

fn corner_prod(grid: &Vec<Vec<SquareTileBorder>>) -> u64 {
    let n = grid.len();
    grid[0][0].id
        * grid[0][n - 1].id
        * grid[n - 1][0].id
        * grid[n - 1][n - 1].id
}

fn assemble_image(tileborders: Vec<Vec<SquareTileBorder>>,
                  tiles: HashMap<u64, Grid>) -> Grid {
    let tile_size = tiles.values().next().unwrap().num_rows;
    let grid_size = tileborders.len();
    let n = (tile_size - 2) * grid_size;
    let mut res = vec![vec![b'.'; n]; n];

    for (grid_y, grid_x, tb) in tileborders.iter().enumerate_2d() {
        let mut gridp = &tiles[&tb.id];
        let mut grid;
        if tb.is_flipped {
            grid = gridp.flip_x();
            gridp = &grid;
        }
        for _ in 0..tb.num_cw_rotations {
            grid = gridp.rotate_cw();
            gridp = &grid;
        }
        let (offset_y, offset_x) = (grid_y * (tile_size - 2), grid_x * (tile_size - 2));
        for i in 1..(tile_size - 1) {
            for j in 1..(tile_size - 1) {
                res[offset_y + i - 1][offset_x + j - 1] = gridp[i][j];
            }
        }
    }
    Grid::new(res)
}

fn calc_water_roughness(grid: &Grid) -> Option<usize> {
    let n = grid.iter().filter(|(_, _, &c)| c == b'#').count();
    for grid in grid.variations() {
        let pts = find_dragon_pts(&grid);
        if pts.len() > 0 {
            return Some(n - pts.len());
        }
    }
    None
}

fn find_dragon_pts(grid: &Grid) -> HashSet<(usize, usize)> {
    let mut res = HashSet::new();
    for y_tl in 0..(grid.num_rows - DRAGON.len() + 1) {
        for x_tl in 0..(grid.num_cols - DRAGON[0].len() + 1) {
            if DRAGON_PTS.iter().all(|(y, x)|
                grid[y_tl + y][x_tl + x] == b'#')
            {
                res.extend(DRAGON_PTS.iter().map(|(y, x)| (y_tl + y, x_tl + x)));
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        Tile 2311:
        ..##.#..#.
        ##..#.....
        #...##..#.
        ####.#...#
        ##.##.###.
        ##...#.###
        .#.#.#..##
        ..#....#..
        ###...#.#.
        ..###..###

        Tile 1951:
        #.##...##.
        #.####...#
        .....#..##
        #...######
        .##.#....#
        .###.#####
        ###.##.##.
        .###....#.
        ..#.#..#.#
        #...##.#..

        Tile 1171:
        ####...##.
        #..##.#..#
        ##.#..#.#.
        .###.####.
        ..###.####
        .##....##.
        .#...####.
        #.##.####.
        ####..#...
        .....##...

        Tile 1427:
        ###.##.#..
        .#..#.##..
        .#.##.#..#
        #.#.#.##.#
        ....#...##
        ...##..##.
        ...#.#####
        .#.####.#.
        ..#..###.#
        ..##.#..#.

        Tile 1489:
        ##.#.#....
        ..##...#..
        .##..##...
        ..#...#...
        #####...#.
        #..#.#.#.#
        ...#.#.#..
        ##.#...##.
        ..##.##.##
        ###.##.#..

        Tile 2473:
        #....####.
        #..#.##...
        #.##..#...
        ######.#.#
        .#...#.#.#
        .#########
        .###.#..#.
        ########.#
        ##...##.#.
        ..###.#.#.

        Tile 2971:
        ..#.#....#
        #...###...
        #.#.###...
        ##.##..#..
        .#####..##
        .#..####.#
        #..#.#..#.
        ..####.###
        ..#.#.###.
        ...#.#.#.#

        Tile 2729:
        ...#.#.#.#
        ####.#....
        ..#.#.....
        ....#..#.#
        .##..##.#.
        .#.####...
        ####.#.#..
        ##.####...
        ##..#.##..
        #.##...##.

        Tile 3079:
        #.#.#####.
        .#..######
        ..#.......
        ######....
        ####.#..#.
        .#...#.##.
        #.#####.##
        ..#.###...
        ..#.......
        ..#.###...
        ";
        let tiles = parse(s);
        let tileborders = solve(&tiles);
        assert_eq!(20899048083289, corner_prod(&tileborders));
        let image = assemble_image(tileborders, tiles);
        assert_eq!(Some(273), calc_water_roughness(&image));
        Ok(())
    }
}

mod chargrid {
    use std::fmt::{Display, Formatter};
    use std::fmt;
    use std::ops::{Index, IndexMut};

    use anyhow::Result;
    use itertools::Itertools;

    use aoc2020::{TrimEmpty, Enumerate2D};

    use crate::geom::Vector2;

    #[derive(Clone, Hash, Eq, PartialEq, Debug)]
    pub struct Grid {
        grid: Vec<Vec<u8>>,
        pub num_rows: usize,
        pub num_cols: usize,
    }

    impl Grid {
        pub fn new(grid: Vec<Vec<u8>>) -> Grid {
            let num_rows = grid.len();
            let num_cols = grid[0].len();
            Grid { grid, num_rows, num_cols }
        }

        pub fn parse(s: &str) -> Result<Grid> {
            let grid = s.lines()
                .trim_empty()
                .map(|line| line.as_bytes().to_vec())
                .collect_vec();
            if grid.is_empty() {
                anyhow::bail!("empty grid");
            }
            let num_rows = grid.len();
            let num_cols = grid[0].len();

            if grid.iter().any(|row| row.len() != num_cols) {
                anyhow::bail!("jagged grid");
            }
            Ok(Grid { grid, num_rows, num_cols })
        }

        pub fn iter(&self) -> impl Iterator<Item=(usize, usize, &u8)> {
            self.grid.iter().enumerate_2d()
        }

        pub fn iter_row(&self) -> impl Iterator<Item=&Vec<u8>> {
            self.grid.iter()
        }

        pub fn flip_x(&self) -> Grid {
            let (num_rows, num_cols) = (self.num_rows, self.num_cols);
            let mut grid = vec![vec![b'.'; num_cols]; num_rows];
            for (y, x, c) in self.iter() {
                grid[y][num_cols - 1 - x] = *c;
            }
            Grid { grid, num_rows, num_cols }
        }

        pub fn rotate_cw(&self) -> Grid {
            let (num_rows, num_cols) = (self.num_rows, self.num_cols);
            let mut grid = vec![vec![b'.'; num_rows]; num_cols];
            for (y, x, c) in self.iter() {
                grid[x][num_rows - 1 - y] = *c;
            }
            Grid { grid, num_cols: num_rows, num_rows: num_cols }
        }

        pub fn variations(&self) -> impl Iterator<Item=Self> {
            std::iter::successors(Some((0, false, self.clone())), |(rot, flipped, prev)|
                if *rot < 3 {
                    Some((rot + 1, *flipped, prev.rotate_cw()))
                } else if !flipped {
                    Some((0, true, prev.rotate_cw().flip_x()))
                } else {
                    None
                },
            ).map(|(_, _, grid)| grid)
        }
    }

    impl Display for Grid {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            for line in self.grid.iter() {
                let s = std::str::from_utf8(&line).unwrap();
                writeln!(f, "{}", s)?;
            }
            Ok(())
        }
    }

    impl Index<Vector2> for Grid {
        type Output = u8;

        fn index(&self, index: Vector2) -> &Self::Output {
            let y = index.y as usize;
            let x = index.x as usize;
            &self.grid[y][x]
        }
    }

    impl IndexMut<Vector2> for Grid {
        fn index_mut(&mut self, index: Vector2) -> &mut Self::Output {
            let y = index.y as usize;
            let x = index.x as usize;
            &mut self.grid[y][x]
        }
    }

    impl Index<usize> for Grid {
        type Output = Vec<u8>;

        fn index(&self, index: usize) -> &Self::Output {
            &self.grid[index]
        }
    }

    impl IndexMut<usize> for Grid {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.grid[index]
        }
    }
}

mod geom {
    use std::convert::{TryFrom, TryInto};
    use std::ops::{Add, AddAssign, Mul, Neg, SubAssign};

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

        #[allow(dead_code)]
        pub fn manhattan_distance(&self, other: Vector2) -> i64 {
            (self.y - other.y).abs() + (self.x - other.x).abs()
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

    impl Mul<i64> for Vector2 {
        type Output = Self;

        fn mul(self, rhs: i64) -> Self::Output {
            Self {
                x: self.x * rhs,
                y: self.y * rhs,
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

    #[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
    pub enum Direction {
        N,
        NE,
        E,
        SE,
        S,
        SW,
        W,
        NW,
    }

    impl Direction {
        #[allow(dead_code)]
        pub const VALUES_4D: [Direction; 4] = [
            Direction::N,
            Direction::E,
            Direction::S,
            Direction::W,
        ];

        #[allow(dead_code)]
        pub const VALUES_8D: [Direction; 8] = [
            Direction::N,
            Direction::NE,
            Direction::E,
            Direction::SE,
            Direction::S,
            Direction::SW,
            Direction::W,
            Direction::NW,
        ];

        #[allow(dead_code)]
        pub fn dxdy(&self) -> Vector2 {
            match self {
                Direction::N => Vector2::new(0, -1),
                Direction::NE => Vector2::new(1, -1),
                Direction::E => Vector2::new(1, 0),
                Direction::SE => Vector2::new(1, 1),
                Direction::S => Vector2::new(0, 1),
                Direction::SW => Vector2::new(-1, 1),
                Direction::W => Vector2::new(-1, 0),
                Direction::NW => Vector2::new(-1, -1),
            }
        }

        #[allow(dead_code)]
        pub fn cw(&self, num_turns: isize) -> Direction {
            let dir = ((*self as isize) + num_turns) % 8;
            dir.try_into().unwrap()
        }

        #[allow(dead_code)]
        pub fn ccw(&self, num_turns: isize) -> Direction {
            let mut dir = ((*self as isize) - num_turns) % 8;
            if dir < 0 { dir += 8; }
            dir.try_into().unwrap()
        }
    }

    impl TryFrom<isize> for Direction {
        type Error = ();

        fn try_from(i: isize) -> Result<Self, Self::Error> {
            match i {
                x if x == Direction::N as isize => Ok(Direction::N),
                x if x == Direction::NE as isize => Ok(Direction::NE),
                x if x == Direction::E as isize => Ok(Direction::E),
                x if x == Direction::SE as isize => Ok(Direction::SE),
                x if x == Direction::S as isize => Ok(Direction::S),
                x if x == Direction::SW as isize => Ok(Direction::SW),
                x if x == Direction::W as isize => Ok(Direction::W),
                x if x == Direction::NW as isize => Ok(Direction::NW),
                _ => Err(()),
            }
        }
    }
}
