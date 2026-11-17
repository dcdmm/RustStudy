// home调用Rust的入口,对应Rust: commands/system.rs
import { invoke } from "@tauri-apps/api/core";

// 对应Rust中的SystemInfo结构体(字段名已由serde转为camelCase)
export type SystemInfo = {
  os: string;
  arch: string;
  cpuCount: number;
};

export function systemInfo(): Promise<SystemInfo> {
  return invoke<SystemInfo>("system_info");
}
