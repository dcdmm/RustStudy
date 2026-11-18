import { useState } from "react";

type RunFormProps = {
  label: string;
  defaultValue: number;
  max: number;
  pending: boolean;
  action: (formData: FormData) => void;
};

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
