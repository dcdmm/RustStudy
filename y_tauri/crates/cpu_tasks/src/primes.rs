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
