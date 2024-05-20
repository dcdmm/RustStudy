// Trait std::io::Seek

// # 常见实现
/*
impl Seek for &File

impl Seek for File
*/

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;

#[test]
fn t0() -> io::Result<()> {
    let mut f = File::open("Write_test.txt")?;
    let mut buffer0 = Vec::new();
    f.read_to_end(&mut buffer0)?; // 当前文件位置: 文件末尾
    println!("{:?}", buffer0); // print->[115, 111, 109, 101, 32, 98, 121, 116, 101, 115]

    f.seek(SeekFrom::Start(2))?; // 文件当前位置: 从文件开头移动2个字节
    let mut buffer1 = Vec::new();
    f.read_to_end(&mut buffer1)?;
    println!("{:?}", buffer1); // [109, 101, 32, 98, 121, 116, 101, 115]

    f.seek(SeekFrom::Start(2))?;
    f.seek(SeekFrom::Current(3))?; // 文件当前位置: 从当前位置向前移动3个字节
    let mut buffer2 = Vec::new();
    f.read_to_end(&mut buffer2)?;
    println!("{:?}", buffer2); // [98, 121, 116, 101, 115]

    f.seek(SeekFrom::End(-3))?; // 文件当前位置: 从文件末尾移动3个字段
    let mut buffer3 = Vec::new();
    f.read_to_end(&mut buffer3)?;
    println!("{:?}", buffer3); // [116, 101, 115]

    Ok(())
}

#[test]
fn t1() -> io::Result<()> {
    let mut f = File::open("Write_test.txt")?;

    // [115, 111, 109, 101, 32, 98, 121, 116, 101, 115]

    let mut buffer = [0; 3];
    f.read(&mut buffer)?; // 当前文件位置: 文件第3个(从0开始)字符位置处
    println!("{:?}", &buffer); // print->[115, 111, 109]

    let mut buffer1 = [0; 2];
    f.read(&mut buffer1)?; // 当前文件位置: 文件第3 + 2个(从0开始)字符位置处
    println!("{:?}", &buffer1); // print->[101, 32]

    f.seek(SeekFrom::Current(3))?; // 文件当前位置: 从当前位置(3 + 2)向前移动3个字节
    let mut buffer1 = Vec::new();
    f.read_to_end(&mut buffer1)?;
    println!("{:?}", buffer1); // print->[101, 115]

    Ok(())
}
