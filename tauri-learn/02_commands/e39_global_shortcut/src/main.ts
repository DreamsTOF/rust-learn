// ============================================================
// 练习 E39: 全局快捷键
// 目标: 输入组合键，注册/注销/查询全局快捷键，并接收按键事件日志
// 知识点: invoke 调命令 / listen 接收后端广播 / 失焦仍生效 / 冲突报错
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";
// TODO: 完成填空后取消注释（listen 用于监听后端广播事件）
// import { listen } from "@tauri-apps/api/event";

const comboInput = document.querySelector<HTMLInputElement>("#combo");
const registerBtn = document.querySelector<HTMLButtonElement>("#register-btn");
const unregisterBtn = document.querySelector<HTMLButtonElement>("#unregister-btn");
const checkBtn = document.querySelector<HTMLButtonElement>("#check-btn");
const statusEl = document.querySelector<HTMLParagraphElement>("#status");
const logEl = document.querySelector<HTMLDivElement>("#log");

// === 步骤 5: 监听快捷键按下事件 ——————————————————————————————————
// TODO: 用 listen 接收后端广播并追加到日志：
//   listen<string>("shortcut-pressed", (event) => {
//     appendLog(`按下: ${event.payload}`);
//   });
// 提示: import { listen } from "@tauri-apps/api/event"；
//       listen 返回 Promise，也可用 .then 注册回调
// 当前为占位（完成填空后替换为上面的 listen 调用）
function appendLog(text: string): void {
  const line = document.createElement("div");
  line.textContent = `[${new Date().toLocaleTimeString()}] ${text}`;
  logEl!.prepend(line);
}
appendLog("监听逻辑待补全（完成步骤 5 后此处会显示按键日志）");

// 注册
registerBtn!.addEventListener("click", async () => {
  const combo = comboInput!.value.trim();
  try {
    // === 步骤 2: 调用 register_shortcut ————————————————————————————
    // TODO: const msg = await invoke<string>("register_shortcut", { combo });
    //       statusEl!.textContent = msg;
    //       statusEl!.className = "status ok";
    // 提示: invoke 参数名与 Rust 参数名一致（camelCase）
    // 当前为占位（完成填空后替换）
    statusEl!.textContent = `注册逻辑待补全（${combo}）`;
    statusEl!.className = "status";
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
    // === 步骤 3: 调用 unregister_shortcut ——————————————————————————
    // TODO: const msg = await invoke<string>("unregister_shortcut", { combo });
    //       statusEl!.textContent = msg;
    //       statusEl!.className = "status ok";
    // 提示: 与步骤 2 同构，命令名换成 unregister_shortcut
    // 当前为占位（完成填空后替换）
    statusEl!.textContent = `注销逻辑待补全（${combo}）`;
    statusEl!.className = "status";
  } catch (e) {
    statusEl!.textContent = `注销失败: ${e}`;
    statusEl!.className = "status err";
  }
});

// 查询状态
checkBtn!.addEventListener("click", async () => {
  const combo = comboInput!.value.trim();
  try {
    // === 步骤 4: 调用 is_shortcut_registered ————————————————————————
    // TODO: const registered = await invoke<boolean>("is_shortcut_registered", { combo });
    //       statusEl!.textContent = `${combo} 当前状态: ${registered ? "已注册" : "未注册"}`;
    //       statusEl!.className = "status ok";
    // 提示: 命令返回布尔值，invoke 泛型用 boolean
    // 当前为占位（完成填空后替换）
    statusEl!.textContent = `查询逻辑待补全（${combo}）`;
    statusEl!.className = "status";
  } catch (e) {
    statusEl!.textContent = `查询失败: ${e}`;
    statusEl!.className = "status err";
  }
});