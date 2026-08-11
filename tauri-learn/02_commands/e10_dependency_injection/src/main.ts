// ============================================================
// 练习 E10: 依赖注入
// 目标: 调用注入多个依赖的命令，观察注入信息
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

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
    // === 步骤 5: 调用 inspect 命令 ————————————————————————————————————
    // TODO: const info = await invoke<InspectInfo>("inspect");
    // 提示: inspect 不接收参数，返回注入信息对象
    // 占位：完成填空后替换为真实调用结果
    const info: InspectInfo = {
      windowTitle: "",
      windowLabel: "",
      windowCount: 0,
      counter: 0,
      appName: "",
    };
    renderInfo(info);
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

incrementBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 6: 调用 increment 命令 ————————————————————————————————————
    // TODO: const n = await invoke<number>("increment");
    // 占位：完成填空后替换为真实调用结果
    const n: number = 0;
    counterEl!.textContent = String(n);
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});