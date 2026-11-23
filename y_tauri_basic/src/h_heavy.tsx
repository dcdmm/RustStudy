import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface PrimeResult {
  parallel: boolean;
  count: number;
  ms: number;
}

interface Job {
  running: boolean;
  results: PrimeResult[];
}

function PrimeCounter() {
  const [limit, setLimit] = useState(20_000_000);
  const [job, setJob] = useState<Job>({ running: false, results: [] });

  useEffect(() => {
    // 离开页面期间收不到通知,但结果已存入JobState,打开页面时通过get_job取回
    invoke<Job>("get_job").then(setJob);
    // 登记job-changed的回调: Rust每emit一次,Tauri调用一次回调,用payload中的最新状态更新页面
    const unlisten = listen<Job>("job-changed", (event) => setJob(event.payload));
    return () => {
      unlisten.then((stop) => stop());
    }
  }, []);

  return (
    <section>
      <input
        type="number"
        style={{ marginRight: 5 }}
        value={limit}
        onChange={(e) => setLimit(Number(e.currentTarget.value))}
      />
      <button disabled={job.running} onClick={() => invoke("count_primes", { limit, parallel: false })}>
        单线程计算
      </button>
      <button disabled={job.running} onClick={() => invoke("count_primes", { limit, parallel: true })}>
        多线程计算
      </button>
      <p>{job.running ? "计算中,可切换到其他页面" : `统计2到${limit}之间的质数个数`}</p>
      {job.results.map((r, i) => (
        <p key={i}>
          {r.parallel ? "多线程" : "单线程"}: {r.count}个质数,耗时{r.ms}ms
        </p>
      ))}
    </section>
  );
}

export default PrimeCounter;
