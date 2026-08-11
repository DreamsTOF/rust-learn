// ============================================================
// 练习 E28: 系统托盘
// 目标: 了解关闭隐藏到托盘 + 托盘恢复，前端提供「退出应用」按钮
// 知识点: invoke / 托盘交互说明
// ============================================================

import { invoke } from "@tauri-apps/api/core";

const quitBtn = document.querySelector<HTMLButtonElement>("#quit-btn");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

quitBtn!.addEventListener("click", async () => {
  try {
    await invoke("quit_app");
    // 调用成功后应用退出，无需展示结果
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});