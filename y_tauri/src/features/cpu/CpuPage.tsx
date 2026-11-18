import { useEffect, useState } from "react";
import Card from "@/shared/components/Card.tsx";
import PageHeader from "@/shared/components/PageHeader.tsx";
import { cpuThreads } from "./api.ts";
import PrimesCard from "./PrimesCard.tsx";
import ScalingCard from "./ScalingCard.tsx";
import SortCard from "./SortCard.tsx";
import WordFreqCard from "./WordFreqCard.tsx";

export default function CpuPage() {
  const [threads, setThreads] = useState<number | null>(null);

  useEffect(() => {
    let ignore = false;
    cpuThreads().then((n) => {
      if (!ignore) setThreads(n);
    });
    return () => {
      ignore = true;
    };
  }, []);

  return (
    <>
      <PageHeader
        title="8. CPU密集: rayon"
        description={`计算量大的任务用rayon分配到所有CPU核心上并行执行,本机rayon线程数: ${threads ?? "..."}`}
        sources={["src/features/cpu/", "src-tauri/src/commands/cpu.rs", "crates/cpu_tasks/"]}
      />

      <PrimesCard />
      <ScalingCard />
      <WordFreqCard />
      <SortCard />

      <Card title="要点">
        <ol>
          <li>
            并行迭代器: <code>iter()</code>换成<code>par_iter()</code>,rayon自动切分数据,工作窃取均衡负载
          </li>
          <li>
            <code>fold</code> + <code>reduce</code>: 各块数据分别累计再合并,不共享可变状态,不需要加锁
          </li>
          <li>
            <code>rayon::join</code>: 分治算法的并行,par_iter等都基于它实现;数据量小时改用单线程(粒度控制)
          </li>
          <li>
            线程池: 默认线程数等于CPU逻辑核心数;<code>ThreadPoolBuilder</code>可以创建指定线程数的线程池
          </li>
          <li>
            与Tauri集成: 计算通过<code>rayon::spawn</code>交给rayon线程池,用<code>oneshot</code>
            channel把结果交回async command,不占用tokio的线程,界面不会卡住
          </li>
        </ol>
        <p className="hint">
          不启动应用也能运行: <code>cargo run -p cpu_tasks --example cpu_bench --release</code>
        </p>
      </Card>
    </>
  );
}
