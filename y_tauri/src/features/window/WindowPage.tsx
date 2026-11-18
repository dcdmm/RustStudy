import { useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import Card from "@/shared/components/Card.tsx";
import PageHeader from "@/shared/components/PageHeader.tsx";

export default function WindowPage() {
  const [title, setTitle] = useState("");
  const [alwaysOnTop, setAlwaysOnTop] = useState(false);
  const [titleMessage, setTitleMessage] = useState("");
  const [deniedMessage, setDeniedMessage] = useState("");

  useEffect(() => {
    let ignore = false;
    const appWindow = getCurrentWindow();
    Promise.all([appWindow.title(), appWindow.isAlwaysOnTop()]).then(([currentTitle, onTop]) => {
      if (!ignore) {
        setTitle(currentTitle);
        setAlwaysOnTop(onTop);
      }
    });
    return () => {
      ignore = true;
    };
  }, []);

  async function applyTitle() {
    try {
      await getCurrentWindow().setTitle(title);
      setTitleMessage("已修改,看看窗口的标题栏");
    } catch (err) {
      setTitleMessage(`错误: ${err}`);
    }
  }

  async function toggleAlwaysOnTop(checked: boolean) {
    await getCurrentWindow().setAlwaysOnTop(checked);
    setAlwaysOnTop(checked);
  }

  async function tryDenied() {
    try {
      await getCurrentWindow().setResizable(false);
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

      <Card title="窗口置顶(已授权)">
        <label className="row">
          <input type="checkbox" checked={alwaysOnTop} onChange={(e) => toggleAlwaysOnTop(e.target.checked)} />
          窗口始终显示在最前面
        </label>
      </Card>

      <Card title="禁止调整窗口大小(未授权)">
        <button onClick={tryDenied}>调用setResizable(false)</button>
        <p className="output error">{deniedMessage}</p>
      </Card>
    </>
  );
}
