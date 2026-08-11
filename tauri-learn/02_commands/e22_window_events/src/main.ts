// ============================================================
// 练习 E22: 窗口级事件
// 目标: 主窗口与 chat 窗口按 label 渲染不同 UI，体验定向与广播
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 调用命令；listen 监听事件；getCurrentWindow 获取窗口）
// import { invoke } from "@tauri-apps/api/core";
// import { listen } from "@tauri-apps/api/event";
// import { getCurrentWindow } from "@tauri-apps/api/window";

// === 步骤 1: 按窗口 label 切换 UI ————————————————————————————————————
// TODO: 获取当前窗口并判断是否为 chat 窗口：
//   const win = getCurrentWindow();
//   const isChat = win.label === "chat";
//   getCurrentWindow 来自 "@tauri-apps/api/window"
const isChat: boolean = false;

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

const msgList = document.querySelector<HTMLUListElement>("#msg-list");

// === 步骤 2: 窗口级监听 targeted-event ————————————————————————————————————
// TODO: 用 getCurrentWindow().listen 监听 "targeted-event"，
//       只有定向发给本窗口的事件才收到，追加到 #msg-list（标注「定向」）
// 提示: win.listen<string>("targeted-event", (e) => {
//         const li = document.createElement("li");
//         li.className = "ok";
//         li.innerHTML = `<span class="badge">定向</span>${e.payload}`;
//         msgList!.appendChild(li);
//       });

// === 步骤 3: 全局监听 broadcast-event ————————————————————————————————————
// TODO: 用 listen 监听 "broadcast-event"（全局事件，两个窗口都收到，标注「广播」）
// 提示: listen<string>("broadcast-event", (e) => { ...同上，badge 文案「广播」... });

// 占位：先渲染一条提示，完成监听填空后删除
msgList!.innerHTML = `<li class="warn"><span class="badge">?</span>等待监听代码完成</li>`;

// —— 主窗口专属操作（chat 窗口隐藏这些控件）——
const openChatBtn = document.querySelector<HTMLButtonElement>("#open-chat-btn");
const toMainBtn = document.querySelector<HTMLButtonElement>("#to-main-btn");
const toChatBtn = document.querySelector<HTMLButtonElement>("#to-chat-btn");
const toAllBtn = document.querySelector<HTMLButtonElement>("#to-all-btn");
const msgInput = document.querySelector<HTMLInputElement>("#msg-input");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

openChatBtn?.addEventListener("click", async () => {
  try {
    // === 步骤 4: 打开聊天窗 ————————————————————————————————————
    // TODO: await invoke("open_chat_window");
    // 占位：完成填空后替换为真实调用
    resultEl!.textContent = "已打开聊天窗（已存在则聚焦）";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

toMainBtn?.addEventListener("click", async () => {
  try {
    // === 步骤 5: 定向发给主窗 ————————————————————————————————————
    // TODO: const msg = msgInput!.value.trim() || "hello";
    //       await invoke("send_to_main", { msg });
    // 占位：完成填空后替换为真实调用
    resultEl!.textContent = "已定向发给主窗";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

toChatBtn?.addEventListener("click", async () => {
  try {
    // === 步骤 6: 定向发给聊天窗 ————————————————————————————————————
    // TODO: 同上，调用 send_to_chat
    // 提示: await invoke("send_to_chat", { msg });
    // 占位：完成填空后替换为真实调用
    resultEl!.textContent = "已定向发给聊天窗（主窗收不到）";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

toAllBtn?.addEventListener("click", async () => {
  try {
    // === 步骤 7: 广播给全部 ————————————————————————————————————
    // TODO: 同上，调用 send_to_all
    // 提示: await invoke("send_to_all", { msg });
    // 占位：完成填空后替换为真实调用
    resultEl!.textContent = "已广播（两个窗口都收到）";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});