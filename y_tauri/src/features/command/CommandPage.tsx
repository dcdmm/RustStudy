import { useActionState } from "react";
import { greet } from "./api.ts";
import Card from "@/shared/components/Card.tsx";
import PageHeader from "@/shared/components/PageHeader.tsx";

export default function CommandPage() {
  const [message, formAction, isPending] = useActionState(
    async (_prev: string, formData: FormData) => {
      try {
        return await greet(String(formData.get("name")));
      } catch (err) {
        return `错误: ${err}`;
      }
    },
    "",
  );

  return (
    <>
      <PageHeader
        title="1. Command"
        description="前端通过invoke调用Rust中用#[tauri::command]标记的函数,参数和返回值自动通过JSON转换"
      />

      <Card title="调用greet">
        <form className="row" action={formAction}>
          <input name="name" placeholder="输入名字..." required />
          <button type="submit" disabled={isPending}>
            打招呼
          </button>
        </form>
        <p className="output">{message}</p>
      </Card>
    </>
  );
}
