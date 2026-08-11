// ============================================================
// 练习 E24: 创建与操作窗口
// 目标: 主窗口操控 ops 子窗口，两个窗口共用同一页面
// 知识点: invoke 命令 / getCurrentWindow().label 区分窗口
// ============================================================

import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

const win = getCurrentWindow();
const isOps = win.label === "ops";

// 主窗口与 ops 窗口共用同一页面，按 label 显示对应区域
const mainUi = document.querySelector<HTMLDivElement>("#main-ui");
const opsUi = document.querySelector<HTMLDivElement>("#ops-ui");
if (isOps) {
  mainUi!.style.display = "none";
  opsUi!.style.display = "block";
} else {
  mainUi!.style.display = "block";
  opsUi!.style.display = "none";
}

const resultEl = document.querySelector<HTMLParagraphElement>("#result");

// 统一封装：执行 invoke 并展示成功/失败信息
async function run(action: () => Promise<unknown>, okMsg: string) {
  try {
    await action();
    resultEl!.textContent = okMsg;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
}

document
  .querySelector<HTMLButtonElement>("#spawn-btn")
  ?.addEventListener("click", () =>
    run(() => invoke("spawn_ops_window"), "已打开 ops 窗口（已存在则聚焦）")
  );

document
  .querySelector<HTMLButtonElement>("#move-btn")
  ?.addEventListener("click", () =>
    run(() => invoke("move_window", { x: 150, y: 120 }), "已将 ops 窗口移动到 (150, 120)")
  );

document
  .querySelector<HTMLButtonElement>("#resize-btn")
  ?.addEventListener("click", () =>
    run(() => invoke("resize_window", { w: 700, h: 500 }), "已将 ops 窗口放大到 700x500")
  );

document
  .querySelector<HTMLButtonElement>("#center-btn")
  ?.addEventListener("click", () =>
    run(() => invoke("center_window"), "已将 ops 窗口居中")
  );

document
  .querySelector<HTMLButtonElement>("#toggle-btn")
  ?.addEventListener("click", async () => {
    try {
      const visible = await invoke<boolean>("toggle_window");
      resultEl!.textContent = visible ? "ops 窗口已显示" : "ops 窗口已隐藏";
      resultEl!.className = "status ok";
    } catch (e) {
      resultEl!.textContent = `调用失败: ${e}`;
      resultEl!.className = "status err";
    }
  });

document
  .querySelector<HTMLButtonElement>("#close-btn")
  ?.addEventListener("click", () =>
    run(() => invoke("close_ops_window"), "已关闭 ops 窗口")
  );