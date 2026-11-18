use rayon::prelude::*;

pub fn count_primes_seq(limit: u64) -> u64 {
    (2..=limit).filter(|&n| is_prime(n)).count() as u64
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
