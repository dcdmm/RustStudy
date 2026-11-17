//! 对比顺序请求和并发请求的耗时,不需要启动Tauri应用(需要能访问外网)
//!
//! 运行: cargo run -p io_tasks --example check_urls

use std::time::{Duration, Instant};

use io_tasks::UrlChecker;

// #[tokio::main]: 创建tokio runtime并在其中运行async main
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls: Vec<String> = [
        "https://www.rust-lang.org",
        "https://crates.io",
        "https://docs.rs",
        "https://tokio.rs",
    ]
    .map(String::from)
    .to_vec();
    let checker = UrlChecker::new()?;
    let timeout = Duration::from_secs(5);

    println!("顺序请求:");
    let start = Instant::now();
    checker
        .check_sequential(&urls, timeout, |check| println!("  {check:?}"))
        .await;
    println!("总耗时: {:?}\n", start.elapsed());

    println!("并发请求:");
    let start = Instant::now();
    checker
        .check_concurrent(&urls, 4, timeout, |check| println!("  {check:?}"))
        .await;
    println!("总耗时: {:?}", start.elapsed());

    Ok(())
}
