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
      </Card>
    </>
  );
}
