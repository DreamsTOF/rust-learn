// ============================================================
// 练习 E07: 调试
// 目标: 掌握前端 DevTools（Web Inspector）与后端 println! 日志
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

const input = document.querySelector<HTMLInputElement>("#message");
const btn = document.querySelector<HTMLButtonElement>("#trace-btn");
const outputEl = document.querySelector<HTMLPreElement>("#output");

// === 步骤 1: 前端日志 ————————————————————————————————————
// TODO: 用 console.log 输出一条页面加载完成日志
// 提示: console.log("[frontend] 页面加载完成，等待触发调试命令")

btn!.addEventListener("click", async () => {
  // TODO: 读取输入框并打印发送日志：
  //   const message = input!.value.trim() || "hello tauri";
  //   console.log("[frontend] 发送给后端:", message)

  try {
    // === 步骤 2: 调用后端命令 ————————————————————————————————
    // TODO: 改为调用真实命令：
    //   const lines = await invoke<string[]>("run_debug_trace", { message });
    // 当前为空数组占位（保持可编译）
    const lines: string[] = [];

    // === 步骤 3: 展示结果 ——————————————————————————————————
    // TODO: 把 lines 以换行拼接显示到 #output
    // 提示: outputEl!.textContent = lines.join("\n");
    outputEl!.textContent = lines.join("\n");
  } catch (e) {
    // TODO: 用 console.error 输出失败原因
    // 提示: console.error("[frontend] 调用失败:", e)
    outputEl!.textContent = `调用失败: ${e}`;
  }
});