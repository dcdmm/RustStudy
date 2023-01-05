// Note that the standard std library is also a crate that’s external to our package. Because the standard library is shipped with the Rust language, we don’t need to change Cargo.toml to include std. 
use std::io;

use rand::Rng;


// use std::cmp::Ordering;
//use std::io;
use std::{cmp::Ordering, io}; // 与上等价

// use std::io;
// use std::io::Write;
use std::io::{self, Write}; // 与上等价

// If we want to bring all public items defined in a path into scope, we can specify that path followed by the * glob operator:
use std::collections::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
