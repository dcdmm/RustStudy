import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

// 示例2: command返回Result时,前端如何处理错误
// 对应Rust: fn divide(a: f64, b: f64) -> Result<f64, String>
export default function Divide() {
  const [a, setA] = useState("10");
  const [b, setB] = useState("0");
  const [result, setResult] = useState("");

  async function divide() {
    try {
      // Rust返回Ok(v)时,await得到v
      const value = await invoke<number>("divide", { a: Number(a), b: Number(b) });
      setResult(`结果: ${value}`);
    } catch (err) {
      // Rust返回Err(e)时,Promise被reject,这里的err就是e
      setResult(`错误: ${err}`);
    }
  }

  return (
    <section className="card">
      <h2>2. command错误处理: Result</h2>
      <div className="row">
        <input type="number" value={a} onChange={(e) => setA(e.currentTarget.value)} />
        <span>÷</span>
        <input type="number" value={b} onChange={(e) => setB(e.currentTarget.value)} />
        <button onClick={divide}>计算</button>
      </div>
      <p className="output">{result}</p>
    </section>
  );
}
