// state调用Rust的入口,对应Rust: commands/counter.rs
import { invoke } from "@tauri-apps/api/core";

export function getCount(): Promise<number> {
  return invoke<number>("get_count");
}

export function increment(): Promise<number> {
  return invoke<number>("increment");
}
