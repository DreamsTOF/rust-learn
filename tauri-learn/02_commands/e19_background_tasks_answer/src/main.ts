// ============================================================
// 练习 E19: 后台任务
// 目标: 启动后台任务，用事件监听接收完成结果
// 知识点: invoke 启动任务 / listen 事件 / 异步与阻塞任务对比
// ============================================================

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const asyncBtn = document.querySelector<HTMLButtonElement>("#async-btn");
const blockingBtn = document.querySelector<HTMLButtonElement>("#blocking-btn");
const taskList = document.querySelector<HTMLUListElement>("#task-list");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

// 监听后端回传的完成事件，把消息追加到列表
listen<string>("task-done", (e) => {
  const li = document.createElement("li");
  li.className = "ok";
  li.innerHTML = `<span class="badge">异步</span>${e.payload}`;
  taskList!.appendChild(li);
});

listen<string>("blocking-done", (e) => {
  const li = document.createElement("li");
  li.className = "ok";
  li.innerHTML = `<span class="badge">阻塞</span>${e.payload}`;
  taskList!.appendChild(li);
});

asyncBtn!.addEventListener("click", async () => {
  try {
    await invoke("start_async_task", { seconds: 3 });
    resultEl!.textContent = "已启动异步任务（3 秒后完成，期间 UI 不卡顿）";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

blockingBtn!.addEventListener("click", async () => {
  try {
    await invoke("start_blocking_task", { n: 100 });
    resultEl!.textContent = "已启动阻塞任务（计算 1..=100 的平方和）";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});