// https://adventofcode.com/2021/day/20
//
// --- Day 20: Trench Map ---
//
// With the scanners fully deployed, you turn their attention to mapping the floor of the ocean trench.
//
// When you get back the image from the scanners, it seems to just be random noise. Perhaps you can combine an image enhancement algorithm and the input image (your puzzle input) to clean it up a little.
//
// For example:
//
// ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
// #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
// .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
// .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
// .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
// ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
// ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
//
// #..#.
// #....
// ##..#
// ..#..
// ..###
//
// The first section is the image enhancement algorithm. It is normally given on a single line, but it has been wrapped to multiple lines in this example for legibility. The second section is the input image, a two-dimensional grid of light pixels (#) and dark pixels (.).
//
// The image enhancement algorithm describes how to enhance an image by simultaneously converting all pixels in the input image into an output image. Each pixel of the output image is determined by looking at a 3x3 square of pixels centered on the corresponding input image pixel. So, to determine the value of the pixel at (5,10) in the output image, nine pixels from the input image need to be considered: (4,9), (4,10), (4,11), (5,9), (5,10), (5,11), (6,9), (6,10), and (6,11). These nine input pixels are combined into a single binary number that is used as an index in the image enhancement algorithm string.
//
// For example, to determine the output pixel that corresponds to the very middle pixel of the input image, the nine pixels marked by [...] would need to be considered:
//
// # . . # .
// #[. . .].
// #[# . .]#
// .[. # .].
// . . # # #
//
// Starting from the top-left and reading across each row, these pixels are ..., then #.., then .#.; combining these forms ...#...#.. By turning dark pixels (.) into 0 and light pixels (#) into 1, the binary number 000100010 can be formed, which is 34 in decimal.
//
// The image enhancement algorithm string is exactly 512 characters long, enough to match every possible 9-bit binary number. The first few characters of the string (numbered starting from zero) are as follows:
//
// 0         10        20        30  34    40        50        60        70
// |         |         |         |   |     |         |         |         |
// ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
//
// In the middle of this first group of characters, the character at index 34 can be found: #. So, the output pixel in the center of the output image should be #, a light pixel.
//
// This process can then be repeated to calculate every pixel of the output image.
//
// Through advances in imaging technology, the images being operated on here are infinite in size. Every pixel of the infinite output image needs to be calculated exactly based on the relevant pixels of the input image. The small input image you have is only a small region of the actual infinite input image; the rest of the input image consists of dark pixels (.). For the purposes of the example, to save on space, only a portion of the infinite-sized input and output images will be shown.
//
// The starting input image, therefore, looks something like this, with more dark pixels (.) extending forever in every direction not shown here:
//
// ...............
// ...............
// ...............
// ...............
// ...............
// .....#..#......
// .....#.........
// .....##..#.....
// .......#.......
// .......###.....
// ...............
// ...............
// ...............
// ...............
// ...............
//
// By applying the image enhancement algorithm to every pixel simultaneously, the following output image can be obtained:
//
// ...............
// ...............
// ...............
// ...............
// .....##.##.....
// ....#..#.#.....
// ....##.#..#....
// ....####..#....
// .....#..##.....
// ......##..#....
// .......#.#.....
// ...............
// ...............
// ...............
// ...............
//
// Through further advances in imaging technology, the above output image can also be used as an input image! This allows it to be enhanced a second time:
//
// ...............
// ...............
// ...............
// ..........#....
// ....#..#.#.....
// ...#.#...###...
// ...#...##.#....
// ...#.....#.#...
// ....#.#####....
// .....#.#####...
// ......##.##....
// .......###.....
// ...............
// ...............
// ...............
//
// Truly incredible - now the small details are really starting to come through. After enhancing the original input image twice, 35 pixels are lit.
//
// Start with the original input image and apply the image enhancement algorithm twice, being careful to account for the infinite size of the images. How many pixels are lit in the resulting image?
//
// Your puzzle answer was 5081.
// --- Part Two ---
//
// You still can't quite make out the details in the image. Maybe you just didn't enhance it enough.
//
// If you enhance the starting input image in the above example a total of 50 times, 3351 pixels are lit in the final output image.
//
// Start again with the original input image and apply the image enhancement algorithm 50 times. How many pixels are lit in the resulting image?
//
// Your puzzle answer was 15088.

use std::fs;

use anyhow::Result;
use itertools::Itertools;

use aoc2021::{Enumerate2D, TrimEmpty};

fn main() -> Result<()> {
    let input = fs::read_to_string("input/aoc2021/day20")?;
    let (lookup, points) = parse(&input);
    println!("{:?}", evolve(&lookup, &points, 2));
    println!("{:?}", evolve(&lookup, &points, 50));
    Ok(())
}

fn parse(s: &str) -> (Vec<char>, Vec<Vec<char>>) {
    let (lookup, input) = s.trim().split_once("\n\n").unwrap();
    let lookup = lookup.chars().collect_vec();
    assert_eq!(512, lookup.len());

    let points = input.lines().trim_empty()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    (lookup, points)
}

fn evolve(lookup: &Vec<char>, points: &Vec<Vec<char>>, num_steps: usize) -> usize {
    let mut out = vec![vec!['.'; points[0].len() + (2 * num_steps) + 4];
                       points.len() + (2 * num_steps) + 4];
    let mut other = out.clone();

    let mut start_x = num_steps + 2;
    let mut end_x = start_x + points[0].len();
    let mut start_y = num_steps + 2;
    let mut end_y = start_y + points.len();

    copy_vec(points, &mut out, start_y, start_x);

    for _ in 0..num_steps {
        start_x -= 1;
        start_y -= 1;
        end_x += 1;
        end_y += 1;

        let fill = match out[start_y - 1][start_x - 1] {
            '#' => lookup[0b111111111],
            '.' => lookup[0],
            _ => unreachable!(),
        };
        for y in (start_y - 2)..(end_y + 2) {
            for x in (start_x - 2)..(end_x + 2) {
                other[y][x] = if (start_y..end_y).contains(&y) && (start_x..end_x).contains(&x) {
                    let idx = (-1..=1).cartesian_product(-1..=1)
                        .fold(0, |acc, (dy, dx)| (acc << 1) +
                            ((out[(y as i32 + dy) as usize][(x as i32 + dx) as usize] == '#') as usize));
                    lookup[idx]
                } else {
                    fill
                }
            }
        }

        let tmp = other;
        other = out;
        out = tmp;
    }
    out.iter().enumerate_2d().filter(|&(_, _, &c)| c == '#').count()
}

fn copy_vec<T: Copy>(src: &Vec<Vec<T>>, dest: &mut Vec<Vec<T>>, dest_y: usize, dest_x: usize) {
    for (dy, dx) in (0..src.len()).cartesian_product(0..src[0].len()) {
        dest[dest_y + dy][dest_x + dx] = src[dy][dx];
    }
}

#[allow(dead_code)]
fn pprint(grid: &Vec<Vec<char>>) {
    for line in grid.iter() {
        let s = line.into_iter().collect::<String>();
        println!("{}", s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let s = r"
        ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

        #..#.
        #....
        ##..#
        ..#..
        ..###
        ";
        let (lookup, points) = parse(&s);
        assert_eq!(35, evolve(&lookup, &points, 2));
        assert_eq!(3351, evolve(&lookup, &points, 50));
        Ok(())
    }
}
