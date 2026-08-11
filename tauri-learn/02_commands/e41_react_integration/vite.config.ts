import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  // React 支持：JSX 转换等由 @vitejs/plugin-react 提供
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1500,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: "ws", host, port: 1421 } : undefined,
    watch: { ignored: ["**/src-tauri/**"] },
  },
});