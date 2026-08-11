// ============================================================
// 练习 E09: 异步命令
// 目标: 调用 async 命令，观察延迟返回与超时错误
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

const messageInput = document.querySelector<HTMLInputElement>("#message");
const slowBtn = document.querySelector<HTMLButtonElement>("#slow-btn");
const timeoutBtn = document.querySelector<HTMLButtonElement>("#timeout-btn");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

// 慢速回显：调用 async 命令，约 2 秒后返回
slowBtn!.addEventListener("click", async () => {
  resultEl!.textContent = "慢速回显执行中（约 2 秒）...";
  resultEl!.className = "status";
  try {
    // === 步骤 4: 调用 slow_echo ————————————————————————————————————
    // TODO: 读取输入框并调用慢速回显命令：
    //   const message = messageInput!.value.trim() || "Tauri";
    //   const text = await invoke<string>("slow_echo", { message, delayMs: 2000 });
    // 提示: Rust 参数 delay_ms 在 JS 侧写作 delayMs（camelCase）
    // 占位：完成填空后替换为真实调用结果（当前先展示输入框内容）
    const text: string = messageInput!.value.trim() || "Tauri";
    resultEl!.textContent = text;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

// 超时演示：1 秒超时 vs 3 秒任务
timeoutBtn!.addEventListener("click", async () => {
  resultEl!.textContent = "任务执行中（3 秒），1 秒后将超时中断...";
  resultEl!.className = "status";
  try {
    // === 步骤 5: 调用 run_with_timeout ————————————————————————————————
    // TODO: 读取输入框并调用超时演示命令：
    //   const message = messageInput!.value.trim() || "Tauri";
    //   const text = await invoke<string>("run_with_timeout", { message, timeoutMs: 1000 });
    // 提示: 超时后后端返回 Err("操作超时")，会被 catch 捕获
    // 占位：完成填空后替换为真实调用结果（当前先展示输入框内容）
    const text: string = messageInput!.value.trim() || "Tauri";
    resultEl!.textContent = text;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `err: ${e}`;
    resultEl!.className = "status err";
  }
});