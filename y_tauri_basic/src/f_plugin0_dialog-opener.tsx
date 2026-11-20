// 插件作用: 前端运行于WebView,无法访问系统功能;插件用Rust实现这些功能,并提供JS API供前端调用
import { useState } from "react";
// dialog插件: 调用系统原生对话框
// open选择文件或文件夹,save选择保存路径,message显示消息,ask/confirm请求确认
import { open } from "@tauri-apps/plugin-dialog";
// opener插件: 用系统默认程序打开链接或文件
// openUrl打开链接,openPath打开文件(需在capabilities中配置允许的路径),revealItemInDir在文件夹中显示
import { openPath, openUrl, revealItemInDir } from "@tauri-apps/plugin-opener";

function PluginDemo() {
  const [path, setPath] = useState<string | null>(null);

  async function pick() {
    // 内部通过invoke调用插件的command;返回所选路径,取消时返回null
    setPath(await open());
  }

  return (
    <section>
      <button onClick={pick}>选择文件</button>
      <button disabled={!path} onClick={() => path && openPath(path)}>
        用默认程序打开
      </button>
      <button disabled={!path} onClick={() => path && revealItemInDir(path)}>
        在文件夹中显示
      </button>
      <button onClick={() => openUrl("https://tauri.app")}>打开Tauri官网</button>
      <p>{path ?? "未选择"}</p>
    </section>
  );
}

export default PluginDemo;
