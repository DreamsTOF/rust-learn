// ============================================================
// 练习 E21: 前端事件
// 目标: 用 listen / once / unlisten 收发事件，payload 类型化
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 调用命令；listen/once 监听事件）
// import { invoke } from "@tauri-apps/api/core";
// import { listen, once } from "@tauri-apps/api/event";

// === 步骤 1: 定义 payload 类型 ————————————————————————————————————
// TODO: 定义 interface EventPayload { id: number; message: string; ts: number; }
//       与 Rust 端 EventPayload 字段一一对应（payload 类型化）

const msgInput = document.querySelector<HTMLInputElement>("#msg-input");
const customBtn = document.querySelector<HTMLButtonElement>("#custom-btn");
const onceBtn = document.querySelector<HTMLButtonElement>("#once-btn");
const unlistenBtn = document.querySelector<HTMLButtonElement>("#unlisten-btn");
const customList = document.querySelector<HTMLUListElement>("#custom-list");
const onceList = document.querySelector<HTMLUListElement>("#once-list");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

// 保存 listen 返回的取消函数，供「停止监听」按钮调用
let unlisten: (() => void) | null = null;

// === 步骤 2: 普通监听 ————————————————————————————————————
// TODO: 用 listen 监听 "custom-event"，把 e.payload 追加到 #custom-list，
//       并把返回的取消函数存入 unlisten
// 提示: listen<EventPayload>("custom-event", (e) => {
//         const p = e.payload;
//         const li = document.createElement("li");
//         li.className = "ok";
//         li.innerHTML = `<span class="badge">#${p.id}</span>${p.message} <span class="detail">ts=${p.ts}</span>`;
//         customList!.appendChild(li);
//       }).then((fn) => { unlisten = fn; });
//       listen / once 来自 "@tauri-apps/api/event"，返回 Promise<UnlistenFn>

// === 步骤 3: 一次性监听 ————————————————————————————————————
// TODO: 用 once 监听 "one-time-event"，只触发一次（追加到 #once-list）
// 提示: once<EventPayload>("one-time-event", (e) => { ...同上渲染到 onceList... });

// 占位：先渲染一条提示，完成 listen/once 填空后删除
customList!.innerHTML = `<li class="warn"><span class="badge">?</span>等待 listen 完成</li>`;
onceList!.innerHTML = `<li class="warn"><span class="badge">?</span>等待 once 完成</li>`;

customBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 4: 发送普通事件 ————————————————————————————————————
    // TODO: 构造 payload 并调用 emit_custom_event
    // 提示: const payload: EventPayload = {
    //         id: Math.floor(Math.random() * 1000),
    //         message: msgInput!.value.trim() || "默认消息",
    //         ts: Date.now(),
    //       };
    //       await invoke("emit_custom_event", { payload });
    // 占位：完成填空后替换为真实调用
    resultEl!.textContent = "已发送普通事件（可点击多次，每次都会收到）";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

onceBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 5: 发送一次性事件 ————————————————————————————————————
    // TODO: 同上构造 payload，调用 emit_once_event
    // 提示: await invoke("emit_once_event", { payload });
    // 占位：完成填空后替换为真实调用
    resultEl!.textContent = "已发送一次性事件（点击多次也只收到一次）";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

unlistenBtn!.addEventListener("click", () => {
  // === 步骤 6: 停止监听 ————————————————————————————————————
  // TODO: 调用 unlisten() 停止接收 custom-event 并置空
  // 提示: if (unlisten) { unlisten(); unlisten = null; }
  // 占位：当前保持可编译（无监听时按钮无效果）
  resultEl!.textContent = unlisten ? "已停止监听 custom-event" : "当前没有活动的监听";
  resultEl!.className = "status";
});