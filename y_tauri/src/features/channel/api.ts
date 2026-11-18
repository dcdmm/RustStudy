import { Channel, invoke } from "@tauri-apps/api/core";

export type CountdownEvent =
  | { event: "tick"; data: { remaining: number } }
  | { event: "finished" };

export function countdown(
  totalSeconds: number,
  onEvent: (event: CountdownEvent) => void,
): Promise<void> {
  const channel = new Channel<CountdownEvent>(onEvent);
  return invoke("countdown", { totalSeconds, onEvent: channel });
}
