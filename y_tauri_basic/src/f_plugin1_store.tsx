import { useEffect, useState } from "react";
// store插件: 键值对存储,数据写入JSON文件
import { load } from "@tauri-apps/plugin-store";
import { save } from "@tauri-apps/plugin-dialog";

function Note() {
  const [path, setPath] = useState("settings.json");
  const [text, setText] = useState("");
  const [msg, setMsg] = useState("");

  // 作用: 从文件读取上次保存的内容并显示,使应用重启后仍能看到
  useEffect(() => {
    setMsg("");
    // 1. 打开path对应的文件,得到store
    load(path)
      // 2. 取出key为note的值,不存在时为undefined
      .then((store) => store.get<string>("note"))
      // 3. 将值放入输入框,值为undefined时放入空字符串
      .then((value) => setText(value ?? ""))
      .catch((e) => setMsg(String(e)));
  }, [path]);

  async function saveNote() {
    const store = await load(path);
    // set后自动写入文件,文件不存在时自动创建;每次保存覆盖上一次的note
    await store.set("note", text);
    setMsg("已保存,重启应用后仍会显示");
  }

  async function pickByDialog() {
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
