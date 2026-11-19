import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [age, setAge] = useState(0);
  const [error, setError] = useState("");

  async function greet() {
    try {
      // 调用Rust的greet command,参数按名字对应
      setGreetMsg(await invoke<string>("greet", { name, age }));
      setError("");
    } catch (e) {
      // invoke失败时进入catch,如Rust返回Err
      setGreetMsg("");
      setError(String(e));
    }
  }

  return (
    <main className="container">
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <input
          id="age-input"
          type="number"
          onChange={(e) => setAge(Number(e.currentTarget.value))}
          placeholder="Enter an age..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
      <p style={{ color: "red" }}>{error}</p>
    </main>
  );
}

export default App;
