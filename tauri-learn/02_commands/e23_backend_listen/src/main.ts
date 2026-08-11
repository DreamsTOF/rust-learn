// ============================================================
// 练习 E23: 后端监听
// 目标: 发 ping 给后端，接收后端转发的 pong 回应
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 调用命令；listen 监听事件）
// import { invoke } from "@tauri-apps/api/core";
// import { listen } from "@tauri-apps/api/event";

const msgInput = document.querySelector<HTMLInputElement>("#msg-input");
const pingABtn = document.querySelector<HTMLButtonElement>("#ping-a-btn");
const pingBBtn = document.querySelector<HTMLButtonElement>("#ping-b-btn");
const pongList = document.querySelector<HTMLUListElement>("#pong-list");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

// === 步骤 1: 监听后端回应 ————————————————————————————————————
// TODO: 用 listen 监听 "pong"，把 e.payload 追加到 #pong-list（badge 文案「回应」）
// 提示: listen<string>("pong", (e) => {
//         const li = document.createElement("li");
//         li.className = "ok";
//         li.innerHTML = `<span class="badge">回应</span>${e.payload}`;
//         pongList!.appendChild(li);
//       });
//       listen 来自 "@tauri-apps/api/event"

// 占位：先渲染一条提示，完成 listen 填空后删除
pongList!.innerHTML = `<li class="warn"><span class="badge">?</span>等待监听代码完成</li>`;

pingABtn!.addEventListener("click", async () => {
  try {
    // === 步骤 2: 发 ping-a ————————————————————————————————————
    // TODO: const msg = msgInput!.value.trim() || "hello backend";
    //       await invoke("emit_ping", { kind: "a", msg });
    // 占位：完成填空后替换为真实调用
    resultEl!.textContent = "已发送 ping-a";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

pingBBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 3: 发 ping-b ————————————————————————————————————
    // TODO: 同上，kind 改为 "b"
    // 提示: await invoke("emit_ping", { kind: "b", msg });
    // 占位：完成填空后替换为真实调用
    resultEl!.textContent = "已发送 ping-b";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});