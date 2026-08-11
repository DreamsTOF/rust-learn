// ============================================================
// 练习 E29: 窗口状态持久化
// 目标: 保存/清除窗口状态，重启后验证自动恢复
// 知识点: invoke 调用 save_window_state / clear_window_state
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

const saveBtn = document.querySelector<HTMLButtonElement>("#save-btn");
const clearBtn = document.querySelector<HTMLButtonElement>("#clear-btn");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

saveBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 1: 调用保存命令 ————————————————————————————————
    // TODO: const message = await invoke<string>("save_window_state");
    // 提示: 该命令无参数，保存全部窗口状态到磁盘
    let message = "";

    resultEl!.textContent = message;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `保存失败: ${e}`;
    resultEl!.className = "status err";
  }
});

clearBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 2: 调用清除命令 ————————————————————————————————
    // TODO: const message = await invoke<string>("clear_window_state");
    // 提示: 该命令会删除磁盘上的窗口状态文件
    let message = "";

    resultEl!.textContent = message;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `清除失败: ${e}`;
    resultEl!.className = "status err";
  }
});