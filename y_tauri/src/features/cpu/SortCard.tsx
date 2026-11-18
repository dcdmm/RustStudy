import { useActionState } from "react";
import Card from "@/shared/components/Card.tsx";
import { attempt, type Attempt } from "@/shared/lib/attempt.ts";
import { sortNumbers, type Timing } from "./api.ts";
import RunForm from "./RunForm.tsx";
import TimingTable from "./TimingTable.tsx";

export default function SortCard() {
  const [result, action, pending] = useActionState(
    (_prev: Attempt<Timing[]> | null, formData: FormData) => attempt(sortNumbers(Number(formData.get("value")))),
    null,
  );

  return (
    <Card title="rayon::join: 并行快速排序">
      <RunForm label="排序的数字个数 =" defaultValue={5_000_000} max={10_000_000} pending={pending} action={action} />
      {result &&
        (result.ok ? (
          <TimingTable timings={result.data} />
        ) : (
          <p className="output error">错误: {result.error}</p>
        ))}
      <p className="hint">
        <code>rayon::join(a, b)</code>让分区后的左右两半同时排序;但第一次分区只能单线程完成,所以加速有限;
        rayon内置的<code>par_sort_unstable</code>优化得更充分,实际项目中直接用它
      </p>
    </Card>
  );
}
