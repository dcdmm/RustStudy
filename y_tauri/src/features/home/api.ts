import { invoke } from "@tauri-apps/api/core";

export type SystemInfo = {
  os: string;
  arch: string;
  cpuCount: number;
};

export function systemInfo(): Promise<SystemInfo> {
  return invoke<SystemInfo>("system_info");
}
