use std::fmt::Debug;
use std::time::Instant;

use cpu_tasks::{pool, primes, sample, sort, word_freq};

fn main() {
    println!("rayon线程数: {}\n", pool::available_threads());

    let limit = 2_000_000;
    bench("质数 iter()", || primes::count_primes_seq(limit));
    bench("质数 rayon::join", || primes::count_primes_join(limit));
    bench("质数 par_iter()", || primes::count_primes_par(limit));

    let text = sample::text(5_000_000);
    bench("词频 单线程", || word_freq::word_freq_seq(&text).len());
    bench("词频 fold + reduce", || {
        word_freq::word_freq_par(&text).len()
    });

    let numbers = sample::numbers(5_000_000);
    bench("排序 sort_unstable", || {
        sort::sort_seq(&mut numbers.clone())
    });
    bench("排序 par_sort_unstable", || {
        sort::sort_par(&mut numbers.clone())
    });

    println!();
    for threads in pool::thread_steps(pool::available_threads()) {
        bench(&format!("质数 {threads}个线程"), || {
            pool::run_with_threads(threads, || primes::count_primes_par(limit)).unwrap()
        });
    }
}

fn bench<T: Debug>(name: &str, f: impl FnOnce() -> T) {
    let start = Instant::now();
    let result = f();
    let ms = start.elapsed().as_secs_f64() * 1000.0;
    println!("{name:<24}{ms:>10.1} ms   结果: {result:?}");
}
