import { Channel, invoke } from "@tauri-apps/api/core";

export type Outcome =
  | { kind: "response"; status: number; bytes: number }
  | { kind: "timeout" }
  | { kind: "failed"; message: string };

export type UrlCheck = {
  url: string;
  elapsedMs: number;
  outcome: Outcome;
};

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

export function checkUrls(
  urls: string[],
  options: CheckOptions,
  onResult: (check: UrlCheck) => void,
): Promise<CheckSummary> {
  const channel = new Channel<UrlCheck>(onResult);
  return invoke<CheckSummary>("check_urls", { urls, ...options, onResult: channel });
}
