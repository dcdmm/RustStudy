// Trait std::io::Read

// # 常见实现
/*
impl Read for &File

impl Read for File
*/

use std::fs::File;
use std::io;
use std::io::prelude::*;

#[test]
fn read_test() -> io::Result<()> {
    let mut f = File::open("Read_test.txt")?;

    let mut buffer = [0; 10];
    // read up to 10 bytes
    let n = f.read(&mut buffer)?;
    println!("The bytes: {:?}", &buffer);
    println!("{}", n);

    Ok(())
}

#[test]
fn read_to_end_test() -> io::Result<()> {
    let mut f = File::open("Read_test.txt")?;

    let mut buffer = Vec::new();
    // read the whole file
    let n = f.read_to_end(&mut buffer)?;
    println!("The bytes: {:?}", buffer);
    println!("{}", n);

    Ok(())
}

#[test]
fn read_to_string_test() -> io::Result<()> {
    let mut f = File::open("Read_test.txt")?;

    let mut buffer = String::new();
    // read the whole file
    let n = f.read_to_string(&mut buffer)?;
    println!("The bytes: {:?}", buffer);
    println!("{}", n);

    Ok(())
}
