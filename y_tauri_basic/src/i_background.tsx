import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const LIMIT = 20_000_000;

interface PrimeResult {
  count: number;
  ms: number;
}

interface Job {
  running: boolean;
  results: PrimeResult[];
}

function BackgroundJob() {
  const [job, setJob] = useState<Job>({ running: false, results: [] });

  useEffect(() => {
    // 任务状态保存在Rust中,打开页面时获取,切换页面后返回仍可恢复
    invoke<Job>("get_job").then(setJob);
    // Rust中状态变化时发送job-changed事件,payload为最新状态
    const unlisten = listen<Job>("job-changed", (event) => setJob(event.payload));
    return () => {
      unlisten.then((stop) => stop());
    };
  }, []);

  return (
    <section>
      {/* start_job立即返回,不等待计算完成 */}
      <button disabled={job.running} onClick={() => invoke("start_job", { limit: LIMIT })}>
        开始计算
      </button>
      <p>{job.running ? "计算中,可切换到其他页面" : `统计2到${LIMIT}之间的质数个数`}</p>
      {job.results.map((r, i) => (
        <p key={i}>
          {r.count}个质数,耗时{r.ms}ms
        </p>
      ))}
    </section>
  );
}

export default BackgroundJob;
