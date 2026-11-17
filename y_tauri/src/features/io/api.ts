// io调用Rust的入口,对应Rust: commands/io.rs(请求逻辑在crates/io_tasks中)
import { Channel, invoke } from "@tauri-apps/api/core";

// 对应Rust: crates/io_tasks中的Outcome枚举,通过kind字段区分
export type Outcome =
  | { kind: "response"; status: number; bytes: number }
  | { kind: "timeout" }
  | { kind: "failed"; message: string };

// 对应Rust: crates/io_tasks中的UrlCheck结构体
export type UrlCheck = {
  url: string;
  elapsedMs: number;
  outcome: Outcome;
};

// 对应Rust: commands/io.rs中的CheckMode枚举
export type CheckMode = "sequential" | "concurrent";

export type CheckOptions = {
  mode: CheckMode;
  maxConcurrency: number;
  timeoutMs: number;
};

export type CheckSummary = {
  count: number;
  totalMs: number;
};

// 每完成一个网址的检测,onResult就执行一次;全部完成后Promise resolve,得到总耗时
export function checkUrls(
  urls: string[],
  options: CheckOptions,
  onResult: (check: UrlCheck) => void,
): Promise<CheckSummary> {
  const channel = new Channel<UrlCheck>(onResult);
  return invoke<CheckSummary>("check_urls", { urls, ...options, onResult: channel });
}
