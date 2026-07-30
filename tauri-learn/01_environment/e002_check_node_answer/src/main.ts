// ============================================================
// 练习 002: Node.js 环境检查 (答案)
//
// 目标: 调用后端命令检查 Node.js 版本并展示
// ============================================================
import { invoke } from "@tauri-apps/api/core";

const app = document.getElementById("app")!;

async function main() {
  const result = await invoke<string>("check_node_version");
  app.innerHTML = `
    <div class="container">
      <h1>🟢 Node.js 环境检查</h1>
      <div class="card">
        <p class="result">${result}</p>
      </div>
      <p class="hint">如果看到 Node.js 版本号，说明环境配置正确</p>
    </div>
  `;
}

main();
