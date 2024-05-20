// Trait std::io::Write

// # 常见实现
/*
impl Write for &File

impl Write for File
*/

use std::fs::File;
use std::io::prelude::*;

#[test]
fn write_all_test() -> std::io::Result<()> {
    let mut buffer = File::create("Write_test.txt")?;

    // Writes some prefix of the byte string, not necessarily all of it.
    buffer.write_all(b"some bytes")?;

    Ok(())
}