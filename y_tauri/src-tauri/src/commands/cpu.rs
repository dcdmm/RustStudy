// 8. CPU密集: 调用crates/cpu_tasks中的rayon并行计算
//
// cpu_tasks中都是同步的计算函数,这里负责:
// 1. 检查参数
// 2. 把计算交给rayon线程池执行,await等待结果,计算期间界面不会卡住
// 3. 记录每种方式的耗时,返回给前端对比

use std::time::Instant;

use cpu_tasks::word_freq::WordCount;
use cpu_tasks::{pool, primes, sample, sort, word_freq};
use serde::Serialize;
use tokio::sync::oneshot;

use crate::error::{AppError, AppResult, ensure_range};

// 参数上限,防止计算时间过长或内存不足
const MAX_PRIME_LIMIT: u64 = 20_000_000;
const MAX_WORD_COUNT: usize = 10_000_000;
const MAX_SORT_LEN: usize = 10_000_000;

/// 一种计算方式的耗时
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

// 执行f并记录耗时
fn timed<T>(label: impl Into<String>, f: impl FnOnce() -> T) -> (T, Timing) {
    let start = Instant::now();
    let value = f();
    let timing = Timing {
        label: label.into(),
        ms: start.elapsed().as_secs_f64() * 1000.0,
    };
    (value, timing)
}

// 把CPU密集的计算交给rayon线程池执行,通过oneshot channel把结果交回async command
//
// 1. 不直接在async command中计算: async command运行在tokio的线程上,长时间计算会占住tokio线程,影响其他异步任务
// 2. 不用spawn_blocking: tokio的阻塞线程池是为阻塞IO准备的,tokio文档建议CPU密集任务交给rayon这样的专用线程池
async fn run_on_rayon<T, F>(f: F) -> AppResult<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let (tx, rx) = oneshot::channel();
    rayon::spawn(move || {
        // 接收端已经不在了(command被取消)时send会失败,忽略即可
        let _ = tx.send(f());
    });
    // tx没有发送结果就被drop时返回错误,正常情况下不会发生
    rx.await.map_err(|_| AppError::TaskAborted)
}

/// rayon全局线程池的线程数
#[tauri::command]
pub fn cpu_threads() -> usize {
    pool::available_threads()
}

/// 统计质数: 单线程iter() vs 并行par_iter()
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

/// 同样的质数统计,分别在1、2、4...个线程的线程池中执行
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
    // 第一个?处理rayon任务的错误,第二个?处理创建线程池的错误(通过#[from]转换为AppError)
    .await??;
    Ok(timings)
}

/// 词频统计: 单线程 vs 并行fold + reduce
#[tauri::command]
pub async fn word_frequency(word_count: usize) -> AppResult<WordFreqReport> {
    ensure_range("wordCount", word_count, 1, MAX_WORD_COUNT)?;
    run_on_rayon(move || {
        // 生成测试文本,不计入耗时
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

// 排序函数的类型: 函数指针,三种排序方式的签名相同,可以放进同一个数组
type SortFn = fn(&mut [u64]);

/// 排序: 单线程 vs rayon::join快速排序 vs rayon内置的par_sort_unstable
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
                // 每种方式都对同一份数据的副本排序,复制数据不计入耗时
                let mut data = numbers.clone();
                let (_, timing) = timed(label, || sort_fn(&mut data));
                debug_assert!(data.is_sorted());
                timing
            })
            .collect()
    })
    .await
}
