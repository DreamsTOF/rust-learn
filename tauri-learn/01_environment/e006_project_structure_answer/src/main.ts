// ============================================================
// 练习 006: 项目结构详解 (答案版)
//
// 目标: 调用 get_structure_overview 命令，展示目录树
// ============================================================
import { invoke } from "@tauri-apps/api/core";

const app = document.getElementById("app")!;

async function main() {
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
}

main();
