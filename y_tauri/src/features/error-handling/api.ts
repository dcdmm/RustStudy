// error-handling调用Rust的入口,对应Rust: commands/math.rs
import { invoke } from "@tauri-apps/api/core";

// Rust返回Err时Promise会reject,reject的值是AppError序列化后的字符串(见error.rs)
export function divide(a: number, b: number): Promise<number> {
  return invoke<number>("divide", { a, b });
}
