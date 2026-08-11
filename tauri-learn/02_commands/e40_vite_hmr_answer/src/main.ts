// ============================================================
// 练习 E40: Vite 与 HMR
// 目标: 查看构建环境信息，理解 devUrl / strictPort / HMR / TAURI_DEV_HOST
// 知识点: import.meta.env / devUrl / strictPort / HMR / watch.ignored
// ============================================================

// 本练习无 Rust 命令，纯前端信息页。
// 先看项目根目录 vite.config.ts 与 src-tauri/tauri.conf.json，再对照下方说明。

const modeEl = document.querySelector<HTMLSpanElement>("#env-mode");
const devEl = document.querySelector<HTMLSpanElement>("#env-dev");
const prodEl = document.querySelector<HTMLSpanElement>("#env-prod");

// import.meta.env 由 Vite 在编译时注入
// MODE: 当前模式名（默认 development / production）
// DEV:  dev 模式下为 true；PROD: build 后为 true
modeEl!.textContent = import.meta.env.MODE;
devEl!.textContent = String(import.meta.env.DEV);
prodEl!.textContent = String(import.meta.env.PROD);