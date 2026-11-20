import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

// 与Rust的Tick结构对应
interface Tick {
  seconds: number;
  message: string;
}

function Ticker() {
  const [tick, setTick] = useState<Tick | null>(null);

  useEffect(() => {
    // 事件名须与Rust一致,event.payload为emit发送的数据
    const unlisten = listen<Tick>("tick", (event) => setTick(event.payload));
    // listen返回Promise,结果为取消监听的函数,组件卸载时调用
    return () => {
      unlisten.then((stop) => stop());
    };
  }, []);

  return (
    <section>
      {tick && (
        <p>
          Rust已运行{tick.seconds}秒,{tick.message}
        </p>
      )}
    </section>
  );
}

export default Ticker;
