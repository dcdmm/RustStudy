import { Channel, invoke } from "@tauri-apps/api/core";

export type Timing = {
  label: string;
  ms: number;
};

export type PrimesReport = {
  count: number;
  timings: Timing[];
};

export type WordCount = {
  word: string;
  count: number;
};

export type WordFreqReport = {
  uniqueWords: number;
  top: WordCount[];
  timings: Timing[];
};

export function cpuThreads(): Promise<number> {
  return invoke<number>("cpu_threads");
}

export function countPrimes(limit: number): Promise<PrimesReport> {
  return invoke<PrimesReport>("count_primes", { limit });
}

export function wordFrequency(wordCount: number): Promise<WordFreqReport> {
  return invoke<WordFreqReport>("word_frequency", { wordCount });
}

export function sortNumbers(len: number): Promise<Timing[]> {
  return invoke<Timing[]>("sort_numbers", { len });
}

export function countPrimesWithProgress(limit: number, onProgress: (percent: number) => void): Promise<number | null> {
  const channel = new Channel<number>(onProgress);
  return invoke<number | null>("count_primes_with_progress", { limit, onProgress: channel });
}

export function cancelCountPrimes(): Promise<void> {
  return invoke("cancel_count_primes");
}
