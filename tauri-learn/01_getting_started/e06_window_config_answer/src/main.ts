// ============================================================
// 练习 E06: 窗口配置
// 目标: 掌握窗口属性配置（title/尺寸/居中）与多窗口创建
// 知识点: getCurrentWindow().label / 按窗口渲染不同内容 / 命令触发
// ============================================================

import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

// 根据窗口 label 渲染不同内容：主窗口 vs 关于窗口
const label = getCurrentWindow().label;
const isAbout = label === "about";

const openBtn = document.querySelector<HTMLButtonElement>("#open-about");
const contentEl = document.querySelector<HTMLDivElement>("#content");

if (isAbout) {
  // 关于子窗口：展示本窗口的动态配置
  contentEl!.innerHTML = `
    <h2>关于本应用</h2>
    <p>本窗口由后端命令动态创建，配置如下：</p>
    <ul>
      <li><code>title</code>: 关于本应用</li>
      <li><code>inner_size</code>: 420 × 300</li>
      <li><code>center</code>: true</li>
      <li><code>resizable</code>: false</li>
    </ul>
    <p class="sub">主窗口的 title / width / height 则在 <code>tauri.conf.json</code> 中配置。</p>
  `;
} else {
  // 主窗口：按钮触发创建子窗口
  openBtn!.addEventListener("click", () => {
    invoke("open_about_window").catch((e) => console.error(e));
  });
}