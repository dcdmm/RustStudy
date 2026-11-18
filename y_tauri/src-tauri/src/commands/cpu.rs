use std::time::Instant;

use cpu_tasks::word_freq::WordCount;
use cpu_tasks::{pool, primes, sample, sort, word_freq};
use serde::Serialize;
use tokio::sync::oneshot;

use crate::error::{AppError, AppResult, ensure_range};

const MAX_PRIME_LIMIT: u64 = 20_000_000;
const MAX_WORD_COUNT: usize = 10_000_000;
const MAX_SORT_LEN: usize = 10_000_000;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Timing {
    label: String,
    ms: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimesReport {
    count: u64,
    timings: Vec<Timing>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WordFreqReport {
    unique_words: usize,
    top: Vec<WordCount>,
    timings: Vec<Timing>,
}

fn timed<T>(label: impl Into<String>, f: impl FnOnce() -> T) -> (T, Timing) {
    let start = Instant::now();
    let value = f();
    let timing = Timing {
        label: label.into(),
        ms: start.elapsed().as_secs_f64() * 1000.0,
    };
    (value, timing)
}

async fn run_on_rayon<T, F>(f: F) -> AppResult<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let (tx, rx) = oneshot::channel();
    rayon::spawn(move || {
        let _ = tx.send(f());
    });
    rx.await.map_err(|_| AppError::TaskAborted)
}

#[tauri::command]
pub fn cpu_threads() -> usize {
    pool::available_threads()
}

#[tauri::command]
pub async fn count_primes(limit: u64) -> AppResult<PrimesReport> {
    ensure_range("limit", limit, 1, MAX_PRIME_LIMIT)?;
    run_on_rayon(move || {
        let (count, seq) = timed("单线程 iter()", || primes::count_primes_seq(limit));
        let (_, join) = timed("并行 rayon::join", || primes::count_primes_join(limit));
        let (_, par) = timed("并行 par_iter()", || primes::count_primes_par(limit));
        let mut timings = vec![seq, join, par];
        for threads in pool::thread_steps(pool::available_threads()) {
            let (result, timing) = timed(format!("并行 par_iter() {threads}个线程"), || {
                pool::run_with_threads(threads, || primes::count_primes_par(limit))
            });
            result?;
            timings.push(timing);
        }
        Ok(PrimesReport { count, timings })
    })
    .await?
}

#[tauri::command]
pub async fn word_frequency(word_count: usize) -> AppResult<WordFreqReport> {
    ensure_range("wordCount", word_count, 1, MAX_WORD_COUNT)?;
    run_on_rayon(move || {
        let text = sample::text(word_count);
        let (freq, seq) = timed("单线程", || word_freq::word_freq_seq(&text));
        let (_, par) = timed("并行 fold + reduce", || word_freq::word_freq_par(&text));
        WordFreqReport {
            unique_words: freq.len(),
            top: word_freq::top_n(&freq, 5),
            timings: vec![seq, par],
        }
    })
    .await
}

#[tauri::command]
pub async fn sort_numbers(len: usize) -> AppResult<Vec<Timing>> {
    ensure_range("len", len, 1, MAX_SORT_LEN)?;
    run_on_rayon(move || {
        let mut seq_data = sample::numbers(len);
        let mut par_data = seq_data.clone();
        let (_, seq) = timed("单线程 sort_unstable()", || {
            sort::sort_seq(&mut seq_data)
        });
        let (_, par) = timed("并行 par_sort_unstable()", || {
            sort::sort_par(&mut par_data)
        });
        vec![seq, par]
    })
    .await
}
