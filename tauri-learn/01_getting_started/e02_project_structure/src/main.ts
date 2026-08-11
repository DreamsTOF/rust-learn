// ============================================================
// 练习 E02: 项目结构
// 目标: 理解 src/ 与 src-tauri/ 的分工、lib.rs 与 main.rs 的职责
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

const layoutEl = document.querySelector<HTMLPreElement>("#layout");

async function render() {
  // === 步骤 1: 调用后端命令 ————————————————————————————————
  // TODO: 改为调用真实命令：
  //   const lines = await invoke<string[]>("project_layout");
  // 当前为空数组占位（保持可编译），完成填空后页面将显示结构树
  const lines: string[] = [];

  // === 步骤 2: 渲染结构树 ——————————————————————————————————
  // TODO: 补全渲染（把 lines 逐行显示到 #layout）
  // 提示: layoutEl!.textContent = lines.join("\n");
  layoutEl!.textContent = lines.join("\n");
}

render().catch((e) => {
  layoutEl!.textContent = `调用失败: ${e}`;
});