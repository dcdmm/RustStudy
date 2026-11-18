import { useState } from "react";
import Card from "@/shared/components/Card.tsx";
import { cancelCountPrimes, countPrimesWithProgress } from "./api.ts";
import styles from "./PrimeProgressCard.module.css";

export default function PrimeProgressCard() {
  const [limit, setLimit] = useState("100000000");
  const [progress, setProgress] = useState(0);
  const [running, setRunning] = useState(false);
  const [message, setMessage] = useState("");

  async function start() {
    setRunning(true);
    setProgress(0);
    setMessage("");
    try {
      const count = await countPrimesWithProgress(Number(limit), setProgress);
      setMessage(count === null ? "已取消" : `共 ${count.toLocaleString()} 个质数`);
    } catch (err) {
      setMessage(`错误: ${err}`);
    } finally {
      setRunning(false);
    }
  }

  async function cancel() {
    try {
      await cancelCountPrimes();
    } catch (err) {
      setMessage(`错误: ${err}`);
    }
  }

  return (
    <Card title="长任务: 进度和取消">
      <div className="row">
        <label className="row">
          统计2到N之间的质数,N =
          <input
            type="number"
            min={1}
            max={1_000_000_000}
            value={limit}
            onChange={(e) => setLimit(e.target.value)}
          />
        </label>
        <button onClick={start} disabled={running}>
          {running ? "计算中..." : "运行"}
        </button>
        <button onClick={cancel} disabled={!running}>
          取消
        </button>
      </div>
      <div className={`row ${styles.progress}`}>
        <progress className={styles.bar} max={100} value={progress} />
        <span>{progress}%</span>
      </div>
      <p className="output">{message}</p>
    </Card>
  );
}
