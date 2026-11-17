import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
// @ts-expect-error type error without @types/node package
import process from "node:process";

// 移动端开发时由Tauri CLI设置,桌面端开发时为undefined
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(() => ({
  plugins: [react()],

  // 以下配置专门为Tauri准备
  //
  // 1. 不清屏,避免Vite的输出把Rust的编译错误冲掉
  clearScreen: false,
  // 2. Tauri通过tauri.conf.json中的devUrl加载页面,所以端口必须固定,被占用时直接报错
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. 不监听src-tauri,Rust代码的变化由Tauri CLI负责重新编译
      ignored: ["**/src-tauri/**"],
    },
  },
}));
