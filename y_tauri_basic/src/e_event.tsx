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
    // listen的两个参数:
    // 1. "tick": 事件名,须与Rust中emit的第一个参数一致
    // 2. (event) => ...: 回调函数,Rust每emit一次,Tauri就调用一次
    //    event由Tauri传入,event.payload为本次emit发送的Tick数据
    // 此处仅登记回调,不阻塞等待事件;回调在每次收到tick时执行
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
