import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

// 示例3: 数据保存在Rust端的托管状态(State)中
// 对应Rust: fn get_count(counter: State<'_, Counter>) -> u32
//           fn increment(counter: State<'_, Counter>) -> u32
export default function Counter() {
  const [count, setCount] = useState(0);

  // 组件挂载时从Rust读取当前值
  // 按F5刷新页面后数值不会丢失,因为数据保存在Rust进程中,而不是前端
  useEffect(() => {
    invoke<number>("get_count").then(setCount);
  }, []);

  async function increment() {
    // State参数由Tauri自动注入,前端不需要传参数
    setCount(await invoke<number>("increment"));
  }

  return (
    <section className="card">
      <h2>3. 状态管理: State</h2>
      <div className="row">
        <button onClick={increment}>+1</button>
        <span className="output">Rust中的count = {count}</span>
      </div>
    </section>
  );
}
