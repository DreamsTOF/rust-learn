// ============================================================
// 练习 E01: 计数器 —— 答案版
// 目标: invoke / #[tauri::command] / generate_handler! / serde
// ============================================================

import { invoke } from "@tauri-apps/api/core";

const valueEl = document.querySelector<HTMLSpanElement>("#value");
const btnEl = document.querySelector<HTMLButtonElement>("#increment");
const statusEl = document.querySelector<HTMLParagraphElement>("#status");

let count = 0;

async function increment() {
  try {
    count = await invoke<number>("count_up", { current: count });
    valueEl!.textContent = String(count);
    statusEl!.textContent = `后端返回: ${count}`;
  } catch (e) {
    statusEl!.textContent = `调用失败: ${e}`;
  }
}

btnEl!.addEventListener("click", increment);
