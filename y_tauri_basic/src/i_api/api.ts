// 封装与Rust的交互,页面不直接使用invoke
import { invoke } from "@tauri-apps/api/core";

// 统一的错误类型,页面可用instanceof判断
export class ApiError extends Error { }

// 调用command,失败时将Rust的Err转为ApiError;T为返回值类型
async function call<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (e) {
    throw new ApiError(String(e));
  }
}

// 每个command对应一个函数,名称和参数须与Rust一致
export const listTodos = () => call<string[]>("list_todos");

export const addTodo = (text: string) => call<string[]>("add_todo", { text });

export const removeTodo = (itemIndex: number) => call<string[]>("remove_todo", { itemIndex });
