import { useEffect, useState } from "react";
// appDataDir: 获取应用数据目录(Windows下为C:\Users\<用户名>\AppData\Roaming\<identifier>)
// join: 将多段路径拼接为完整路径,自动使用当前系统的分隔符
import { appDataDir, join } from "@tauri-apps/api/path"; 
// fs插件: 读写本地文件
// BaseDirectory: 系统目录的枚举,如AppData表示应用数据目录,Desktop表示桌面
// exists: 判断文件或目录是否存在 
// mkdir: 创建目录;recursive: true时可创建多级目录,目录已存在也不报错
// readTextFile: 读取文本文件的全部内容
// writeTextFile: 将文本写入文件,文件不存在时创建,已存在时覆盖
import { BaseDirectory, exists, mkdir, readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";

const FILE = "note.txt";
const opts = { baseDir: BaseDirectory.AppData };

function FileDemo() {
  const [path, setPath] = useState("");
  const [text, setText] = useState("");
  const [content, setContent] = useState("");

  // 作用: 显示文件的完整路径,仅供查看,读写文件不依赖它
  useEffect(() => {
    // 1. 获取应用数据目录
    appDataDir()
      // 2. 拼接文件名
      .then((dir) => join(dir, FILE))
      // 3. 显示在页面上
      .then(setPath);
  }, []);

  // 作用: 将输入内容写入文件
  async function saveFile() {
    await mkdir(await appDataDir(), { recursive: true });
    await writeTextFile(FILE, text, opts);
    setContent("已保存");
  }

  // 作用: 读取文件内容并显示
  async function readFile() {
    setContent((await exists(FILE, opts)) ? await readTextFile(FILE, opts) : "文件不存在");
  }

  return (
    <section>
      <p>文件位置: {path}</p>
      <input
        style={{ marginRight: 5 }}
        value={text}
        onChange={(e) => setText(e.currentTarget.value)}
        placeholder="输入内容..."
      />
      <button onClick={saveFile}>保存</button>
      <button onClick={readFile}>读取</button>
      <p>{content}</p>
    </section>
  );
}

export default FileDemo;
