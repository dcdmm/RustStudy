import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface PrimeResult {
  count: number;
  ms: number;
}

function PrimeCounter() {
  const [limit, setLimit] = useState(5_000_000);
  const [running, setRunning] = useState(false);
  const [lines, setLines] = useState<string[]>([]);

  async function run(parallel: boolean) {
    setRunning(true);
    const mode = parallel ? "多线程" : "单线程";
    try {
      const r = await invoke<PrimeResult>("count_primes", { limit, parallel });
      setLines((prev) => [...prev, `${mode}: ${r.count}个质数,耗时${r.ms}ms`]);
    } catch (e) {
      setLines((prev) => [...prev, `${mode}: ${String(e)}`]);
    } finally {
      setRunning(false);
    }
  }

  return (
    <section>
      <input
        type="number"
        style={{ marginRight: 5 }}
        value={limit}
        onChange={(e) => setLimit(Number(e.currentTarget.value))}
      />
      <button disabled={running} onClick={() => run(false)}>
        单线程计算
      </button>
      <button disabled={running} onClick={() => run(true)}>
        多线程计算
      </button>
      <p>{running ? "计算中..." : `统计2到${limit}之间的质数个数`}</p>
      {lines.map((line, i) => (
        <p key={i}>{line}</p>
      ))}
    </section>
  );
}

export default PrimeCounter;
