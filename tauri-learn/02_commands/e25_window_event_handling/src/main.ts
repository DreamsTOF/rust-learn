// ============================================================
// 练习 E25: 窗口事件处理
// 目标: 实时查看窗口事件日志（拖拽/缩放/焦点变化）
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

const refreshBtn = document.querySelector<HTMLButtonElement>("#refresh-btn");
const logList = document.querySelector<HTMLUListElement>("#log-list");

// 拉取后端日志（最新在前）并渲染到列表
async function refreshLog() {
  try {
    // === 步骤 1: 拉取窗口日志 ————————————————————————————————————
    // TODO: const logs = await invoke<string[]>("get_window_log");
    // 提示: 返回的数组最新在前，直接按顺序渲染即可
    // 占位：完成填空后替换为真实调用
    const logs: string[] = [];
    logList!.innerHTML = "";
    if (logs.length === 0) {
      const li = document.createElement("li");
      li.className = "warn";
      li.innerHTML = `<span class="badge">!</span>暂无日志：拖拽窗口、调整大小或切换焦点试试`;
      logList!.appendChild(li);
      return;
    }
    for (const line of logs) {
      const li = document.createElement("li");
      li.className = "ok";
      li.innerHTML = `<span class="badge">•</span>${line}`;
      logList!.appendChild(li);
    }
  } catch (e) {
    const li = document.createElement("li");
    li.className = "warn";
    li.innerHTML = `<span class="badge">✕</span>刷新失败: ${e}`;
    logList!.appendChild(li);
  }
}

refreshBtn!.addEventListener("click", refreshLog);
refreshLog();