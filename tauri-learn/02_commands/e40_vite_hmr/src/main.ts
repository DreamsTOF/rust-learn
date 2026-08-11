// ============================================================
// 练习 E40: Vite 与 HMR
// 目标: 查看构建环境信息，理解 devUrl / strictPort / HMR / TAURI_DEV_HOST
// 知识点: import.meta.env / devUrl / strictPort / HMR / watch.ignored
// TODO: 按照注释提示补全代码
// ============================================================

// 本练习无 Rust 命令，纯前端信息页。
// 先看项目根目录 vite.config.ts 与 src-tauri/tauri.conf.json，再对照下方说明。

const modeEl = document.querySelector<HTMLSpanElement>("#env-mode");
const devEl = document.querySelector<HTMLSpanElement>("#env-dev");
const prodEl = document.querySelector<HTMLSpanElement>("#env-prod");

// === 步骤 1: 展示构建环境信息 ——————————————————————————————————
// TODO: 补全展示代码：
//   modeEl!.textContent = import.meta.env.MODE;
//   prodEl!.textContent = String(import.meta.env.PROD);
// 提示: DEV 在 vite dev 下为 true，PROD 在 build 后为 true，
//       MODE 默认 development / production，由 Vite 编译时注入
// 当前为占位（完成填空后替换；DEV 一行作为参考保留）
devEl!.textContent = String(import.meta.env.DEV);
modeEl!.textContent = "—";
prodEl!.textContent = "—";