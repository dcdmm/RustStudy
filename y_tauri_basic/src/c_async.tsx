import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function SlowTask() {
  const [msg, setMsg] = useState("");

  async function run(command: string) {
    setMsg(`${command} 运行中...`);
    const start = Date.now();
    // 前端调用方式相同,区别只在Rust端是否async
    const result = await invoke<string>(command);
    setMsg(`${result}, 耗时${Date.now() - start}ms`);
  }

  return (
    <section>
      <div className="row" style={{ gap: 5 }}>
        <button onClick={() => run("slow_sync")}>同步等待3秒</button>
        <button onClick={() => run("slow_async")}>异步等待3秒</button>
      </div>
      <p>{msg}</p>
    </section>
  );
}

export default SlowTask;
