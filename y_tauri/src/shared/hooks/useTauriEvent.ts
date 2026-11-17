import { useEffect, useEffectEvent } from "react";
import { listen } from "@tauri-apps/api/event";
import type { EventMap } from "../tauri/events.ts";

// 在组件中监听Rust广播的事件,组件卸载时自动取消监听
// 事件名只能是EventMap中定义的,handler的参数类型也会根据事件名自动推断
export function useTauriEvent<K extends keyof EventMap>(
  name: K,
  handler: (payload: EventMap[K]) => void,
) {
  // useEffectEvent: 回调里总能拿到最新的handler,又不会因为handler变化而重新注册监听
  const onEvent = useEffectEvent(handler);

  useEffect(() => {
    // listen是异步的,返回Promise<取消监听的函数>
    const unlisten = listen<EventMap[K]>(name, (event) => onEvent(event.payload));
    // 组件卸载时取消监听,否则会重复注册(开发模式下StrictMode会让组件挂载两次)
    return () => {
      unlisten.then((off) => off());
    };
  }, [name]);
}
