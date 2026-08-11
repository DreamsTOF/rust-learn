// ============================================================
// 练习 E16: setup 钩子
// 目标: 展示 setup 初始化状态与后端事件推送
// 知识点: listen 事件监听 / invoke 查询状态
// ============================================================

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const statusEl = document.querySelector<HTMLSpanElement>("#setup-status");
const refreshBtn = document.querySelector<HTMLButtonElement>("#refresh-btn");

// 查询后端 setup 中写入的初始化状态
async function refresh() {
  try {
    const s = await invoke<string>("get_setup_state");
    statusEl!.textContent = s;
    statusEl!.className = "status";
  } catch (e) {
    statusEl!.textContent = `err: ${e}`;
    statusEl!.className = "status err";
  }
}

// 监听后端在 setup 中广播的初始化完成事件
listen<string>("init-done", (event) => {
  statusEl!.textContent = event.payload;
  statusEl!.className = "status ok";
}).catch((e) => console.error("监听失败", e));

refreshBtn!.addEventListener("click", refresh);

refresh();