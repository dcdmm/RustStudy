import { useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import Card from "@/shared/components/Card.tsx";
import PageHeader from "@/shared/components/PageHeader.tsx";

const appWindow = getCurrentWindow();

export default function WindowPage() {
  const [title, setTitle] = useState("");
  const [titleMessage, setTitleMessage] = useState("");
  const [controlMessage, setControlMessage] = useState("");
  const [deniedMessage, setDeniedMessage] = useState("");

  useEffect(() => {
    let ignore = false;
    appWindow
      .title()
      .then((currentTitle) => {
        if (!ignore) setTitle(currentTitle);
      })
      .catch((err) => {
        if (!ignore) setTitleMessage(`错误: ${err}`);
      });
    return () => {
      ignore = true;
    };
  }, []);

  async function applyTitle() {
    try {
      await appWindow.setTitle(title);
      setTitleMessage("已修改,看看窗口的标题栏");
    } catch (err) {
      setTitleMessage(`错误: ${err}`);
    }
  }

  async function minimize() {
    try {
      await appWindow.minimize();
    } catch (err) {
      setControlMessage(`错误: ${err}`);
    }
  }

  async function toggleMaximize() {
    try {
      await appWindow.toggleMaximize();
    } catch (err) {
      setControlMessage(`错误: ${err}`);
    }
  }

  async function tryDenied() {
    try {
      await appWindow.setResizable(false);
      setDeniedMessage("调用成功(说明已经授权了)");
    } catch (err) {
      setDeniedMessage(`错误: ${err}`);
    }
  }

  return (
    <>
      <PageHeader
        title="6. 窗口与权限"
        description="Tauri 2的安全模型: 前端能调用哪些API,由capabilities中的权限决定,没有授权的调用会被拒绝"
      />

      <Card title="修改窗口标题(已授权)">
        <div className="row">
          <input value={title} onChange={(e) => setTitle(e.target.value)} />
          <button onClick={applyTitle}>修改</button>
        </div>
        <p className="output">{titleMessage}</p>
      </Card>

      <Card title="最小化和最大化(已授权)">
        <div className="row">
          <button onClick={minimize}>最小化</button>
          <button onClick={toggleMaximize}>最大化/还原</button>
        </div>
        <p className="output error">{controlMessage}</p>
      </Card>

      <Card title="禁止调整窗口大小(未授权)">
        <button onClick={tryDenied}>调用setResizable(false)</button>
        <p className="output error">{deniedMessage}</p>
      </Card>
    </>
  );
}
