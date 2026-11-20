use rayon::prelude::*;

// 单线程统计2..=limit中质数的个数
pub fn count_primes_seq(limit: u64) -> u64 {
    (2..=limit).filter(|&n| is_prime(n)).count() as u64
}

// 多线程统计: into_par_iter将区间拆分给多个线程同时计算
pub fn count_primes_par(limit: u64) -> u64 {
    (2..=limit).into_par_iter().filter(|&n| is_prime(n)).count() as u64
}

fn is_prime(n: u64) -> bool {
    let mut i = 2;
    while i * i <= n {
        if n.is_multiple_of(i) {
            return false;
        }
        i += 1;
    }
    true
}

// 不依赖Tauri,可直接用cargo test -p heavy测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_result() {
        assert_eq!(count_primes_seq(100), 25);
        assert_eq!(count_primes_par(100), 25);
    }
}
