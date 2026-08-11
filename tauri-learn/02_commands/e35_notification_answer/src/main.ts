// ============================================================
// 练习 E35: 通知
// 目标: 使用 @tauri-apps/plugin-notification 请求权限、发送系统通知并监听点击
// 知识点: isPermissionGranted / requestPermission / sendNotification / onAction
// ============================================================

import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
  onAction,
} from "@tauri-apps/plugin-notification";

const badgeEl = document.querySelector<HTMLSpanElement>("#permission-badge");
const requestBtn = document.querySelector<HTMLButtonElement>("#request-btn");
const sendBtn = document.querySelector<HTMLButtonElement>("#send-btn");
const titleInput = document.querySelector<HTMLInputElement>("#title");
const bodyInput = document.querySelector<HTMLInputElement>("#body");
const logEl = document.querySelector<HTMLPreElement>("#log");

logEl!.textContent = "";

// 操作日志：把消息追加到日志区
function log(msg: string): void {
  const time = new Date().toLocaleTimeString();
  logEl!.textContent += `[${time}] ${msg}\n`;
}

// 当前是否已获得通知权限
let granted = false;

// 根据权限状态更新徽标与按钮显隐
function updatePermissionUI(): void {
  badgeEl!.textContent = granted ? "权限状态: 已授权" : "权限状态: 未授权";
  badgeEl!.className = granted ? "status ok" : "status err";
  requestBtn!.style.display = granted ? "none" : "";
  sendBtn!.style.display = granted ? "" : "none";
}

// === 页面加载: 检查权限 ——————————————————————————————————
isPermissionGranted()
  .then((g) => {
    granted = g;
    updatePermissionUI();
    log(`页面加载，权限已${granted ? "授权" : "未授权"}`);
  })
  .catch((e) => {
    log(`权限检查失败: ${e}`);
    updatePermissionUI();
  });

// 请求权限（首次使用需用户授权）
requestBtn!.addEventListener("click", async () => {
  try {
    const permission = await requestPermission();
    granted = permission === "granted";
    updatePermissionUI();
    log(`请求权限结果: ${permission}`);
  } catch (e) {
    log(`请求权限失败: ${e}`);
  }
});

// 发送通知（Windows 上显示在系统通知中心）
sendBtn!.addEventListener("click", () => {
  const title = titleInput!.value.trim() || "Tauri 通知";
  const body = bodyInput!.value.trim() || "这是一条来自 Tauri 的通知";
  try {
    sendNotification({ title, body });
    log(`已发送通知: ${title} - ${body}`);
  } catch (e) {
    log(`发送失败: ${e}`);
  }
});

// === 监听通知点击 ————————————————————————————————————————
// 回调收到通知对象（含 title/body/id 等）；点击后在前端显示提示
onAction((notification) => {
  log(`通知被点击: ${notification.title ?? "(无标题)"}`);
}).catch((e) => {
  log(`onAction 注册失败: ${e}`);
});