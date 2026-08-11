// ============================================================
// 练习 E09: 异步命令
// 目标: 调用 async 命令，观察延迟返回与超时错误
// 知识点: invoke 异步调用 / 参数 camelCase / try-catch 错误捕获
// ============================================================

import { invoke } from "@tauri-apps/api/core";

const messageInput = document.querySelector<HTMLInputElement>("#message");
const slowBtn = document.querySelector<HTMLButtonElement>("#slow-btn");
const timeoutBtn = document.querySelector<HTMLButtonElement>("#timeout-btn");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

// 慢速回显：调用 async 命令，约 2 秒后返回
slowBtn!.addEventListener("click", async () => {
  const message = messageInput!.value.trim() || "Tauri";
  resultEl!.textContent = "慢速回显执行中（约 2 秒）...";
  resultEl!.className = "status";
  try {
    // Rust 参数 delay_ms 在 JS 侧写作 delayMs（camelCase）
    const text = await invoke<string>("slow_echo", { message, delayMs: 2000 });
    resultEl!.textContent = text;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

// 超时演示：1 秒超时 vs 3 秒任务
timeoutBtn!.addEventListener("click", async () => {
  const message = messageInput!.value.trim() || "Tauri";
  resultEl!.textContent = "任务执行中（3 秒），1 秒后将超时中断...";
  resultEl!.className = "status";
  try {
    const text = await invoke<string>("run_with_timeout", { message, timeoutMs: 1000 });
    resultEl!.textContent = text;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `err: ${e}`;
    resultEl!.className = "status err";
  }
});