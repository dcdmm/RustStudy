import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

// 示例1: 调用Rust command并获取返回值
// 对应Rust: fn greet(name: &str) -> String
export default function Greet() {
  const [name, setName] = useState("");
  const [message, setMessage] = useState("");

  async function greet() {
    // invoke<返回值类型>(command名, 参数对象),返回Promise
    const msg = await invoke<string>("greet", { name });
    setMessage(msg);
  }

  return (
    <section className="card">
      <h2>1. command基础: 参数和返回值</h2>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          value={name}
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="输入名字..."
        />
        <button type="submit">打招呼</button>
      </form>
      <p className="output">{message}</p>
    </section>
  );
}
