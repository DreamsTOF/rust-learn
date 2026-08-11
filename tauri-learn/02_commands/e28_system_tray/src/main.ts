// ============================================================
// 练习 E28: 系统托盘
// 目标: 了解关闭隐藏到托盘 + 托盘恢复，前端提供「退出应用」按钮
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

const quitBtn = document.querySelector<HTMLButtonElement>("#quit-btn");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

quitBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 1: 调用退出命令 ————————————————————————————————————
    // TODO: await invoke("quit_app");
    // 提示: 调用成功后应用退出，无需展示结果
    // 占位：完成填空后替换为真实调用
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});