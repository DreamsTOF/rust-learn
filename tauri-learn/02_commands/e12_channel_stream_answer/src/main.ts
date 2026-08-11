// ============================================================
// 练习 E12: Channel 流式传输
// 目标: 创建 Channel 接收后端流式推送并实时渲染
// 知识点: new Channel<T> / onmessage / invoke 传 channel
// ============================================================

import { Channel, invoke } from "@tauri-apps/api/core";

// 与 Rust 端 StreamItem 对应
interface StreamItem {
  step: number;
  label: string;
}

const progressBtn = document.querySelector<HTMLButtonElement>("#progress-btn");
const streamBtn = document.querySelector<HTMLButtonElement>("#stream-btn");
const progressFill = document.querySelector<HTMLDivElement>("#progress-fill");
const progressText = document.querySelector<HTMLSpanElement>("#progress-text");
const streamList = document.querySelector<HTMLUListElement>("#stream-list");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

// 进度流：Channel<number> 接收 0-100 的进度值
progressBtn!.addEventListener("click", async () => {
  const ch = new Channel<number>();
  ch.onmessage = (msg) => {
    progressFill!.style.width = `${msg}%`;
    progressText!.textContent = `${msg}%`;
  };
  try {
    await invoke("start_progress", { channel: ch });
    resultEl!.textContent = "进度推送完成";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

// 消息流：Channel<StreamItem> 接收结构化消息并追加到列表
streamBtn!.addEventListener("click", async () => {
  const ch = new Channel<StreamItem>();
  ch.onmessage = (msg) => {
    const li = document.createElement("li");
    li.className = "ok";
    li.innerHTML = `<span class="badge">${msg.step}</span>${msg.label}`;
    streamList!.appendChild(li);
  };
  try {
    await invoke("start_stream", { channel: ch });
    resultEl!.textContent = "消息流推送完成";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});