import { useActionState } from "react";
import Card from "@/shared/components/Card.tsx";
import { attempt, type Attempt } from "@/shared/lib/attempt.ts";
import { wordFrequency, type WordFreqReport } from "./api.ts";
import RunForm from "./RunForm.tsx";
import TimingTable from "./TimingTable.tsx";

export default function WordFreqCard() {
  const [result, action, pending] = useActionState(
    (_prev: Attempt<WordFreqReport> | null, formData: FormData) =>
      attempt(wordFrequency(Number(formData.get("value")))),
    null,
  );

  return (
    <Card title="fold + reduce: 词频统计">
      <RunForm label="生成的文本词数 =" defaultValue={5_000_000} max={10_000_000} pending={pending} action={action} />
      {result &&
        (result.ok ? (
          <>
            <TimingTable timings={result.data.timings} />
            <p className="output">
              共 {result.data.uniqueWords} 个不同的词,出现最多的:{" "}
              {result.data.top
                .slice(0, 5)
                .map((item) => `${item.word}(${item.count.toLocaleString()})`)
                .join("、")}
            </p>
          </>
        ) : (
          <p className="output error">错误: {result.error}</p>
        ))}
      <p className="hint">
        <code>fold</code>让每块数据用自己的HashMap计数,<code>reduce</code>再把它们两两合并,整个过程不需要加锁
      </p>
    </Card>
  );
}
