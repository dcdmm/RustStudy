import { invoke } from "@tauri-apps/api/core";

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

export function primeScaling(limit: number): Promise<Timing[]> {
  return invoke<Timing[]>("prime_scaling", { limit });
}

export function wordFrequency(wordCount: number): Promise<WordFreqReport> {
  return invoke<WordFreqReport>("word_frequency", { wordCount });
}

export function sortNumbers(len: number): Promise<Timing[]> {
  return invoke<Timing[]>("sort_numbers", { len });
}
