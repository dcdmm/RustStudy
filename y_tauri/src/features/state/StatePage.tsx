import { useEffect, useState } from "react";
import { getCount, increment } from "./api.ts";
import Card from "@/shared/components/Card.tsx";
import PageHeader from "@/shared/components/PageHeader.tsx";

// 3. 状态管理: 数据保存在Rust端的托管状态(State)中
export default function StatePage() {
  // null表示还没从Rust读取到
  const [count, setCount] = useState<number | null>(null);

  // 页面打开时从Rust读取当前值
  useEffect(() => {
    // ignore: 组件已卸载时不再更新state(React官方推荐的写法)
    let ignore = false;
    getCount().then((n) => {
      if (!ignore) setCount(n);
    });
    return () => {
      ignore = true;
    };
  }, []);

  async function handleClick() {
    setCount(await increment());
  }

  return (
    <>
      <PageHeader
        title="3. 状态管理"
        description="用.manage()把数据交给Tauri托管,command参数中声明State<'_, T>即可拿到,前端调用时不需要传"
        sources={["src/features/state/", "src-tauri/src/commands/counter.rs", "src-tauri/src/lib.rs"]}
      />

      <Card title="计数器">
        <div className="row">
          <button onClick={handleClick}>+1</button>
          <span className="output">Rust中的count = {count ?? "..."}</span>
        </div>
        <p className="hint">切换到其他页面或按F5刷新后再回来,数值仍然保留: 数据保存在Rust进程中,而不是前端</p>
      </Card>
    </>
  );
}
