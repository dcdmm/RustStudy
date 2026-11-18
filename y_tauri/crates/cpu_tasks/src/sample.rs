use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

pub fn numbers(len: usize) -> Vec<u64> {
    let mut rng = StdRng::seed_from_u64(42);
    (0..len).map(|_| rng.random()).collect()
}

const WORDS: [&str; 16] = [
    "rust", "tauri", "rayon", "tokio", "async", "await", "thread", "future", "trait", "crate",
    "cargo", "borrow", "lifetime", "closure", "iterator", "channel",
];

pub fn text(word_count: usize) -> String {
    let mut rng = StdRng::seed_from_u64(7);
    let mut text = String::with_capacity(word_count * 8);
    for i in 0..word_count {
        if i > 0 {
            text.push(' ');
        }
        text.push_str(WORDS[rng.random_range(0..WORDS.len())]);
    }
    text
}
