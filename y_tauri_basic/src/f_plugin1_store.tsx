import { useEffect, useState } from "react";
// store插件: 键值对存储,数据写入JSON文件,应用重启后仍保留
import { load } from "@tauri-apps/plugin-store";
// dialog的save仅弹出另存为窗口并返回所选路径,不写入文件
import { save } from "@tauri-apps/plugin-dialog";

function Note() {
  // load的参数即文件路径: 相对路径基于应用数据目录,绝对路径按原样使用
  const [path, setPath] = useState("settings.json");
  const [text, setText] = useState("");
  const [msg, setMsg] = useState("");

  useEffect(() => {
    // 路径变化时读取该文件中保存的值,key不存在时返回undefined
    setMsg("");
    load(path)
      .then((store) => store.get<string>("note"))
      .then((value) => setText(value ?? ""))
      .catch((e) => setMsg(String(e)));
  }, [path]);

  async function saveNote() {
    // 文件及所在目录不存在时自动创建;set修改后默认自动写入文件
    const store = await load(path);
    await store.set("note", text);
    setMsg("已保存,重启应用后仍会显示");
  }

  async function pickByDialog() {
    // 用户取消时返回null
    const selected = await save({ defaultPath: "settings.json" });
    if (selected) setPath(selected);
  }

  return (
    <section>
      <button onClick={pickByDialog}>自行选择保存位置</button>
      <p>当前文件: {path}</p>
      <input
        style={{ marginRight: 5 }}
        value={text}
        onChange={(e) => setText(e.currentTarget.value)}
        placeholder="输入内容..."
      />
      <button onClick={saveNote}>保存</button>
      <p>{msg}</p>
    </section>
  );
}

export default Note;
