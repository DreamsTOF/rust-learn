// ============================================================
// 练习 E26: 无边框窗口
// 目标: 用 data-tauri-drag-region 实现自定义标题栏拖拽与按钮控制
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（getCurrentWindow 获取当前窗口）
// import { getCurrentWindow } from "@tauri-apps/api/window";

// === 步骤 1: 最小化按钮 ————————————————————————————————————
// TODO: 获取当前窗口，给 #min-btn 绑定 click 事件调用 win.minimize()
// 提示: const win = getCurrentWindow();
//       document.querySelector<HTMLButtonElement>("#min-btn")!
//         .addEventListener("click", () => win.minimize());

// === 步骤 2: 关闭按钮 ————————————————————————————————————
// TODO: 给 #close-btn 绑定 click 事件调用 win.close()
// 提示: document.querySelector<HTMLButtonElement>("#close-btn")!
//         .addEventListener("click", () => win.close());