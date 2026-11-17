// Rust广播的所有事件: 事件名 -> payload类型
// 与Rust端emit时使用的事件名和payload结构体一一对应(修改Rust时要同步修改这里)
export type EventMap = {
  // 对应Rust: uptime.rs中的UPTIME_EVENT和UptimePayload
  uptime: { uptimeSecs: number };
};
