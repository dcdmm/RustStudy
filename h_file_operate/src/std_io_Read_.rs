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
    // Pull some bytes from this source into the specified buffer, returning how many bytes were read.
    let n = f.read(&mut buffer)?; // read up to 10 bytes
    println!("The bytes: {:?}", &buffer);
    println!("{}", n);

    Ok(())
}

#[test]
fn read_to_end_test() -> io::Result<()> {
    let mut f = File::open("Read_test.txt")?;

    let mut buffer = Vec::new();
    /*
    Read all bytes until EOF in this source, placing them into buf.

    If successful, this function will return the total number of bytes read.
     */
    let n = f.read_to_end(&mut buffer)?; // read the whole file
    println!("The bytes: {:?}", buffer);
    println!("{}", n);

    Ok(())
}

#[test]
fn read_to_string_test() -> io::Result<()> {
    let mut f = File::open("Read_test.txt")?;

    let mut buffer = String::new();
    /*
    Read all bytes until EOF in this source, appending them to buf.

    If successful, this function returns the number of bytes which were read and appended to buf.
     */
    let n = f.read_to_string(&mut buffer)?;
    println!("The bytes: {:?}", buffer);
    println!("{}", n);

    Ok(())
}
