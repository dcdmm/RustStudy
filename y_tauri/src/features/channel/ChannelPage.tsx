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
      />

      <Card title="5秒倒计时">
        <div className="row">
          <button onClick={start} disabled={running}>
            开始
          </button>
          <span className="output">{status}</span>
        </div>
      </Card>
    </>
  );
}
