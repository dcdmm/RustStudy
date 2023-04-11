// Providing New Names with the as Keyword
#[allow(unused)]
use std::fmt::Result;
#[allow(unused)]
use std::io::Result as IoResult;

// Using Nested Paths to Clean Up Large use Lists
// use std::cmp::Ordering;
// use std::io;
// use std::{cmp::Ordering, io}; // 与上等价

// use std::io;
// use std::io::Write;
#[allow(unused)]
use std::io::{self, Write}; // 与上等价

// If we want to bring all public items defined in a path into scope, we can specify that path followed by the * glob operator:
#[allow(unused)]
use std::collections::*;
