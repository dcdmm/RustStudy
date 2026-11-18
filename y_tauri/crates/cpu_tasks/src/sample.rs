struct SplitMix64(u64);

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

pub fn numbers(len: usize) -> Vec<u64> {
    let mut rng = SplitMix64(42);
    (0..len).map(|_| rng.next()).collect()
}

const WORDS: [&str; 16] = [
    "rust", "tauri", "rayon", "tokio", "async", "await", "thread", "future", "trait", "crate",
    "cargo", "borrow", "lifetime", "closure", "iterator", "channel",
];

pub fn text(word_count: usize) -> String {
    let mut rng = SplitMix64(7);
    let len = WORDS.len() as u64;
    let mut text = String::with_capacity(word_count * 8);
    for i in 0..word_count {
        if i > 0 {
            text.push(' ');
        }
        let index = (rng.next() % len).min(rng.next() % len) as usize;
        text.push_str(WORDS[index]);
    }
    text
}
