// ============================================================
// 练习 E39: 全局快捷键
// 目标: 输入组合键，注册/注销/查询全局快捷键，并接收按键事件日志
// 知识点: invoke 调命令 / listen 接收后端广播 / 失焦仍生效 / 冲突报错
// ============================================================

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const comboInput = document.querySelector<HTMLInputElement>("#combo");
const registerBtn = document.querySelector<HTMLButtonElement>("#register-btn");
const unregisterBtn = document.querySelector<HTMLButtonElement>("#unregister-btn");
const checkBtn = document.querySelector<HTMLButtonElement>("#check-btn");
const statusEl = document.querySelector<HTMLParagraphElement>("#status");
const logEl = document.querySelector<HTMLDivElement>("#log");

// 监听后端广播的快捷键按下事件（全局快捷键在应用失焦时依然生效）
listen<string>("shortcut-pressed", (event) => {
  const line = document.createElement("div");
  line.textContent = `[${new Date().toLocaleTimeString()}] 按下: ${event.payload}`;
  logEl!.prepend(line);
});

// 注册
registerBtn!.addEventListener("click", async () => {
  const combo = comboInput!.value.trim();
  try {
    const msg = await invoke<string>("register_shortcut", { combo });
    statusEl!.textContent = msg;
    statusEl!.className = "status ok";
  } catch (e) {
    // 与系统或其他应用快捷键冲突时注册失败——错误展示也是教学点
    statusEl!.textContent = `注册失败: ${e}`;
    statusEl!.className = "status err";
  }
});

// 注销
unregisterBtn!.addEventListener("click", async () => {
  const combo = comboInput!.value.trim();
  try {
    const msg = await invoke<string>("unregister_shortcut", { combo });
    statusEl!.textContent = msg;
    statusEl!.className = "status ok";
  } catch (e) {
    statusEl!.textContent = `注销失败: ${e}`;
    statusEl!.className = "status err";
  }
});

// 查询状态
checkBtn!.addEventListener("click", async () => {
  const combo = comboInput!.value.trim();
  try {
    const registered = await invoke<boolean>("is_shortcut_registered", { combo });
    statusEl!.textContent = `${combo} 当前状态: ${registered ? "已注册" : "未注册"}`;
    statusEl!.className = "status ok";
  } catch (e) {
    statusEl!.textContent = `查询失败: ${e}`;
    statusEl!.className = "status err";
  }
});