use std::collections::HashMap;

use rayon::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WordCount {
    pub word: String,
    pub count: usize,
}

pub fn word_freq_seq(text: &str) -> HashMap<&str, usize> {
    let mut freq = HashMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word).or_insert(0) += 1;
    }
    freq
}

pub fn word_freq_par(text: &str) -> HashMap<&str, usize> {
    text.par_split_whitespace()
        .fold(HashMap::new, |mut freq, word| {
            *freq.entry(word).or_insert(0) += 1;
            freq
        })
        .reduce(HashMap::new, merge)
}

fn merge<'a>(
    mut a: HashMap<&'a str, usize>,
    b: HashMap<&'a str, usize>,
) -> HashMap<&'a str, usize> {
    for (word, count) in b {
        *a.entry(word).or_insert(0) += count;
    }
    a
}

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
