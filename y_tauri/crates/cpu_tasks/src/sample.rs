//! 生成测试数据: 使用固定的种子,每次生成的数据都相同,方便对比和测试

/// 伪随机数生成器(SplitMix64算法),只用来生成测试数据
/// 实际项目中生成随机数应该用rand crate
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

/// len个伪随机数
pub fn numbers(len: usize) -> Vec<u64> {
    let mut rng = SplitMix64(42);
    (0..len).map(|_| rng.next()).collect()
}

const WORDS: [&str; 16] = [
    "rust", "tauri", "rayon", "tokio", "async", "await", "thread", "future", "trait", "crate",
    "cargo", "borrow", "lifetime", "closure", "iterator", "channel",
];

/// word_count个用空格分隔的词,排在WORDS前面的词出现得更多
pub fn text(word_count: usize) -> String {
    let mut rng = SplitMix64(7);
    let len = WORDS.len() as u64;
    let mut text = String::with_capacity(word_count * 8);
    for i in 0..word_count {
        if i > 0 {
            text.push(' ');
        }
        // 两个随机下标取较小的,让靠前的词出现的概率更大
        let index = (rng.next() % len).min(rng.next() % len) as usize;
        text.push_str(WORDS[index]);
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_deterministic() {
        assert_eq!(numbers(5), numbers(5));
        assert_eq!(text(5), text(5));
        assert_eq!(text(1000).split_whitespace().count(), 1000);
    }
}
