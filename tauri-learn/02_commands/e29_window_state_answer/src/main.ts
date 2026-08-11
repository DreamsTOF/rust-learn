// ============================================================
// 练习 E29: 窗口状态持久化
// 目标: 保存/清除窗口状态，重启后验证自动恢复
// 知识点: invoke 调用 save_window_state / clear_window_state
// ============================================================

import { invoke } from "@tauri-apps/api/core";

const saveBtn = document.querySelector<HTMLButtonElement>("#save-btn");
const clearBtn = document.querySelector<HTMLButtonElement>("#clear-btn");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

saveBtn!.addEventListener("click", async () => {
  try {
    const message = await invoke<string>("save_window_state");
    resultEl!.textContent = message;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `保存失败: ${e}`;
    resultEl!.className = "status err";
  }
});

clearBtn!.addEventListener("click", async () => {
  try {
    const message = await invoke<string>("clear_window_state");
    resultEl!.textContent = message;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `清除失败: ${e}`;
    resultEl!.className = "status err";
  }
});