// ============================================================
// 练习 E19: 后台任务
// 目标: 启动后台任务，用事件监听接收完成结果
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 调用命令；listen 监听后端事件）
// import { invoke } from "@tauri-apps/api/core";
// import { listen } from "@tauri-apps/api/event";

const asyncBtn = document.querySelector<HTMLButtonElement>("#async-btn");
const blockingBtn = document.querySelector<HTMLButtonElement>("#blocking-btn");
const taskList = document.querySelector<HTMLUListElement>("#task-list");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

// === 步骤 1: 监听异步任务完成事件 ————————————————————————————————
// TODO: 用 listen 监听 "task-done"，把 e.payload 追加到 #task-list
// 提示: listen<string>("task-done", (e) => {
//         const li = document.createElement("li");
//         li.className = "ok";
//         li.innerHTML = `<span class="badge">异步</span>${e.payload}`;
//         taskList!.appendChild(li);
//       });
//       listen 来自 "@tauri-apps/api/event"

// === 步骤 2: 监听阻塞任务完成事件 ————————————————————————————————
// TODO: 同上，监听 "blocking-done"（badge 文案改为「阻塞」）
// 提示: listen<string>("blocking-done", (e) => { ... })

// 占位：先渲染一条提示，完成 listen 填空后删除
const placeholderLi = document.createElement("li");
placeholderLi.className = "warn";
placeholderLi.innerHTML = `<span class="badge">?</span>等待事件监听代码完成`;
taskList!.appendChild(placeholderLi);

asyncBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 3: 启动异步任务 ————————————————————————————————————
    // TODO: await invoke("start_async_task", { seconds: 3 });
    // 占位：完成填空后替换为真实调用
    resultEl!.textContent = "已启动异步任务（3 秒后完成，期间 UI 不卡顿）";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

blockingBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 4: 启动阻塞任务 ————————————————————————————————————
    // TODO: await invoke("start_blocking_task", { n: 100 });
    // 占位：完成填空后替换为真实调用
    resultEl!.textContent = "已启动阻塞任务（计算 1..=100 的平方和）";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});