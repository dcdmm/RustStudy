// channel调用Rust的入口,对应Rust: commands/countdown.rs
import { Channel, invoke } from "@tauri-apps/api/core";

// 对应Rust中的CountdownEvent枚举
// 联合类型(discriminated union),通过event字段区分是哪一种消息
export type CountdownEvent =
  | { event: "tick"; data: { remaining: number } }
  | { event: "finished" };

// 1. 创建Channel并作为参数传给Rust,Rust每send一次,onEvent就执行一次
// 2. Rust参数名total_seconds/on_event -> 前端写成camelCase: totalSeconds/onEvent
// 3. 返回的Promise在倒计时全部结束后才resolve
export function countdown(
  totalSeconds: number,
  onEvent: (event: CountdownEvent) => void,
): Promise<void> {
  const channel = new Channel<CountdownEvent>(onEvent);
  return invoke("countdown", { totalSeconds, onEvent: channel });
}
