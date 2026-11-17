import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

// 与Rust中的CountdownPayload结构体对应
type CountdownPayload = {
  remaining: number;
};

// 示例4: Rust通过事件主动向前端推送消息
// 对应Rust: fn start_countdown(app: AppHandle, total_seconds: u32)
export default function Countdown() {
  // null表示还没开始
  const [remaining, setRemaining] = useState<number | null>(null);

  useEffect(() => {
    // 监听名为countdown的事件,Rust每emit一次,回调就执行一次
    // listen返回Promise<unlisten函数>
    const unlisten = listen<CountdownPayload>("countdown", (event) => {
      setRemaining(event.payload.remaining);
    });
    // 组件卸载时取消监听,否则会重复注册(开发模式下StrictMode会挂载两次)
    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  function start() {
    // 参数名: Rust中的total_seconds -> 前端写成totalSeconds
    // command立即返回,后续进度都通过事件推送过来
    invoke("start_countdown", { totalSeconds: 5 });
  }

  const running = remaining !== null && remaining > 0;

  return (
    <section className="card">
      <h2>4. 事件: Rust推送 -&gt; 前端监听</h2>
      <div className="row">
        <button onClick={start} disabled={running}>
          开始5秒倒计时
        </button>
        <span className="output">
          {remaining === null ? "未开始" : running ? `剩余 ${remaining} 秒` : "倒计时结束!"}
        </span>
      </div>
    </section>
  );
}
