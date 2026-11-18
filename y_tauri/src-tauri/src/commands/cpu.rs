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
        let (_, par) = timed("并行 par_iter()", || primes::count_primes_par(limit));
        PrimesReport {
            count,
            timings: vec![seq, par],
        }
    })
    .await
}

#[tauri::command]
pub async fn prime_scaling(limit: u64) -> AppResult<Vec<Timing>> {
    ensure_range("limit", limit, 1, MAX_PRIME_LIMIT)?;
    let timings = run_on_rayon(move || {
        pool::thread_steps(pool::available_threads())
            .into_iter()
            .map(|threads| {
                let (result, timing) = timed(format!("{threads}个线程"), || {
                    pool::run_with_threads(threads, || primes::count_primes_par(limit))
                });
                result.map(|_| timing)
            })
            .collect::<Result<Vec<_>, _>>()
    })
    .await??;
    Ok(timings)
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
            top: word_freq::top_n(&freq, 10),
            timings: vec![seq, par],
        }
    })
    .await
}

type SortFn = fn(&mut [u64]);

#[tauri::command]
pub async fn sort_numbers(len: usize) -> AppResult<Vec<Timing>> {
    ensure_range("len", len, 1, MAX_SORT_LEN)?;
    run_on_rayon(move || {
        let numbers = sample::numbers(len);
        let methods: [(&str, SortFn); 3] = [
            ("单线程 sort_unstable()", sort::sort_seq),
            ("rayon::join 快速排序", sort::quick_sort_par),
            ("rayon par_sort_unstable()", sort::sort_par),
        ];
        methods
            .into_iter()
            .map(|(label, sort_fn)| {
                let mut data = numbers.clone();
                let (_, timing) = timed(label, || sort_fn(&mut data));
                debug_assert!(data.is_sorted());
                timing
            })
            .collect()
    })
    .await
}
