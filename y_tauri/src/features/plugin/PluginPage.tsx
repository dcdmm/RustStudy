import { useState } from "react";
import { openUrl } from "@tauri-apps/plugin-opener";
import Card from "@/shared/components/Card.tsx";
import PageHeader from "@/shared/components/PageHeader.tsx";

export default function PluginPage() {
  const [message, setMessage] = useState("");

  async function openTauriSite() {
    try {
      await openUrl("https://tauri.app");
      setMessage("已在系统默认浏览器中打开");
    } catch (err) {
      setMessage(`错误: ${err}`);
    }
  }

  return (
    <>
      <PageHeader
        title="7. 插件"
        description="Tauri核心只提供基础能力,文件、对话框、通知、打开链接等功能由官方插件提供"
      />

      <Card title="用系统默认浏览器打开链接">
        <button onClick={openTauriSite}>打开Tauri官网</button>
        <p className="output">{message}</p>
      </Card>
    </>
  );
}
