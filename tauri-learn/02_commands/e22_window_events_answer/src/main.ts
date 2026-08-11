// ============================================================
// 练习 E22: 窗口级事件
// 目标: 主窗口与 chat 窗口按 label 渲染不同 UI，体验定向与广播
// 知识点: emit_to 定向 / 全局 broadcast / 窗口级与全局监听差异
// ============================================================

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";

const win = getCurrentWindow();
const isChat = win.label === "chat";

// 主窗口 UI 与聊天窗 UI 共用同一页面，按窗口 label 显示对应区域
const mainUi = document.querySelector<HTMLDivElement>("#main-ui");
const chatUi = document.querySelector<HTMLDivElement>("#chat-ui");
if (isChat) {
  mainUi!.style.display = "none";
  chatUi!.style.display = "block";
} else {
  mainUi!.style.display = "block";
  chatUi!.style.display = "none";
}

// 本窗口的消息列表（主窗与 chat 窗各自展示）
const msgList = document.querySelector<HTMLUListElement>("#msg-list");

// 追加一条消息，标注事件来源
function appendLog(tag: string, payload: string) {
  const li = document.createElement("li");
  li.className = "ok";
  li.innerHTML = `<span class="badge">${tag}</span>${payload}`;
  msgList!.appendChild(li);
}

// 窗口级监听：只有定向发给本窗口的 targeted-event 才收到
win.listen<string>("targeted-event", (e) => {
  appendLog("定向", e.payload);
});

// 全局监听：所有窗口都会收到 broadcast-event
listen<string>("broadcast-event", (e) => {
  appendLog("广播", e.payload);
});

// —— 主窗口专属操作（chat 窗口隐藏这些控件）——
const openChatBtn = document.querySelector<HTMLButtonElement>("#open-chat-btn");
const toMainBtn = document.querySelector<HTMLButtonElement>("#to-main-btn");
const toChatBtn = document.querySelector<HTMLButtonElement>("#to-chat-btn");
const toAllBtn = document.querySelector<HTMLButtonElement>("#to-all-btn");
const msgInput = document.querySelector<HTMLInputElement>("#msg-input");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

const msg = () => msgInput!.value.trim() || "hello";

openChatBtn?.addEventListener("click", async () => {
  try {
    await invoke("open_chat_window");
    resultEl!.textContent = "已打开聊天窗（已存在则聚焦）";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

toMainBtn?.addEventListener("click", async () => {
  try {
    await invoke("send_to_main", { msg: msg() });
    resultEl!.textContent = `已定向发给主窗: ${msg()}`;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

toChatBtn?.addEventListener("click", async () => {
  try {
    await invoke("send_to_chat", { msg: msg() });
    resultEl!.textContent = `已定向发给聊天窗: ${msg()}（主窗收不到）`;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

toAllBtn?.addEventListener("click", async () => {
  try {
    await invoke("send_to_all", { msg: msg() });
    resultEl!.textContent = `已广播: ${msg()}（两个窗口都收到）`;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});