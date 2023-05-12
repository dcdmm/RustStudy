use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

fn main() {

    fn t0_inner() -> Result<(), Box<dyn Error>> {
        let greeting_file = File::open("hello.txt")?;
        Ok(())
    }

    t0_inner();

}