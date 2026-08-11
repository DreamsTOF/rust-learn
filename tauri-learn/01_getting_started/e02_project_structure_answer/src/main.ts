// ============================================================
// 练习 E02: 项目结构
// 目标: 理解 src/ 与 src-tauri/ 的分工、lib.rs 与 main.rs 的职责
// 知识点: invoke() / 数组返回类型渲染
// ============================================================

import { invoke } from "@tauri-apps/api/core";

const layoutEl = document.querySelector<HTMLPreElement>("#layout");

async function render() {
  // 后端返回结构说明行数组，前端按行渲染
  const lines = await invoke<string[]>("project_layout");
  layoutEl!.textContent = lines.join("\n");
}

render().catch((e) => {
  layoutEl!.textContent = `调用失败: ${e}`;
});