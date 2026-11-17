import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// 移动端开发时由Tauri CLI设置,桌面端开发时为undefined
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  // 使用tsconfig.app.json中的paths配置,让@/指向src/
  resolve: {
    tsconfigPaths: true,
  },

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
});
