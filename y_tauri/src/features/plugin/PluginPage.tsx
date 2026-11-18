import { useState } from "react";
import { openUrl } from "@tauri-apps/plugin-opener";
import Card from "@/shared/components/Card.tsx";
import PageHeader from "@/shared/components/PageHeader.tsx";

export default function PluginPage() {
  const [message, setMessage] = useState("");
  const [scopeMessage, setScopeMessage] = useState("");

  async function openTauriSite() {
    try {
      await openUrl("https://tauri.app");
      setMessage("已在系统默认浏览器中打开");
    } catch (err) {
      setMessage(`错误: ${err}`);
    }
  }

  async function tryOutOfScope() {
    try {
      await openUrl("file:///C:/");
      setScopeMessage("打开成功(说明scope允许了file://)");
    } catch (err) {
      setScopeMessage(`错误: ${err}`);
    }
  }

  return (
    <>
      <PageHeader
        title="7. 插件"
        description="Tauri核心只提供基础能力,文件、对话框、通知、打开链接等功能由官方插件提供"
        sources={["src/features/plugin/", "src-tauri/src/lib.rs", "src-tauri/capabilities/default.json"]}
      />

      <Card title="用系统默认浏览器打开链接">
        <button onClick={openTauriSite}>打开Tauri官网</button>
        <p className="output">{message}</p>
        <p className="hint">
          在WebView中直接用<code>&lt;a href&gt;</code>跳转,会在应用窗口内打开网页;要用系统浏览器打开,需要opener插件
        </p>
      </Card>

      <Card title="权限范围(scope)">
        <button onClick={tryOutOfScope}>打开file:///C:/</button>
        <p className="output error">{scopeMessage}</p>
        <p className="hint">权限不仅控制能不能调用某个API,还可以限制参数的范围,比如只允许打开哪些URL</p>
      </Card>

      <Card title="添加插件: npm run tauri add opener">
        <p>这一条命令自动完成了4件事:</p>
        <ol>
          <li>
            <code>src-tauri/Cargo.toml</code>: 添加Rust依赖<code>tauri-plugin-opener</code>
          </li>
          <li>
            <code>package.json</code>: 添加前端依赖<code>@tauri-apps/plugin-opener</code>
          </li>
          <li>
            <code>src-tauri/src/lib.rs</code>: 注册插件<code>.plugin(tauri_plugin_opener::init())</code>
          </li>
          <li>
            <code>capabilities/default.json</code>: 添加权限<code>opener:default</code>
          </li>
        </ol>
        <p className="hint">
          其他官方插件: dialog(对话框)、fs(文件读写)、notification(系统通知)、store(本地存储)等,添加方式相同
        </p>
      </Card>
    </>
  );
}
