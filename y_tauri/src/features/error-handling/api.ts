import { invoke } from "@tauri-apps/api/core";

export function divide(a: number, b: number): Promise<number> {
  return invoke<number>("divide", { a, b });
}
