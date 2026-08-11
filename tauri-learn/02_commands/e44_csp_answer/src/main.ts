// ============================================================
// 练习 E44: 内容安全策略（CSP）
// 目标: 理解 security.csp 配置，并用命令读取当前 CSP 字符串
// 知识点: invoke / CSP 指令 / 内联样式与 style-src
// ============================================================

import { invoke } from "@tauri-apps/api/core";

const cspEl = document.querySelector<HTMLPreElement>("#csp-value");

async function loadCsp(): Promise<void> {
  try {
    // 前端无法直接读 tauri.conf.json，通过后端命令读取配置
    const csp = await invoke<string>("get_csp");
    cspEl!.textContent = csp || "（未配置：app.security.csp 为 null）";
  } catch (e) {
    cspEl!.textContent = `读取失败: ${e}`;
  }
}

loadCsp().catch((e) => {
  cspEl!.textContent = `加载失败: ${e}`;
});