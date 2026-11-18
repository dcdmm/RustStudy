import { useActionState, useState } from "react";
import { divide } from "./api.ts";
import Card from "@/shared/components/Card.tsx";
import PageHeader from "@/shared/components/PageHeader.tsx";

type DivideResult = { ok: true; value: number } | { ok: false; error: string };

async function calculate(_prev: DivideResult | null, formData: FormData): Promise<DivideResult> {
  try {
    const value = await divide(Number(formData.get("a")), Number(formData.get("b")));
    return { ok: true, value };
  } catch (err) {
    return { ok: false, error: String(err) };
  }
}

export default function ErrorHandlingPage() {
  const [result, formAction, isPending] = useActionState(calculate, null);
  const [a, setA] = useState("10");
  const [b, setB] = useState("0");

  return (
    <>
      <PageHeader
        title="2. 错误处理"
        description="command返回Result: Ok时Promise resolve,Err时Promise reject;错误类型用thiserror定义,并实现Serialize"
        sources={["src/features/error-handling/", "src-tauri/src/commands/math.rs", "src-tauri/src/error.rs"]}
      />

      <Card title="调用divide(除数为0时返回Err)">
        <form className="row" action={formAction}>
          <input name="a" type="number" step="any" value={a} onChange={(e) => setA(e.target.value)} required />
          <span>÷</span>
          <input name="b" type="number" step="any" value={b} onChange={(e) => setB(e.target.value)} required />
          <button type="submit" disabled={isPending}>
            计算
          </button>
        </form>
        {result && (
          <p className={result.ok ? "output" : "output error"}>
            {result.ok ? `结果: ${result.value}` : `错误: ${result.error}`}
          </p>
        )}
      </Card>
    </>
  );
}
