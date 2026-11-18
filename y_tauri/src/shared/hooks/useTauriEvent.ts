import { useEffect, useEffectEvent } from "react";
import { listen } from "@tauri-apps/api/event";
import type { EventMap } from "../tauri/events.ts";

export function useTauriEvent<K extends keyof EventMap>(
  name: K,
  handler: (payload: EventMap[K]) => void,
) {
  const onEvent = useEffectEvent(handler);

  useEffect(() => {
    const unlisten = listen<EventMap[K]>(name, (event) => onEvent(event.payload));
    return () => {
      unlisten.then((off) => off());
    };
  }, [name]);
}
