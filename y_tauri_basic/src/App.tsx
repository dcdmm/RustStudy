import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

// 与Rust的Greeting结构对应
interface Greeting {
  message: string;
  isAdult: boolean;
}

function App() {
  const [greeting, setGreeting] = useState<Greeting | null>(null);
  const [name, setName] = useState("");
  const [age, setAge] = useState(0);
  const [error, setError] = useState("");

  async function greet() {
    try {
      // <Greeting>为Rust返回Ok中的类型,如返回String则写<string>
      // 外层person对应Rust参数名,内层name和age对应Person的字段
      setGreeting(await invoke<Greeting>("greet", { person: { name, age } }));
      setError("");
    } catch (e) {
      // invoke失败时进入catch,如Rust返回Err
      setGreeting(null);
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
      {greeting && (
        <p>
          {greeting.message} ({greeting.isAdult ? "成年" : "未成年"})
        </p>
      )}
      <p style={{ color: "red" }}>{error}</p>
    </main>
  );
}

export default App;
