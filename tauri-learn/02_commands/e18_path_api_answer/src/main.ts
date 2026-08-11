// ============================================================
// 练习 E18: 路径 API
// 目标: 调用 list_paths 命令并渲染系统目录表格
// 知识点: invoke<T[]> 泛型 / 表格渲染
// ============================================================

import { invoke } from "@tauri-apps/api/core";

// 与 Rust 端 PathItem 对应
interface PathItem {
  name: string;
  path: string;
}

const loadBtn = document.querySelector<HTMLButtonElement>("#load-btn");
const pathBody = document.querySelector<HTMLTableSectionElement>("#path-body");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

loadBtn!.addEventListener("click", async () => {
  try {
    const items = await invoke<PathItem[]>("list_paths");
    pathBody!.innerHTML = items
      .map((it) => `<tr><th>${it.name}</th><td><code>${it.path}</code></td></tr>`)
      .join("");
    resultEl!.textContent = `共解析 ${items.length} 个目录`;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});