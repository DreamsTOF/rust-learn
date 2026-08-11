// ============================================================
// 练习 E11: 错误处理
// 目标: 观察 thiserror 错误消息如何传回前端
// 知识点: try-catch / invoke 错误捕获
// ============================================================

import { invoke } from "@tauri-apps/api/core";

const numberInput = document.querySelector<HTMLInputElement>("#number");
const parseBtn = document.querySelector<HTMLButtonElement>("#parse-btn");
const readBtn = document.querySelector<HTMLButtonElement>("#read-btn");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

// 解析数字：输入非法时后端返回 InvalidInput 错误
parseBtn!.addEventListener("click", async () => {
  const input = numberInput!.value;
  try {
    const v = await invoke<number>("parse_number", { input });
    resultEl!.textContent = `解析成功：${input} × 2 = ${v}`;
    resultEl!.className = "status ok";
  } catch (e) {
    // Tauri 会把 Err 序列化为字符串（此处即 thiserror 的错误消息）
    resultEl!.textContent = `err: ${e}`;
    resultEl!.className = "status err";
  }
});

// 读文件：文件不存在时后端经 ? 传播 Io 错误
readBtn!.addEventListener("click", async () => {
  try {
    const text = await invoke<string>("read_marker");
    resultEl!.textContent = text;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `err: ${e}`;
    resultEl!.className = "status err";
  }
});