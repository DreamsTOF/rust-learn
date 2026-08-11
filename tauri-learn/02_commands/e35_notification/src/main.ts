// ============================================================
// 练习 E35: 通知
// 目标: 使用 @tauri-apps/plugin-notification 请求权限、发送系统通知并监听点击
// 知识点: isPermissionGranted / requestPermission / sendNotification / onAction
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（通知相关 API）
// import {
//   isPermissionGranted,
//   requestPermission,
//   sendNotification,
//   onAction,
// } from "@tauri-apps/plugin-notification";

const badgeEl = document.querySelector<HTMLSpanElement>("#permission-badge");
const requestBtn = document.querySelector<HTMLButtonElement>("#request-btn");
const sendBtn = document.querySelector<HTMLButtonElement>("#send-btn");
const titleInput = document.querySelector<HTMLInputElement>("#title");
const bodyInput = document.querySelector<HTMLInputElement>("#body");
const logEl = document.querySelector<HTMLPreElement>("#log");

logEl!.textContent = "";

// 操作日志：把消息追加到日志区（练习版已实现，填空时直接使用）
function log(msg: string): void {
  const time = new Date().toLocaleTimeString();
  logEl!.textContent += `[${time}] ${msg}\n`;
}

// === 步骤 1: 权限状态 ————————————————————————————————————
// TODO: 声明权限状态变量：
//   let granted = false;
// 提示: isPermissionGranted() / requestPermission() 会更新该值
// 当前为占位（完成填空后替换为上面一行）
let granted = false;

// 根据权限状态更新徽标与按钮显隐
function updatePermissionUI(): void {
  // === 步骤 5: 更新权限状态徽标 ——————————————————————————————
  // TODO: 按 granted 更新界面：
  //   badgeEl!.textContent = granted ? "权限状态: 已授权" : "权限状态: 未授权";
  //   badgeEl!.className = granted ? "status ok" : "status err";
  //   requestBtn!.style.display = granted ? "none" : "";
  //   sendBtn!.style.display = granted ? "" : "none";
  // 提示: 未授权时显示「请求权限」按钮，授权后显示「发送通知」按钮
  // 当前为空函数（完成填空后实现）；下方 void 引用仅为避免未使用变量报错，
  // 完成填空后删除
  void badgeEl;
  void requestBtn;
  void sendBtn;
}

// === 步骤 2: 页面加载时检查权限 ————————————————————————————
// TODO: 检查通知权限并刷新界面：
//   isPermissionGranted()
//     .then((g) => {
//       granted = g;
//       updatePermissionUI();
//     })
//     .catch((e) => console.error(e));
// 提示: isPermissionGranted() 返回 Promise<boolean>
// 当前为占位调用（完成填空后替换）
updatePermissionUI();

// 请求权限（首次使用需用户授权）
requestBtn!.addEventListener("click", async () => {
  // === 步骤 3: 请求权限 ————————————————————————————————————
  // TODO: 请求通知权限并刷新界面：
  //   const permission = await requestPermission();
  //   granted = permission === "granted";
  //   updatePermissionUI();
  // 提示: requestPermission() 返回 'granted' | 'denied' | 'default'
  // 当前为占位（完成填空后替换）
  granted = false;
  updatePermissionUI();
});

// 发送通知（Windows 上显示在系统通知中心）
sendBtn!.addEventListener("click", () => {
  const title = titleInput!.value.trim() || "Tauri 通知";
  const body = bodyInput!.value.trim() || "这是一条来自 Tauri 的通知";
  // === 步骤 4: 发送通知 ————————————————————————————————————
  // TODO: 发送通知：
  //   sendNotification({ title, body });
  // 提示: sendNotification 接收 { title, body } 对象
  // 当前为占位（完成填空后替换）
  log(`（占位）将发送通知: ${title} - ${body}`);
});

// === 步骤 6: 监听通知点击 —————————————————————————————————
// TODO: 注册点击监听，点击通知后在前端显示「通知被点击」：
//   onAction((notification) => {
//     log(`通知被点击: ${notification.title ?? "(无标题)"}`);
//   });
// 提示: 回调收到通知对象（含 title/body）；旧版回调带 { type: 'clicked' } 字段
// 当前为占位日志（完成填空后替换）
log("（占位）onAction 监听尚未注册");