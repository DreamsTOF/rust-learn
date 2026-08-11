// ============================================================
// 练习 E16: setup 钩子
// 目标: 展示 setup 初始化状态与后端事件推送
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 调用命令；listen 监听后端事件）
// import { invoke } from "@tauri-apps/api/core";
// import { listen } from "@tauri-apps/api/event";

const statusEl = document.querySelector<HTMLSpanElement>("#setup-status");
const refreshBtn = document.querySelector<HTMLButtonElement>("#refresh-btn");

// 查询后端 setup 中写入的初始化状态
async function refresh() {
  try {
    // === 步骤 6: 查询初始化状态 ————————————————————————————————————
    // TODO: const s = await invoke<string>("get_setup_state");
    // 占位：完成填空后替换为真实调用结果
    const s: string = "未初始化";
    statusEl!.textContent = s;
    statusEl!.className = "status";
  } catch (e) {
    statusEl!.textContent = `err: ${e}`;
    statusEl!.className = "status err";
  }
}

// === 步骤 7: 监听初始化完成事件 ————————————————————————————————————
// TODO: listen<string>("init-done", (event) => {
//         statusEl!.textContent = event.payload;
//         statusEl!.className = "status ok";
//       }).catch((e) => console.error("监听失败", e));
// 提示: listen 来自 "@tauri-apps/api/event"，返回 Promise<UnlistenFn>；
//       应用启动约 1 秒后后端会广播 init-done 事件

refreshBtn!.addEventListener("click", refresh);

refresh();