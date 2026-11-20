import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

// 与Rust的Greeting结构对应
interface Greeting {
  message: string;
  isAdult: boolean;
}

function Greet() {
  const [greeting, setGreeting] = useState<Greeting | null>(null);
  const [name, setName] = useState("");
  const [age, setAge] = useState(0);
  const [error, setError] = useState("");

  async function greet() {
    try {
      // <Greeting>为Rust返回Ok中的类型,如返回String则写<string>
      // 外层key对应Rust函数的参数名person,snake_case参数名需写成camelCase
      // 内层key对应Person的字段名,变量名与key相同时可简写(即{ name: name, age: age }可以简写为{ name, age })
      setGreeting(await invoke<Greeting>("greet", { person: { name, age } }));
      setError("");
    } catch (e) {
      // invoke失败时进入catch,如Rust返回Err
      setGreeting(null);
      setError(String(e));
    }
  }

  return (
    <section>
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
    </section>
  );
}

export default Greet;
