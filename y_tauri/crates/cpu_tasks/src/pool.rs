//! 自定义线程池: 控制rayon使用多少个线程
//!
//! 不做任何配置时,par_iter等操作使用rayon的全局线程池,线程数默认等于CPU逻辑核心数

use rayon::{ThreadPoolBuildError, ThreadPoolBuilder};

/// rayon全局线程池的线程数
pub fn available_threads() -> usize {
    rayon::current_num_threads()
}

/// 创建一个有threads个线程的线程池,在其中执行op
///
/// op中的rayon操作(par_iter、join等)都只会使用这个线程池中的线程
/// 实际项目中线程池应该创建一次后重复使用;这里为了对比不同线程数的耗时,每次都新建
pub fn run_with_threads<T, F>(threads: usize, op: F) -> Result<T, ThreadPoolBuildError>
where
    F: FnOnce() -> T + Send,
    T: Send,
{
    let pool = ThreadPoolBuilder::new().num_threads(threads).build()?;
    Ok(pool.install(op))
}

/// 要对比的线程数: 从1开始每次翻倍,最后一项是max,如max = 12时返回[1, 2, 4, 8, 12]
pub fn thread_steps(max: usize) -> Vec<usize> {
    let mut steps: Vec<usize> = std::iter::successors(Some(1), |&n| Some(n * 2))
        .take_while(|&n| n < max)
        .collect();
    steps.push(max.max(1));
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_run_with_threads() {
        // 在2个线程的线程池中,current_num_threads返回的是这个线程池的线程数
        let threads = run_with_threads(2, rayon::current_num_threads).unwrap();
        assert_eq!(threads, 2);
    }

    #[test]
    fn test02_thread_steps() {
        assert_eq!(thread_steps(1), [1]);
        assert_eq!(thread_steps(8), [1, 2, 4, 8]);
        assert_eq!(thread_steps(12), [1, 2, 4, 8, 12]);
    }
}
