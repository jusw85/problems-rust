#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

fn quick_benchmark() {
    let now = std::time::Instant::now();
    // code
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn dynamic_iterator1() {
    let nums = [1, 2, 3];
    let mut iter = nums.iter();
    let mut iterp: &mut dyn Iterator<Item=&i32> = &mut iter;

    let mut iter1;
    if nums.len() > 1 {
        iter1 = iter.take(1);
        iterp = &mut iter1;
    }

    for i in iterp {
        println!("{}", i);
    }
}

fn dynamic_iterator2() {
    let nums = [1, 2, 3];

    let iter = if nums.len() > 1 {
        itertools::Either::Left(nums.iter().take(1))
    } else {
        itertools::Either::Right(nums.iter())
    };

    for i in iter {
        println!("{}", i);
    }
}

fn main() -> std::io::Result<()> {
    read_file_twice()?;
    Ok(())
}

// Stream read file twice example
//
// File has an internal cursor, second read will return empty if cursor does not seek to start of file
// Cursor belongs to file, recreating a new BufReader will not recreate the cursor / reset file position
pub fn read_file_twice() -> std::io::Result<()> {
    let file = File::open("input/aoc2019/day1")?;
    let mut reader = BufReader::new(file);

    let count1 = reader.by_ref().lines().count();
    println!("{}", count1);

    reader.seek(SeekFrom::Start(0))?;

    let count2 = reader.by_ref().lines().count();
    println!("{}", count2);

    Ok(())
}
