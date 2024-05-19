use std::fmt::write;
use std::fs::File;
use std::fs::OpenOptions;

#[test]
fn create_test() {
    /*
    Opens a file in write-only mode.

    This function will create a file if it does not exist, and will truncate it if it does
     */
    let file = File::create("foo.txt");

    let _file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("foo.txt"); // 与上等价
}

#[test]
fn open_test() {
    // Attempts to open a file in read-only mode.
    let f = File::open("foo.txt");

    let _f = OpenOptions::new().read(true).open("foo.txt"); // 与上等价
}

#[test]
fn OpenOptions_test() {
    /*
    read(true): 对应python文件打开模式中的"r"
    write(true) + truncate(true): 对应python文件打开模式中的"w"
    append(true): 对应python文件打开模式中的"a"
     */
    let file0 = OpenOptions::new().read(true).open("foo.txt");
    let file1 = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("foo.txt");
    let file2 = OpenOptions::new().append(true).open("foo.txt");
    /*
    pub fn create(&mut self, create: bool) -> &mut Self
        Sets the option to create a new file, or open it if it already exists.

        In order for the file to be created, OpenOptions::write or OpenOptions::append access must be used.
     */
    let file2 = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("foo.txt");
}
