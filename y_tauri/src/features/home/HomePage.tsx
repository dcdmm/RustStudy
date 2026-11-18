import { useEffect, useState } from "react";
import { getIdentifier, getName, getTauriVersion, getVersion } from "@tauri-apps/api/app";
import { systemInfo, type SystemInfo } from "./api.ts";
import Card from "@/shared/components/Card.tsx";
import PageHeader from "@/shared/components/PageHeader.tsx";

type AppInfo = {
  name: string;
  version: string;
  identifier: string;
  tauriVersion: string;
};

export default function HomePage() {
  const [appInfo, setAppInfo] = useState<AppInfo | null>(null);
  const [system, setSystem] = useState<SystemInfo | null>(null);

  useEffect(() => {
    let ignore = false;
    Promise.all([getName(), getVersion(), getIdentifier(), getTauriVersion()]).then(
      ([name, version, identifier, tauriVersion]) => {
        if (!ignore) setAppInfo({ name, version, identifier, tauriVersion });
      },
    );
    systemInfo().then((info) => {
      if (!ignore) setSystem(info);
    });
    return () => {
      ignore = true;
    };
  }, []);

  return (
    <>
      <PageHeader
        title="首页"
        description="Tauri: 用Web技术写界面,用Rust写后端逻辑,由系统自带的WebView渲染,打包成很小的桌面应用"
        sources={["src/features/home/", "src-tauri/src/commands/system.rs"]}
      />

      <Card title="应用信息: Tauri内置JS API">
        <dl className="info-list">
          <dt>名称</dt>
          <dd>{appInfo?.name}</dd>
          <dt>版本</dt>
          <dd>{appInfo?.version}</dd>
          <dt>标识符</dt>
          <dd>{appInfo?.identifier}</dd>
          <dt>Tauri版本</dt>
          <dd>{appInfo?.tauriVersion}</dd>
        </dl>
        <p className="hint">
          来自<code>@tauri-apps/api/app</code>,数据来源是<code>tauri.conf.json</code>
        </p>
      </Card>

      <Card title="系统信息: 自定义command返回结构体">
        <dl className="info-list">
          <dt>操作系统</dt>
          <dd>{system?.os}</dd>
          <dt>CPU架构</dt>
          <dd>{system?.arch}</dd>
          <dt>CPU核心数</dt>
          <dd>{system?.cpuCount}</dd>
        </dl>
        <p className="hint">
          Rust字段名<code>cpu_count</code>通过<code>#[serde(rename_all = "camelCase")]</code>变成了
          <code>cpuCount</code>
        </p>
      </Card>

      <Card title="如何学习">
        <ol>
          <li>按左侧菜单的顺序学习,每页一个知识点</li>
          <li>每页顶部列出了对应的源码,前端和Rust对照着看</li>
          <li>
            用<code>npm run tauri dev</code>运行,修改前端代码会热更新,修改Rust代码会自动重新编译
          </li>
        </ol>
      </Card>
    </>
  );
}
