import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

// 与Rust的ErrorBody结构对应
interface AppError {
  kind: "emptyPath" | "io";
  message: string;
}

function ReadText() {
  const [path, setPath] = useState("");
  const [content, setContent] = useState("");
  const [error, setError] = useState<AppError | null>(null);

  async function read() {
    try {
      setContent(await invoke<string>("read_text", { path }));
      setError(null);
    } catch (e) {
      // Rust返回Err时invoke抛出ErrorBody转成的普通对象
      setContent("");
      setError(e as AppError);
    }
  }

  return (
    <section>
      <input
        style={{ marginRight: 5 }}
        value={path}
        onChange={(e) => setPath(e.currentTarget.value)}
        placeholder="输入文件路径..."
      />
      <button onClick={read}>读取</button>
      {error && (
        <p style={{ color: "red" }}>
          错误类型: {error.kind}, 错误信息: {error.message}
        </p>
      )}
      <pre style={{ textAlign: "left", whiteSpace: "pre-wrap" }}>{content}</pre>
    </section>
  );
}

export default ReadText;
