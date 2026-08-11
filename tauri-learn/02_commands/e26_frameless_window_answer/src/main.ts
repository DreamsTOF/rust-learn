// ============================================================
// 练习 E26: 无边框窗口
// 目标: 用 data-tauri-drag-region 实现自定义标题栏拖拽与按钮控制
// 知识点: decorations / 拖拽区域 / getCurrentWindow 最小化与关闭
// ============================================================

import { getCurrentWindow } from "@tauri-apps/api/window";

const win = getCurrentWindow();

document.querySelector<HTMLButtonElement>("#min-btn")!.addEventListener("click", () => {
  win.minimize();
});

document.querySelector<HTMLButtonElement>("#close-btn")!.addEventListener("click", () => {
  win.close();
});