// ============================================================
// 练习 E10: 依赖注入
// 目标: 调用注入多个依赖的命令，观察注入信息
// 知识点: invoke 传参 / 返回值类型化
// ============================================================

import { invoke } from "@tauri-apps/api/core";

// 与 Rust 端 InspectInfo 对应的类型（字段自动转 camelCase）
interface InspectInfo {
  windowTitle: string;
  windowLabel: string;
  windowCount: number;
  counter: number;
  appName: string;
}

const inspectBtn = document.querySelector<HTMLButtonElement>("#inspect-btn");
const incrementBtn = document.querySelector<HTMLButtonElement>("#increment-btn");
const resultEl = document.querySelector<HTMLDivElement>("#result");
const counterEl = document.querySelector<HTMLSpanElement>("#counter");

// 把注入信息渲染为 kv 表格
function renderInfo(info: InspectInfo) {
  const rows: Array<[string, string]> = [
    ["窗口标题", info.windowTitle],
    ["窗口标签", info.windowLabel],
    ["窗口数量", String(info.windowCount)],
    ["计数器", String(info.counter)],
    ["应用名", info.appName],
  ];
  resultEl!.innerHTML = `<table class="kv">${rows
    .map(([k, v]) => `<tr><th>${k}</th><td>${v}</td></tr>`)
    .join("")}</table>`;
}

inspectBtn!.addEventListener("click", async () => {
  try {
    const info = await invoke<InspectInfo>("inspect");
    renderInfo(info);
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

incrementBtn!.addEventListener("click", async () => {
  try {
    const n = await invoke<number>("increment");
    counterEl!.textContent = String(n);
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});