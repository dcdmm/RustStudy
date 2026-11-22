// 集中封装与Rust的交互: command名、参数名和返回类型只在此文件出现
// 页面调用普通的TS函数,不直接使用invoke
import { invoke } from "@tauri-apps/api/core";

// invoke抛出的是Rust返回的原始数据,在此统一转为Error实例,页面可用instanceof判断
export class ApiError extends Error {}

async function call<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (e) {
    throw new ApiError(String(e));
  }
}

export const listTodos = () => call<string[]>("list_todos");

export const addTodo = (text: string) => call<string[]>("add_todo", { text });

// Rust参数item_index在JS中须写作itemIndex,封装后页面无需关心
export const removeTodo = (itemIndex: number) => call<string[]>("remove_todo", { itemIndex });
