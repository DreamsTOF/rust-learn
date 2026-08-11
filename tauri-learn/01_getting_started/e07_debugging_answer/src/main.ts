// ============================================================
// 练习 E07: 调试
// 目标: 掌握前端 DevTools（Web Inspector）与后端 println! 日志
// 知识点: console.log / DevTools 打开方式 / invoke 结果回传
// ============================================================

import { invoke } from "@tauri-apps/api/core";

const input = document.querySelector<HTMLInputElement>("#message");
const btn = document.querySelector<HTMLButtonElement>("#trace-btn");
const outputEl = document.querySelector<HTMLPreElement>("#output");

// 前端日志：DevTools Console 可见（dev 模式按 F12 或右键 → 检查）
console.log("[frontend] 页面加载完成，等待触发调试命令");

btn!.addEventListener("click", async () => {
  const message = input!.value.trim() || "hello tauri";
  console.log("[frontend] 发送给后端:", message);

  try {
    const lines = await invoke<string[]>("run_debug_trace", { message });
    console.log("[frontend] 后端返回:", lines);
    outputEl!.textContent = lines.join("\n");
  } catch (e) {
    console.error("[frontend] 调用失败:", e);
    outputEl!.textContent = `调用失败: ${e}`;
  }
});