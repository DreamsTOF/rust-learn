// ============================================================
// 练习 004: 安装 Tauri CLI (答案)
//
// 目标: 调用后端命令检查 Tauri CLI 是否已安装并展示结果
// ============================================================
import { invoke } from "@tauri-apps/api/core";

const app = document.getElementById("app")!;

async function main() {
  const result = await invoke<string>("check_tauri_cli");
  app.innerHTML = `
    <div class="container">
      <h1>📦 Tauri CLI 检查</h1>
      <div class="card">
        <p class="result">${result}</p>
      </div>
      <p class="hint">Tauri CLI 是开发 Tauri 应用的重要工具</p>
    </div>
  `;
}

main();
