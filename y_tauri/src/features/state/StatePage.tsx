import { useEffect, useState } from "react";
import { getCount, increment } from "./api.ts";
import Card from "@/shared/components/Card.tsx";
import PageHeader from "@/shared/components/PageHeader.tsx";

export default function StatePage() {
  const [count, setCount] = useState<number | null>(null);

  useEffect(() => {
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
      />

      <Card title="计数器">
        <div className="row">
          <button onClick={handleClick}>+1</button>
          <span className="output">Rust中的count = {count ?? "..."}</span>
        </div>
      </Card>
    </>
  );
}
