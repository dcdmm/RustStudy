use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::Serialize;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlCheck {
    pub url: String,
    pub elapsed_ms: u64,
    pub outcome: Outcome,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Outcome {
    Response { status: u16, bytes: u64 },
    Timeout,
    Failed { message: String },
}

#[derive(Clone)]
pub struct UrlChecker {
    client: reqwest::Client,
}

impl UrlChecker {
    pub fn new() -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::builder()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()?;
        Ok(Self { client })
    }

    pub async fn check(&self, url: &str, timeout: Duration) -> UrlCheck {
        let start = Instant::now();
        let outcome = match tokio::time::timeout(timeout, self.fetch(url)).await {
            Ok(Ok((status, bytes))) => Outcome::Response { status, bytes },
            Ok(Err(err)) => Outcome::Failed {
                message: error_chain(&err),
            },
            Err(_elapsed) => Outcome::Timeout,
        };
        UrlCheck {
            url: url.to_string(),
            elapsed_ms: start.elapsed().as_millis() as u64,
            outcome,
        }
    }

    async fn fetch(&self, url: &str) -> Result<(u16, u64), reqwest::Error> {
        let response = self.client.get(url).send().await?;
        let status = response.status().as_u16();
        let body = response.bytes().await?;
        Ok((status, body.len() as u64))
    }

    pub async fn check_sequential(
        &self,
        urls: &[String],
        timeout: Duration,
        mut on_result: impl FnMut(&UrlCheck),
    ) -> Vec<UrlCheck> {
        let mut results = Vec::with_capacity(urls.len());
        for url in urls {
            let check = self.check(url, timeout).await;
            on_result(&check);
            results.push(check);
        }
        results
    }

    pub async fn check_concurrent(
        &self,
        urls: &[String],
        max_concurrency: usize,
        timeout: Duration,
        mut on_result: impl FnMut(&UrlCheck),
    ) -> Vec<UrlCheck> {
        let semaphore = Arc::new(Semaphore::new(max_concurrency.max(1)));
        let mut tasks = JoinSet::new();

        for url in urls {
            let checker = self.clone();
            let semaphore = Arc::clone(&semaphore);
            let url = url.clone();
            tasks.spawn(async move {
                let _permit = semaphore
                    .acquire_owned()
                    .await
                    .expect("semaphore不会被关闭");
                checker.check(&url, timeout).await
            });
        }

        let mut results = Vec::with_capacity(urls.len());
        while let Some(joined) = tasks.join_next().await {
            if let Ok(check) = joined {
                on_result(&check);
                results.push(check);
            }
        }
        results
    }
}

fn error_chain(err: &dyn Error) -> String {
    let mut message = err.to_string();
    let mut source = err.source();
    while let Some(cause) = source {
        message.push_str(": ");
        message.push_str(&cause.to_string());
        source = cause.source();
    }
    message
}
