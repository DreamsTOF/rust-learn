// ============================================================
// 练习 E21: 前端事件
// 目标: 用 listen / once / unlisten 收发事件，payload 类型化
// 知识点: listen 多次接收 / once 一次性 / unlisten 取消 / 类型化 payload
// ============================================================

import { invoke } from "@tauri-apps/api/core";
import { listen, once } from "@tauri-apps/api/event";

// 与 Rust 端 EventPayload 对应（payload 类型化）
interface EventPayload {
  id: number;
  message: string;
  ts: number;
}

const msgInput = document.querySelector<HTMLInputElement>("#msg-input");
const customBtn = document.querySelector<HTMLButtonElement>("#custom-btn");
const onceBtn = document.querySelector<HTMLButtonElement>("#once-btn");
const unlistenBtn = document.querySelector<HTMLButtonElement>("#unlisten-btn");
const customList = document.querySelector<HTMLUListElement>("#custom-list");
const onceList = document.querySelector<HTMLUListElement>("#once-list");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

// 保存 listen 返回的取消函数，供「停止监听」按钮调用
let unlisten: (() => void) | null = null;

// 普通监听：可多次收到事件；listen 返回 Promise<UnlistenFn>
listen<EventPayload>("custom-event", (e) => {
  const p = e.payload;
  const li = document.createElement("li");
  li.className = "ok";
  li.innerHTML = `<span class="badge">#${p.id}</span>${p.message} <span class="detail">ts=${p.ts}</span>`;
  customList!.appendChild(li);
}).then((fn) => {
  unlisten = fn;
});

// 一次性监听：只触发一次，之后自动移除
once<EventPayload>("one-time-event", (e) => {
  const p = e.payload;
  const li = document.createElement("li");
  li.className = "ok";
  li.innerHTML = `<span class="badge">#${p.id}</span>${p.message} <span class="detail">ts=${p.ts}</span>`;
  onceList!.appendChild(li);
});

// 构造 payload 并发送
function buildPayload(): EventPayload {
  return {
    id: Math.floor(Math.random() * 1000),
    message: msgInput!.value.trim() || "默认消息",
    ts: Date.now(),
  };
}

customBtn!.addEventListener("click", async () => {
  try {
    await invoke("emit_custom_event", { payload: buildPayload() });
    resultEl!.textContent = "已发送普通事件（可点击多次，每次都会收到）";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

onceBtn!.addEventListener("click", async () => {
  try {
    await invoke("emit_once_event", { payload: buildPayload() });
    resultEl!.textContent = "已发送一次性事件（点击多次也只收到一次）";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

unlistenBtn!.addEventListener("click", () => {
  if (unlisten) {
    unlisten();
    unlisten = null;
    resultEl!.textContent = "已停止监听 custom-event";
    resultEl!.className = "status";
  } else {
    resultEl!.textContent = "当前没有活动的监听";
    resultEl!.className = "status";
  }
});