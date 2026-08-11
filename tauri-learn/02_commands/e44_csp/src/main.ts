// ============================================================
// 练习 E44: 内容安全策略（CSP）
// 目标: 理解 security.csp 配置，并用命令读取当前 CSP 字符串
// 知识点: invoke / CSP 指令 / 内联样式与 style-src
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

const cspEl = document.querySelector<HTMLPreElement>("#csp-value");

async function loadCsp(): Promise<void> {
  try {
    // === 步骤 1: 调用 get_csp 命令 ——————————————————————————
    // TODO: 调用后端命令并把结果赋给 csp 变量
    // 提示: const csp = await invoke<string>("get_csp");
    // 当前为空字符串占位（保持可编译），完成填空后将显示配置的 CSP
    let csp = "";

    // === 步骤 2: 展示结果 ——————————————————————————————————
    // TODO: 把 csp 显示到 #csp-value（空字符串时提示"未配置"）
    // 提示: cspEl!.textContent = csp || "（未配置：app.security.csp 为 null）";
    cspEl!.textContent = csp || "（未配置：app.security.csp 为 null）";
  } catch (e) {
    cspEl!.textContent = `读取失败: ${e}`;
  }
}

loadCsp().catch((e) => {
  cspEl!.textContent = `加载失败: ${e}`;
});