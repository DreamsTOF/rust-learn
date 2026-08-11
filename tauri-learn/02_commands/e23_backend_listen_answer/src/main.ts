// ============================================================
// 练习 E23: 后端监听
// 目标: 发 ping 给后端，接收后端转发的 pong 回应
// 知识点: invoke 发事件 / listen 收回应 / 事件链路
// ============================================================

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const msgInput = document.querySelector<HTMLInputElement>("#msg-input");
const pingABtn = document.querySelector<HTMLButtonElement>("#ping-a-btn");
const pingBBtn = document.querySelector<HTMLButtonElement>("#ping-b-btn");
const pongList = document.querySelector<HTMLUListElement>("#pong-list");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

const msg = () => msgInput!.value.trim() || "hello backend";

// 监听后端转发的 pong 回应，标注来源 a/b
listen<string>("pong", (e) => {
  const li = document.createElement("li");
  li.className = "ok";
  li.innerHTML = `<span class="badge">回应</span>${e.payload}`;
  pongList!.appendChild(li);
});

pingABtn!.addEventListener("click", async () => {
  try {
    await invoke("emit_ping", { kind: "a", msg: msg() });
    resultEl!.textContent = `已发送 ping-a: ${msg()}`;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

pingBBtn!.addEventListener("click", async () => {
  try {
    await invoke("emit_ping", { kind: "b", msg: msg() });
    resultEl!.textContent = `已发送 ping-b: ${msg()}`;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});