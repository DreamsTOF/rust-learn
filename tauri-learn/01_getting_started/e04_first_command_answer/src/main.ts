// ============================================================
// 练习 E04: 第一个命令
// 目标: 走通 #[tauri::command] → generate_handler! → invoke() 全链路
// 知识点: invoke() / 传参 / async-await / 错误捕获
// ============================================================

import { invoke } from "@tauri-apps/api/core";

const nameInput = document.querySelector<HTMLInputElement>("#name");
const greetBtn = document.querySelector<HTMLButtonElement>("#greet-btn");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

greetBtn!.addEventListener("click", async () => {
  const name = nameInput!.value.trim() || "Tauri";
  try {
    // invoke 第二参数字段名与 Rust 参数名对应（camelCase）
    const message = await invoke<string>("greet", { name });
    resultEl!.textContent = message;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});