import { useState } from "react";
import Card from "@/shared/components/Card.tsx";
import PageHeader from "@/shared/components/PageHeader.tsx";
import { checkUrls, type CheckMode, type Outcome, type UrlCheck } from "./api.ts";

const DEFAULT_URLS = [
  "https://www.baidu.com",
  "https://www.qq.com",
  "https://cn.bing.com",
  "https://www.rust-lang.org",
  "https://crates.io",
  "https://tauri.app",
  "http://10.255.255.1",
  "https://not-exist.invalid",
].join("\n");

const MODE_LABELS: Record<CheckMode, string> = {
  sequential: "顺序执行",
  concurrent: "并发执行",
};

export default function IoPage() {
  const [urlsText, setUrlsText] = useState(DEFAULT_URLS);
  const [maxConcurrency, setMaxConcurrency] = useState("4");
  const [timeoutMs, setTimeoutMs] = useState("3000");
  const [rows, setRows] = useState<UrlCheck[]>([]);
  const [totals, setTotals] = useState<Partial<Record<CheckMode, number>>>({});
  const [running, setRunning] = useState<CheckMode | null>(null);
  const [error, setError] = useState("");

  async function run(mode: CheckMode) {
    const urls = urlsText
      .split("\n")
      .map((line) => line.trim())
      .filter(Boolean);
    setRunning(mode);
    setRows([]);
    setError("");
    try {
      const summary = await checkUrls(
        urls,
        { mode, maxConcurrency: Number(maxConcurrency), timeoutMs: Number(timeoutMs) },
        (check) => setRows((prev) => [...prev, check]),
      );
      setTotals((prev) => ({ ...prev, [mode]: summary.totalMs }));
    } catch (err) {
      setError(String(err));
    } finally {
      setRunning(null);
    }
  }

  return (
    <>
      <PageHeader
        title="9. IO密集: tokio"
        description="网络请求的大部分时间在等待服务器响应;tokio在等待时去执行其他任务,少量线程就能同时处理大量请求"
        sources={["src/features/io/", "src-tauri/src/commands/io.rs", "crates/io_tasks/"]}
      />

      <Card title="并发检测网址">
        <textarea rows={8} value={urlsText} onChange={(e) => setUrlsText(e.target.value)} spellCheck={false} />
        <div className="row">
          <label className="row">
            最大并发数
            <input
              type="number"
              min={1}
              max={32}
              value={maxConcurrency}
              onChange={(e) => setMaxConcurrency(e.target.value)}
            />
          </label>
          <label className="row">
            超时(ms)
            <input
              type="number"
              min={100}
              max={30000}
              value={timeoutMs}
              onChange={(e) => setTimeoutMs(e.target.value)}
            />
          </label>
        </div>
        <div className="row">
          {(["sequential", "concurrent"] as const).map((mode) => (
            <button key={mode} onClick={() => run(mode)} disabled={running !== null}>
              {running === mode ? "执行中..." : MODE_LABELS[mode]}
            </button>
          ))}
          <span className="output">
            {(["sequential", "concurrent"] as const)
              .filter((mode) => totals[mode] !== undefined)
              .map((mode) => `${MODE_LABELS[mode]}总耗时: ${totals[mode]!.toLocaleString()} ms`)
              .join(" | ")}
          </span>
        </div>
        {error && <p className="output error">错误: {error}</p>}
      </Card>

      <Card title="检测结果(按完成的先后顺序)">
        {rows.length === 0 ? (
          <p className="hint">点击上面的按钮开始</p>
        ) : (
          <table className="table">
            <thead>
              <tr>
                <th className="num">#</th>
                <th>网址</th>
                <th>结果</th>
                <th className="num">耗时</th>
              </tr>
            </thead>
            <tbody>
              {rows.map((row, index) => {
                const { text, ok } = describe(row.outcome);
                return (
                  <tr key={row.url}>
                    <td className="num">{index + 1}</td>
                    <td>{row.url}</td>
                    <td className={ok ? undefined : "error"}>{text}</td>
                    <td className="num">{row.elapsedMs.toLocaleString()} ms</td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        )}
      </Card>

      <Card title="要点">
        <ol>
          <li>
            <code>async fn</code> + <code>.await</code>: 每个<code>.await</code>都是一个等待点,等待时线程去执行其他任务
          </li>
          <li>
            <code>JoinSet</code>: 同时执行一组任务,<code>join_next()</code>按完成的先后顺序返回结果
          </li>
          <li>
            <code>Semaphore</code>: 限制同时执行的任务数;把最大并发数改成1,并发执行就和顺序执行一样慢
          </li>
          <li>
            <code>tokio::time::timeout</code>: 超时后future被drop,请求自动取消
          </li>
          <li>
            <code>reqwest::Client</code>内部有连接池,在setup中创建一次,放到State中共用
          </li>
          <li>请求由Rust发出,不受WebView的CORS和CSP限制</li>
        </ol>
        <p className="hint">
          不启动应用也能运行: <code>cargo run -p io_tasks --example check_urls</code>
        </p>
      </Card>
    </>
  );
}

function describe(outcome: Outcome): { text: string; ok: boolean } {
  switch (outcome.kind) {
    case "response":
      return { text: `HTTP ${outcome.status} · ${formatBytes(outcome.bytes)}`, ok: outcome.status < 400 };
    case "timeout":
      return { text: "超时", ok: false };
    case "failed":
      return { text: outcome.message, ok: false };
  }
}

function formatBytes(bytes: number): string {
  return bytes < 1024 ? `${bytes} B` : `${(bytes / 1024).toFixed(1)} KB`;
}
