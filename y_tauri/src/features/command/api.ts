// command调用Rust的入口,对应Rust: commands/greet.rs
//
// 每个feature都把调用Rust的代码放在自己的api.ts中:
// 1. 页面中不直接写invoke("xxx"),而是调用这里的函数,command名和参数名只在这里出现一次
// 2. 参数和返回值都标注了TS类型,与Rust端的函数签名一一对应(修改Rust时要同步修改这里)
import { invoke } from "@tauri-apps/api/core";

export function greet(name: string): Promise<string> {
  return invoke<string>("greet", { name });
}
