import { invoke } from "@tauri-apps/api/core";

export type OpenedFile = {
  path: string;
  size: number;
  lines: number;
  preview: string;
};

export type RecentFiles = {
  storePath: string;
  paths: string[];
};

export function openTextFile(path: string): Promise<OpenedFile> {
  return invoke<OpenedFile>("open_text_file", { path });
}

export function recentFiles(): Promise<RecentFiles> {
  return invoke<RecentFiles>("recent_files");
}
