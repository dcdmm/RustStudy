import { useActionState, useState, type ReactNode } from "react";

type BenchResult<T> = { ok: true; data: T } | { ok: false; error: string };

type BenchFormProps<T> = {
  label: string;
  defaultValue: number;
  max: number;
  run: (value: number) => Promise<T>;
  children: (data: T) => ReactNode;
};

export default function BenchForm<T>({ label, defaultValue, max, run, children }: BenchFormProps<T>) {
  const [value, setValue] = useState(String(defaultValue));
  const [result, action, pending] = useActionState(
    async (_prev: BenchResult<T> | null, formData: FormData): Promise<BenchResult<T>> => {
      try {
        return { ok: true, data: await run(Number(formData.get("value"))) };
      } catch (err) {
        return { ok: false, error: String(err) };
      }
    },
    null,
  );

  return (
    <>
      <form className="row" action={action}>
        <label className="row">
          {label}
          <input
            name="value"
            type="number"
            min={1}
            max={max}
            value={value}
            onChange={(e) => setValue(e.target.value)}
            required
          />
        </label>
        <button type="submit" disabled={pending}>
          {pending ? "计算中..." : "运行"}
        </button>
      </form>
      {result && (result.ok ? children(result.data) : <p className="output error">错误: {result.error}</p>)}
    </>
  );
}
