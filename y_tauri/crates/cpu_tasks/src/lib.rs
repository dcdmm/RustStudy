//! CPU密集型任务: 用rayon把计算分配到所有CPU核心上并行执行
//!
//! 每个模块演示rayon的一个核心用法:
//! 1. primes: 并行迭代器,把iter换成par_iter即可并行
//! 2. word_freq: fold + reduce,并行聚合数据且不需要加锁
//! 3. sort: rayon::join,分治算法的并行
//! 4. pool: 自定义线程池,控制使用的线程数
//! 5. sample: 生成测试数据
//!
//! 本crate只包含同步的纯计算代码,不依赖tokio和Tauri,可以单独测试和使用

pub mod pool;
pub mod primes;
pub mod sample;
pub mod sort;
pub mod word_freq;
