import { useEffect, useState } from "react";
// 只通过api.ts与Rust交互
import { addTodo, ApiError, listTodos, removeTodo } from "./api";

function TodoPage() {
  const [items, setItems] = useState<string[]>([]);
  const [text, setText] = useState("");
  const [error, setError] = useState("");

  useEffect(() => {
    listTodos().then(setItems);
  }, []);

  async function add() {
    try {
      setItems(await addTodo(text));
      setText("");
      setError("");
    } catch (e) {
      if (e instanceof ApiError) setError(e.message);
    }
  }

  return (
    <section>
      <input
        style={{ marginRight: 5 }}
        value={text}
        onChange={(e) => setText(e.currentTarget.value)}
        placeholder="输入待办..."
      />
      <button onClick={add}>添加</button>
      <p style={{ color: "red" }}>{error}</p>
      {items.map((item, i) => (
        <p key={i}>
          {item}{" "}
          <button onClick={async () => setItems(await removeTodo(i))}>删除</button>
        </p>
      ))}
    </section>
  );
}

export default TodoPage;
