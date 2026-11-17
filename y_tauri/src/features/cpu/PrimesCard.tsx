import { useActionState } from "react";
import Card from "@/shared/components/Card.tsx";
import { attempt, type Attempt } from "@/shared/lib/attempt.ts";
import { countPrimes, type PrimesReport } from "./api.ts";
import RunForm from "./RunForm.tsx";
import TimingTable from "./TimingTable.tsx";

// 并行迭代器: 对应crates/cpu_tasks/src/primes.rs
export default function PrimesCard() {
  const [result, action, pending] = useActionState(
    (_prev: Attempt<PrimesReport> | null, formData: FormData) =>
      attempt(countPrimes(Number(formData.get("value")))),
    null,
  );

  return (
    <Card title="并行迭代器: 统计质数">
      <RunForm label="统计2到N之间的质数,N =" defaultValue={5_000_000} max={20_000_000} pending={pending} action={action} />
      {result &&
        (result.ok ? (
          <>
            <p className="output">共 {result.data.count.toLocaleString()} 个质数</p>
            <TimingTable timings={result.data.timings} />
          </>
        ) : (
          <p className="output error">错误: {result.error}</p>
        ))}
      <p className="hint">
        两个版本只差一处: <code>(2..=limit).into_iter()</code>换成<code>into_par_iter()</code>
        ;越大的数判断越慢,rayon的工作窃取会自动均衡各线程的负载
      </p>
    </Card>
  );
}
