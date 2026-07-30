// ============================================================
// 练习 006: 项目结构详解 (练习版)
//
// 目标: 调用 get_structure_overview 命令，展示目录树
// ============================================================
import { invoke } from "@tauri-apps/api/core";

const app = document.getElementById("app")!;

async function main() {
  // TODO: 取消注释下面的代码以调用后端命令
  /*
  const structure = await invoke<string>("get_structure_overview");
  app.innerHTML = `
    <div class="container">
      <div class="card">
        <h1>📁 项目结构</h1>
        <pre>${structure}</pre>
        <p class="hint">使用 std::fs::read_dir 遍历目录</p>
      </div>
    </div>
  `;
  */

  // 临时占位显示
  app.innerHTML = `
    <div class="container">
      <div class="card">
        <h1>📁 项目结构</h1>
        <p class="hint">请完成 lib.rs 中的 TODO，然后取消注释 main.ts 中的代码</p>
      </div>
    </div>
  `;
}

main();
