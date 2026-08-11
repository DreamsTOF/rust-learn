// ============================================================
// 练习 E18: 路径 API
// 目标: 调用 list_paths 命令并渲染系统目录表格
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

const loadBtn = document.querySelector<HTMLButtonElement>("#load-btn");
const pathBody = document.querySelector<HTMLTableSectionElement>("#path-body");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

loadBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 4: 调用 list_paths ————————————————————————————————————
    // TODO: 定义接口并调用命令：
    //   interface PathItem { name: string; path: string; }
    //   const items = await invoke<PathItem[]>("list_paths");
    // 占位：完成填空后替换为真实调用结果（渲染逻辑保持不变）
    const items: Array<{ name: string; path: string }> = [];
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