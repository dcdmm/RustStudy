import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function Counter() {
  const [count, setCount] = useState(0);

  useEffect(() => {
    // 数值保存在Rust端,页面刷新后前端state清空,需重新读取
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
