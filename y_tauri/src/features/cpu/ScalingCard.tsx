import { useActionState } from "react";
import Card from "@/shared/components/Card.tsx";
import { attempt, type Attempt } from "@/shared/lib/attempt.ts";
import { primeScaling, type Timing } from "./api.ts";
import RunForm from "./RunForm.tsx";
import TimingTable from "./TimingTable.tsx";

// 自定义线程池: 对应crates/cpu_tasks/src/pool.rs
export default function ScalingCard() {
  const [result, action, pending] = useActionState(
    (_prev: Attempt<Timing[]> | null, formData: FormData) => attempt(primeScaling(Number(formData.get("value")))),
    null,
  );

  return (
    <Card title="线程池: 线程数与耗时">
      <RunForm label="统计2到N之间的质数,N =" defaultValue={5_000_000} max={20_000_000} pending={pending} action={action} />
      {result &&
        (result.ok ? (
          <TimingTable timings={result.data} />
        ) : (
          <p className="output error">错误: {result.error}</p>
        ))}
      <p className="hint">
        用<code>ThreadPoolBuilder::new().num_threads(n)</code>创建线程池,<code>pool.install()</code>
        中的并行操作只使用这个线程池;线程数超过CPU物理核心数后,耗时基本不再下降
      </p>
    </Card>
  );
}
