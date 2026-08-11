import { defineConfig } from "vite";

// TAURI_DEV_HOST：局域网/移动设备调试时由 tauri CLI 注入（如 192.168.1.5），
// 有值时 Vite 监听该地址，HMR 也绑定到同一主机（见下方 hmr 配置）
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  clearScreen: false,
  server: {
    // 固定端口：必须与 tauri.conf.json 的 build.devUrl 保持一致
    port: 1499,
    // strictPort: 端口被占用时直接报错退出，而不是自动换端口
    // （自动换端口会导致 devUrl 与实际端口失配，应用加载不到页面）
    strictPort: true,
    host: host || false,
    // 远程调试（TAURI_DEV_HOST 有值）时 HMR 也走该主机与专用端口 1421
    hmr: host ? { protocol: "ws", host, port: 1421 } : undefined,
    // src-tauri 目录变化不触发前端热更新（Rust 代码由 cargo 重新编译）
    watch: { ignored: ["**/src-tauri/**"] },
  },
});