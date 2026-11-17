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

        // 任务spawn后由runtime管理,JoinHandle仅用于获取结果,drop后任务仍在后台运行
        drop(handle);

        let message = rx.recv().await;
        println!("JoinHandle已drop,任务仍然执行完毕:{message:?}");
    }

    #[tokio::test]
    async fn test02_abort() {
        let handle = tokio::spawn(async {
            sleep(Duration::from_secs(10)).await; // 模拟长时间运行的任务
            "任务完成"
        });

        // Abort the task associated with the handle.
        handle.abort(); // 任务在下一个.await处停止

        // 被取消的任务,await得到Err(JoinError)
        let err = match handle.await {
            Ok(_) => panic!("任务应该被取消,却正常完成了"),
            Err(e) => e,
        };
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

        // abort_all: 一次取消所有任务
        for name in ["D", "E"] {
            set.spawn(async move {
                sleep(Duration::from_secs(10)).await;
                name
            });
        }
        set.abort_all();
        while let Some(result) = set.join_next().await {
            match result {
                Ok(name) => finished.push(name),
                Err(e) => println!("任务被取消:{e:?}"),
            }
        }
        // D和E被取消,没有返回值,finished中仍然只有B、C、A
        println!("取消后:{finished:?}");
    }
}
