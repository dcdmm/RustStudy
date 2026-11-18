use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use rayon::prelude::*;

pub fn count_primes_seq(limit: u64) -> u64 {
    (2..=limit).filter(|&n| is_prime(n)).count() as u64
}

pub fn count_primes_join(limit: u64) -> u64 {
    count_range(2, limit + 1)
}

fn count_range(start: u64, end: u64) -> u64 {
    if end - start <= 10_000 {
        return (start..end).filter(|&n| is_prime(n)).count() as u64;
    }
    let mid = start + (end - start) / 2;
    let (left, right) = rayon::join(|| count_range(start, mid), || count_range(mid, end));
    left + right
}

pub fn count_primes_par(limit: u64) -> u64 {
    (2..=limit).into_par_iter().filter(|&n| is_prime(n)).count() as u64
}

pub fn count_primes_cancellable(
    limit: u64,
    cancel: &AtomicBool,
    on_progress: impl Fn(u32) + Sync,
) -> Option<u64> {
    const CHUNKS: u64 = 1000;
    let chunk_size = limit / CHUNKS + 1;
    let finished = AtomicU64::new(0);
    (0..CHUNKS)
        .into_par_iter()
        .map(|i| {
            let start = i * chunk_size;
            let end = (start + chunk_size).min(limit + 1);
            let mut count = 0;
            for n in start..end {
                if cancel.load(Ordering::Relaxed) {
                    return None;
                }
                if is_prime(n) {
                    count += 1;
                }
            }
            let done = finished.fetch_add(1, Ordering::Relaxed) + 1;
            on_progress((done * 100 / CHUNKS) as u32);
            Some(count)
        })
        .try_reduce(|| 0, |a, b| Some(a + b))
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n.is_multiple_of(i) {
            return false;
        }
        i += 1;
    }
    true
}
