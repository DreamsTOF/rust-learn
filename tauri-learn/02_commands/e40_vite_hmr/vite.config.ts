import { defineConfig } from "vite";

// TAURI_DEV_HOST：局域网/移动设备调试时由 tauri CLI 注入（如 192.168.1.5）
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  clearScreen: false,
  server: {
    // === 步骤 1: 固定开发端口 ————————————————————————————————————
    // TODO: 把端口改为 1498（与 src-tauri/tauri.conf.json 的 build.devUrl 一致）
    // 提示: 当前 port: 0 表示随机选一个空闲端口，可以编译运行；
    //       但 devUrl 写死了端口，只有两者一致时 tauri dev 才能加载页面
    port: 0,
    // === 步骤 2: 端口冲突行为 ————————————————————————————————————
    // TODO: 取消注释并补全，说明 strictPort 的作用：
    //   strictPort: true,  // 端口被占用时直接报错，而不是自动换端口
    // 提示: 自动换端口会让 devUrl 与实际端口失配；配合步骤 1 一起完成
    // === 步骤 3: 远程调试 HMR ————————————————————————————————————
    // TODO: 取消注释并补全，说明 TAURI_DEV_HOST 场景：
    //   hmr: host ? { protocol: "ws", host, port: 1421 } : undefined,
    // 提示: 设置 TAURI_DEV_HOST（如 192.168.1.5）后 Vite 监听局域网地址，
    //       手机/局域网设备可直接访问 devUrl；HMR 也要绑定同一主机才能热更新
    host: host || false,
    // src-tauri 目录变化不触发前端热更新（Rust 代码由 cargo 重新编译）
    watch: { ignored: ["**/src-tauri/**"] },
  },
});