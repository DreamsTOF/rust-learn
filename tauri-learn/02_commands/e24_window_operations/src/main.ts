// ============================================================
// 练习 E24: 创建与操作窗口
// 目标: 主窗口操控 ops 子窗口，两个窗口共用同一页面
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 调用后端命令；getCurrentWindow 获取当前窗口）
// import { invoke } from "@tauri-apps/api/core";
// import { getCurrentWindow } from "@tauri-apps/api/window";

// === 步骤 1: 按窗口 label 切换 UI ————————————————————————————————————
// TODO: 获取当前窗口并判断是否为 ops 窗口：
//   const win = getCurrentWindow();
//   const isOps = win.label === "ops";
//   getCurrentWindow 来自 "@tauri-apps/api/window"
const isOps: boolean = false;

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

document
  .querySelector<HTMLButtonElement>("#spawn-btn")
  ?.addEventListener("click", async () => {
    try {
      // === 步骤 2: 打开 ops 窗口 ————————————————————————————————————
      // TODO: await invoke("spawn_ops_window");
      // 占位：完成填空后替换为真实调用
      resultEl!.textContent = "已打开 ops 窗口（已存在则聚焦）";
      resultEl!.className = "status ok";
    } catch (e) {
      resultEl!.textContent = `调用失败: ${e}`;
      resultEl!.className = "status err";
    }
  });

document
  .querySelector<HTMLButtonElement>("#move-btn")
  ?.addEventListener("click", async () => {
    try {
      // === 步骤 3: 移动窗口 ————————————————————————————————————
      // TODO: await invoke("move_window", { x: 150, y: 120 });
      // 提示: invoke 参数名 camelCase，与 Rust 参数名对应
      // 占位：完成填空后替换为真实调用
      resultEl!.textContent = "已将 ops 窗口移动到 (150, 120)";
      resultEl!.className = "status ok";
    } catch (e) {
      resultEl!.textContent = `调用失败: ${e}`;
      resultEl!.className = "status err";
    }
  });

document
  .querySelector<HTMLButtonElement>("#resize-btn")
  ?.addEventListener("click", async () => {
    try {
      // === 步骤 4: 缩放窗口 ————————————————————————————————————
      // TODO: await invoke("resize_window", { w: 700, h: 500 });
      // 占位：完成填空后替换为真实调用
      resultEl!.textContent = "已将 ops 窗口放大到 700x500";
      resultEl!.className = "status ok";
    } catch (e) {
      resultEl!.textContent = `调用失败: ${e}`;
      resultEl!.className = "status err";
    }
  });

document
  .querySelector<HTMLButtonElement>("#center-btn")
  ?.addEventListener("click", async () => {
    try {
      // === 步骤 5: 居中 ————————————————————————————————————
      // TODO: 同上，调用 center_window（无参数）
      // 提示: await invoke("center_window");
      // 占位：完成填空后替换为真实调用
      resultEl!.textContent = "已将 ops 窗口居中";
      resultEl!.className = "status ok";
    } catch (e) {
      resultEl!.textContent = `调用失败: ${e}`;
      resultEl!.className = "status err";
    }
  });

document
  .querySelector<HTMLButtonElement>("#toggle-btn")
  ?.addEventListener("click", async () => {
    try {
      // === 步骤 6: 显隐切换 ————————————————————————————————————
      // TODO: 同上，调用 toggle_window，用返回的 boolean 显示新状态
      // 提示: const visible = await invoke<boolean>("toggle_window");
      //       resultEl!.textContent = visible ? "ops 窗口已显示" : "ops 窗口已隐藏";
      // 占位：完成填空后替换为真实调用
      resultEl!.textContent = "已切换可见性";
      resultEl!.className = "status ok";
    } catch (e) {
      resultEl!.textContent = `调用失败: ${e}`;
      resultEl!.className = "status err";
    }
  });

document
  .querySelector<HTMLButtonElement>("#close-btn")
  ?.addEventListener("click", async () => {
    try {
      // === 步骤 7: 关闭窗口 ————————————————————————————————————
      // TODO: 同上，调用 close_ops_window
      // 提示: await invoke("close_ops_window");
      // 占位：完成填空后替换为真实调用
      resultEl!.textContent = "已关闭 ops 窗口";
      resultEl!.className = "status ok";
    } catch (e) {
      resultEl!.textContent = `调用失败: ${e}`;
      resultEl!.className = "status err";
    }
  });