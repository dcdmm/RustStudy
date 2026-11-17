fn main() {}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc;
    use tokio::task::JoinSet;
    use tokio::time::{Duration, sleep};

    #[tokio::test]
    async fn test01_drop_handle_not_cancel() {
        let (tx, mut rx) = mpsc::unbounded_channel();

        let handle = tokio::spawn(async move {
            sleep(Duration::from_millis(20)).await;
            tx.send("任务完成").unwrap();
        });

        // spawn之后任务由runtime管理,JoinHandle只是获取结果的凭证
        // drop(handle)只是放弃获取结果,任务仍在后台继续运行
        // 对比: h_tokio3_more_futures.rs的test04中,select!丢弃的是Future本身,Future被drop就是被取消
        drop(handle);

        let message = rx.recv().await;
        println!("JoinHandle已drop,任务仍然执行完毕:{message:?}");
        assert_eq!(message, Some("任务完成"));
    }

    #[tokio::test]
    async fn test02_abort() {
        let handle = tokio::spawn(async {
            sleep(Duration::from_secs(10)).await; // 模拟长时间运行的任务
            "任务完成"
        });

        // Abort the task associated with the handle.
        // 任务在下一个.await处停止,后面的代码不再执行
        handle.abort();

        // 被取消的任务,await得到Err(JoinError)
        let err = handle.await.unwrap_err();
        println!("任务被取消:{err:?}");
        assert!(err.is_cancelled());
    }

    #[tokio::test]
    async fn test03_joinset() {
        // A collection of tasks spawned on a Tokio runtime.
        let mut set = JoinSet::new();
        for (name, ms) in [("A", 60), ("B", 20), ("C", 40)] {
            set.spawn(async move {
                sleep(Duration::from_millis(ms)).await;
                name
            });
        }

        // join_next按完成的先后顺序返回结果,全部取完后返回None
        let mut finished = Vec::new();
        while let Some(result) = set.join_next().await {
            finished.push(result.unwrap());
        }
        println!("完成顺序:{finished:?}");
        assert_eq!(finished, ["B", "C", "A"]);

        // abort_all: 一次取消所有任务(drop(set)时也会自动取消所有任务)
        for name in ["D", "E"] {
            set.spawn(async move {
                sleep(Duration::from_secs(10)).await;
                name
            });
        }
        set.abort_all();
        while let Some(result) = set.join_next().await {
            assert!(result.unwrap_err().is_cancelled());
        }
    }
}
