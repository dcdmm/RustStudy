import { useEffect, useState } from "react";
import { getName, getVersion } from "@tauri-apps/api/app";
import { useTauriEvent } from "@/shared/hooks/useTauriEvent.ts";
import styles from "./StatusBar.module.css";

// 底部状态栏
// 和EventPage同时监听uptime事件: 事件是广播,所有监听者都会收到
export default function StatusBar() {
  const [appLabel, setAppLabel] = useState("");
  const [uptimeSecs, setUptimeSecs] = useState(0);

  useEffect(() => {
    let ignore = false;
    Promise.all([getName(), getVersion()]).then(([name, version]) => {
      if (!ignore) setAppLabel(`${name} v${version}`);
    });
    return () => {
      ignore = true;
    };
  }, []);

  useTauriEvent("uptime", (payload) => setUptimeSecs(payload.uptimeSecs));

  return (
    <footer className={styles.statusBar}>
      <span>{appLabel}</span>
      <span>已运行 {uptimeSecs} 秒</span>
    </footer>
  );
}
