import { useState } from "react";
import { Channel, invoke } from "@tauri-apps/api/core";

function TaskProgress() {
  const [progress, setProgress] = useState(0);
  const [msg, setMsg] = useState("");

  async function start() {
    setProgress(0);
    setMsg("运行中...");
    // 创建Channel,接收Rust发送的进度
    const onProgress = new Channel<number>();
    // 收到进度时更新进度条
    onProgress.onmessage = (p) => setProgress(p);
    // 传入Channel并等待最终结果
    // Rust参数on_progress转为camelCase即onProgress,变量同名时可简写
    setMsg(await invoke<string>("run_task", { onProgress }));
  }

  return (
    <section>
      <button onClick={start}>开始任务</button>
      <p>
        <progress value={progress} max={100} /> {progress}% {msg}
      </p>
    </section>
  );
}

export default TaskProgress;
