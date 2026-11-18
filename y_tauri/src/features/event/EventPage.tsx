import { useState } from "react";
import Card from "@/shared/components/Card.tsx";
import PageHeader from "@/shared/components/PageHeader.tsx";
import { useTauriEvent } from "@/shared/hooks/useTauriEvent.ts";

type LogItem = {
  uptimeSecs: number;
  receivedAt: string;
};

export default function EventPage() {
  const [logs, setLogs] = useState<LogItem[]>([]);

  useTauriEvent("uptime", (payload) => {
    const item = { uptimeSecs: payload.uptimeSecs, receivedAt: new Date().toLocaleTimeString() };
    setLogs((prev) => [item, ...prev].slice(0, 5));
  });

  return (
    <>
      <PageHeader
        title="5. 事件"
        description="Rust在setup中启动后台任务,每秒通过emit广播一次uptime事件,前端用listen监听"
        sources={["src/features/event/", "src/shared/hooks/useTauriEvent.ts", "src-tauri/src/uptime.rs"]}
      />

      <Card title="收到的事件(最近5条)">
        {logs.length === 0 ? (
          <p className="hint">等待事件...</p>
        ) : (
          <ul>
            {logs.map((log) => (
              <li key={log.uptimeSecs}>
                {log.receivedAt} 收到 <code>{JSON.stringify({ uptimeSecs: log.uptimeSecs })}</code>
              </li>
            ))}
          </ul>
        )}
        <p className="hint">底部状态栏也在监听同一个事件: 事件是广播,所有监听者都会收到</p>
      </Card>

      <Card title="Channel和事件的区别">
        <ol>
          <li>Channel: 属于某一次command调用,由前端发起,推送这次调用的进度</li>
          <li>事件: 不依赖command,后端随时都可以向所有监听者广播</li>
        </ol>
      </Card>
    </>
  );
}
