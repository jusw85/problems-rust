use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

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
