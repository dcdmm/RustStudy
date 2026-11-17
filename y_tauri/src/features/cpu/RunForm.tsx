import { useState } from "react";

type RunFormProps = {
  label: string;
  defaultValue: number;
  max: number;
  pending: boolean;
  action: (formData: FormData) => void;
};

// 一个数字输入框 + 运行按钮,输入的值以"value"为名提交
// 输入框用受控组件: React 19在表单Action执行后会重置非受控输入框,受控的则保留输入
export default function RunForm({ label, defaultValue, max, pending, action }: RunFormProps) {
  const [value, setValue] = useState(String(defaultValue));

  return (
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
  );
}
