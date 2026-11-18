import { useEffect, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import Card from "@/shared/components/Card.tsx";
import PageHeader from "@/shared/components/PageHeader.tsx";
import { openTextFile, recentFiles, type OpenedFile, type RecentFiles } from "./api.ts";
import styles from "./FilesPage.module.css";

const TEXT_EXTENSIONS = ["txt", "md", "json", "toml", "log", "rs", "ts", "tsx"];

export default function FilesPage() {
  const [file, setFile] = useState<OpenedFile | null>(null);
  const [recent, setRecent] = useState<RecentFiles | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");

  useEffect(() => {
    let ignore = false;
    recentFiles()
      .then((result) => {
        if (!ignore) setRecent(result);
      })
      .catch((err) => {
        if (!ignore) setError(String(err));
      });
    return () => {
      ignore = true;
    };
  }, []);

  async function openPath(path: string) {
    setBusy(true);
    setError("");
    try {
      setFile(await openTextFile(path));
      setRecent(await recentFiles());
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }

  async function pickFile() {
    try {
      const path = await open({ filters: [{ name: "文本文件", extensions: TEXT_EXTENSIONS }] });
      if (path) await openPath(path);
    } catch (err) {
      setError(String(err));
    }
  }

  return (
    <>
      <PageHeader
        title="8. 文件与数据"
        description="用dialog插件选择文件,Rust用tokio::fs读取文件,并把最近打开的文件记录保存到应用数据目录"
      />

      <Card title="打开文本文件">
        <button onClick={pickFile} disabled={busy}>
          选择文件...
        </button>
        {error && <p className="output error">错误: {error}</p>}
        {file && (
          <div className={styles.file}>
            <dl className="info-list">
              <dt>路径</dt>
              <dd>{file.path}</dd>
              <dt>大小</dt>
              <dd>{file.size.toLocaleString()} 字节</dd>
              <dt>行数</dt>
              <dd>{file.lines.toLocaleString()}</dd>
            </dl>
            <pre className={styles.preview}>{file.preview}</pre>
          </div>
        )}
      </Card>

      <Card title="最近打开">
        {recent && recent.paths.length > 0 ? (
          <ul className={styles.recent}>
            {recent.paths.map((path) => (
              <li key={path}>
                <button onClick={() => openPath(path)} disabled={busy}>
                  {path}
                </button>
              </li>
            ))}
          </ul>
        ) : (
          <p className="hint">还没有打开过文件</p>
        )}
        {recent && (
          <p className="hint">
            保存位置: <code>{recent.storePath}</code>
          </p>
        )}
      </Card>
    </>
  );
}
