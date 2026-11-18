import { useEffect, useState } from "react";
import { getName, getVersion } from "@tauri-apps/api/app";
import { useTauriEvent } from "@/shared/hooks/useTauriEvent.ts";
import styles from "./StatusBar.module.css";

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
