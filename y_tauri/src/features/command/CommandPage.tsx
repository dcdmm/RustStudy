import { useActionState } from "react";
import { greet } from "./api.ts";
import Card from "@/shared/components/Card.tsx";
import PageHeader from "@/shared/components/PageHeader.tsx";

// 1. Command: 调用Rust函数并获取返回值
export default function CommandPage() {
  // React 19的Action: 表单提交时执行,返回值成为新的state,isPending表示是否正在执行
  const [message, formAction, isPending] = useActionState(
    (_prev: string, formData: FormData) => greet(String(formData.get("name"))),
    "",
  );

  return (
    <>
      <PageHeader
        title="1. Command"
        description="前端通过invoke调用Rust中用#[tauri::command]标记的函数,参数和返回值自动通过JSON转换"
        sources={["src/features/command/", "src-tauri/src/commands/greet.rs"]}
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

      <Card title="调用过程">
        <ol>
          <li>
            前端: <code>invoke("greet", {"{ name }"})</code>,参数对象被序列化为JSON
          </li>
          <li>
            Rust: 把JSON反序列化为<code>name: &amp;str</code>,执行<code>greet</code>
          </li>
          <li>Rust: 把返回的String序列化为JSON</li>
          <li>前端: invoke返回的Promise resolve,得到字符串</li>
        </ol>
        <p className="hint">
          新写的command必须在<code>lib.rs</code>的<code>generate_handler![]</code>中注册,否则会报错: Command xxx not
          found
        </p>
      </Card>
    </>
  );
}
