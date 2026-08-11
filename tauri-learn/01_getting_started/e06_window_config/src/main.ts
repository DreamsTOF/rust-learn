// ============================================================
// 练习 E06: 窗口配置
// 目标: 掌握窗口属性配置（title/尺寸/居中）与多窗口创建
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

// 根据窗口 label 渲染不同内容：主窗口 vs 关于窗口
const label = getCurrentWindow().label;
const isAbout = label === "about";

const openBtn = document.querySelector<HTMLButtonElement>("#open-about");
const contentEl = document.querySelector<HTMLDivElement>("#content");

if (isAbout) {
  // === 步骤 1: 关于窗口内容 ————————————————————————————————
  // TODO: 补充关于内容，列出本窗口的动态配置：
  //   <code>title</code>: 关于本应用
  //   <code>inner_size</code>: 420 × 300
  //   <code>center</code>: true / <code>resizable</code>: false
  // 提示: contentEl!.innerHTML = `...`
  contentEl!.innerHTML = `
    <h2>关于本应用</h2>
    <p>本窗口由后端命令动态创建。</p>
  `;
} else {
  // === 步骤 2: 主窗口按钮事件 ——————————————————————————————
  // TODO: 点击按钮时调用 open_about_window 命令
  // 提示: invoke("open_about_window").catch((e) => console.error(e))
  openBtn!.addEventListener("click", () => {
    // TODO: 在这里调用命令
  });
}