//! 并行迭代器: 统计质数个数
//!
//! 把into_iter换成into_par_iter,rayon会自动把区间切分成很多小块,分给线程池中的线程执行
//! 越大的数判断越慢,每块的工作量并不均匀;rayon的工作窃取(work stealing)会让空闲线程从忙碌线程那里"偷"任务,自动均衡负载

use rayon::prelude::*;

/// 单线程版本: 普通迭代器
pub fn count_primes_seq(limit: u64) -> u64 {
    (2..=limit).filter(|&n| is_prime(n)).count() as u64
}

/// 并行版本: 和单线程版本只差一个into_par_iter
pub fn count_primes_par(limit: u64) -> u64 {
    (2..=limit).into_par_iter().filter(|&n| is_prime(n)).count() as u64
}

// 试除法判断质数,故意不做优化,让每个数都有一定的计算量
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_is_prime() {
        let primes: Vec<u64> = (0..20).filter(|&n| is_prime(n)).collect();
        assert_eq!(primes, [2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn test02_seq_and_par_same_result() {
        assert_eq!(count_primes_seq(100), 25);
        assert_eq!(count_primes_par(100), 25);
        // 10万以内有9592个质数
        assert_eq!(count_primes_seq(100_000), 9592);
        assert_eq!(count_primes_par(100_000), 9592);
    }
}
