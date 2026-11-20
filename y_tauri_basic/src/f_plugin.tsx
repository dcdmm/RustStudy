import { useState } from "react";
// dialog插件: 调用系统原生对话框
// open选择文件或文件夹,save选择保存路径,message显示消息,ask/confirm请求确认
import { open } from "@tauri-apps/plugin-dialog";

function PickFile() {
  const [path, setPath] = useState("");

  async function pick() {
    // 内部通过invoke调用插件的command;返回所选路径,取消时返回null
    const selected = await open();
    setPath(selected ?? "未选择");
  }

  return (
    <section>
      <button onClick={pick}>选择文件</button>
      <p>{path}</p>
    </section>
  );
}

export default PickFile;
