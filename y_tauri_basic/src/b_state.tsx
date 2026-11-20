import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function Counter() {
  const [count, setCount] = useState(0);

  useEffect(() => {
    // 页面加载时从Rust读取当前值,结果交给setCount显示
    invoke<number>("get_count").then(setCount);
  }, []);

  async function increment() {
    setCount(await invoke<number>("increment"));
  }

  return (
    <section>
      <button onClick={increment}>count: {count}</button>
    </section>
  );
}

export default Counter;
