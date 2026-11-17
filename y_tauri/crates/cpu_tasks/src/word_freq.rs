//! fold + reduce: 并行词频统计

use std::collections::HashMap;

use rayon::prelude::*;
use serde::Serialize;

/// 一个词及其出现次数
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WordCount {
    pub word: String,
    pub count: usize,
}

/// 单线程版本
pub fn word_freq_seq(text: &str) -> HashMap<&str, usize> {
    let mut freq = HashMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word).or_insert(0) += 1;
    }
    freq
}

/// 并行版本: fold + reduce
///
/// 1. fold: rayon把文本切分成很多小块,每块用自己的HashMap计数(互不共享,不需要加锁)
/// 2. reduce: 把各块的HashMap两两合并,最终得到一个HashMap
///
/// 如果改成所有线程共用一个Mutex<HashMap>,每个词都要加锁,线程越多争抢越严重,反而比单线程还慢
pub fn word_freq_par(text: &str) -> HashMap<&str, usize> {
    text.par_split_whitespace()
        .fold(HashMap::new, |mut freq, word| {
            *freq.entry(word).or_insert(0) += 1;
            freq
        })
        .reduce(HashMap::new, merge)
}

// 把b中的计数合并到a中
fn merge<'a>(
    mut a: HashMap<&'a str, usize>,
    b: HashMap<&'a str, usize>,
) -> HashMap<&'a str, usize> {
    for (word, count) in b {
        *a.entry(word).or_insert(0) += count;
    }
    a
}

/// 按出现次数从多到少排序,取前n个;次数相同时按字母顺序
pub fn top_n(freq: &HashMap<&str, usize>, n: usize) -> Vec<WordCount> {
    let mut words: Vec<WordCount> = freq
        .iter()
        .map(|(&word, &count)| WordCount {
            word: word.to_string(),
            count,
        })
        .collect();
    words.sort_unstable_by(|a, b| b.count.cmp(&a.count).then_with(|| a.word.cmp(&b.word)));
    words.truncate(n);
    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_word_freq() {
        let freq = word_freq_par("rust tauri rust\nrayon rust tauri");
        println!("{:?}", freq); // print->{"rust": 3, "tauri": 2, "rayon": 1}(顺序不固定)
        assert_eq!(freq["rust"], 3);
        assert_eq!(freq["tauri"], 2);
        assert_eq!(freq["rayon"], 1);
    }

    #[test]
    fn test02_seq_and_par_same_result() {
        let text = crate::sample::text(100_000);
        assert_eq!(word_freq_seq(&text), word_freq_par(&text));
    }

    #[test]
    fn test03_top_n() {
        let freq = word_freq_seq("b a c a b a");
        let top = top_n(&freq, 2);
        assert_eq!(
            top,
            [
                WordCount {
                    word: "a".into(),
                    count: 3
                },
                WordCount {
                    word: "b".into(),
                    count: 2
                },
            ]
        );
    }
}
