import { useState } from "react";
import { countdown, type CountdownEvent } from "./api.ts";
import Card from "@/shared/components/Card.tsx";
import PageHeader from "@/shared/components/PageHeader.tsx";

function describe(event: CountdownEvent): string {
  switch (event.event) {
    case "tick":
      return `剩余 ${event.data.remaining} 秒`;
    case "finished":
      return "倒计时结束!";
  }
}

export default function ChannelPage() {
  const [status, setStatus] = useState("未开始");
  const [running, setRunning] = useState(false);

  async function start() {
    setRunning(true);
    try {
      await countdown(5, (event) => setStatus(describe(event)));
    } catch (err) {
      setStatus(`错误: ${err}`);
    } finally {
      setRunning(false);
    }
  }

  return (
    <>
      <PageHeader
        title="4. Channel"
        description="async command执行耗时任务时,通过Channel把进度多次推送给前端,适合下载进度、日志输出等场景"
        sources={["src/features/channel/", "src-tauri/src/commands/countdown.rs"]}
      />

      <Card title="5秒倒计时">
        <div className="row">
          <button onClick={start} disabled={running}>
            开始
          </button>
          <span className="output">{status}</span>
        </div>
      </Card>

      <Card title="要点">
        <ol>
          <li>
            <code>async fn</code>: 在tokio中执行,不会卡住界面;普通command在主线程执行
          </li>
          <li>
            前端<code>new Channel(回调)</code>作为参数传给Rust,Rust通过<code>on_event.send()</code>多次发送
          </li>
          <li>
            Rust枚举加<code>#[serde(tag = "event", content = "data")]</code>,在TS中对应联合类型
          </li>
        </ol>
      </Card>
    </>
  );
}
