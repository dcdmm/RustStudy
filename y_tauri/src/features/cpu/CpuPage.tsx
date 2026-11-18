import { useEffect, useState } from "react";
import Card from "@/shared/components/Card.tsx";
import PageHeader from "@/shared/components/PageHeader.tsx";
import { countPrimes, cpuThreads, sortNumbers, wordFrequency } from "./api.ts";
import BenchForm from "./BenchForm.tsx";
import TimingTable from "./TimingTable.tsx";

export default function CpuPage() {
  const [threads, setThreads] = useState<number | null>(null);

  useEffect(() => {
    let ignore = false;
    cpuThreads().then((n) => {
      if (!ignore) setThreads(n);
    });
    return () => {
      ignore = true;
    };
  }, []);

  return (
    <>
      <PageHeader
        title="8. CPU密集: rayon"
        description={`计算量大的任务用rayon分配到所有CPU核心上并行执行,本机rayon线程数: ${threads ?? "..."}`}
      />

      <Card title="统计质数: par_iter、rayon::join、线程池">
        <BenchForm label="统计2到N之间的质数,N =" defaultValue={2_000_000} max={20_000_000} run={countPrimes}>
          {(report) => (
            <>
              <p className="output">共 {report.count.toLocaleString()} 个质数</p>
              <TimingTable timings={report.timings} />
            </>
          )}
        </BenchForm>
      </Card>

      <Card title="词频统计: fold + reduce">
        <BenchForm label="生成的文本词数 =" defaultValue={5_000_000} max={10_000_000} run={wordFrequency}>
          {(report) => (
            <>
              <TimingTable timings={report.timings} />
              <p className="output">
                共 {report.uniqueWords} 个不同的词,出现最多的:{" "}
                {report.top.map((item) => `${item.word}(${item.count.toLocaleString()})`).join("、")}
              </p>
            </>
          )}
        </BenchForm>
      </Card>

      <Card title="排序: par_sort_unstable">
        <BenchForm label="排序的数字个数 =" defaultValue={5_000_000} max={10_000_000} run={sortNumbers}>
          {(timings) => <TimingTable timings={timings} />}
        </BenchForm>
      </Card>
    </>
  );
}
