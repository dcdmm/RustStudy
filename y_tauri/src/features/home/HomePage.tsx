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
  const [error, setError] = useState("");

  useEffect(() => {
    let ignore = false;
    const onError = (err: unknown) => {
      if (!ignore) setError(String(err));
    };
    Promise.all([getName(), getVersion(), getIdentifier(), getTauriVersion()])
      .then(([name, version, identifier, tauriVersion]) => {
        if (!ignore) setAppInfo({ name, version, identifier, tauriVersion });
      })
      .catch(onError);
    systemInfo()
      .then((info) => {
        if (!ignore) setSystem(info);
      })
      .catch(onError);
    return () => {
      ignore = true;
    };
  }, []);

  return (
    <>
      <PageHeader
        title="首页"
        description="Tauri: 用Web技术写界面,用Rust写后端逻辑,由系统自带的WebView渲染,打包成很小的桌面应用"
      />
      {error && <p className="output error">错误: {error}</p>}

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
      </Card>
    </>
  );
}
