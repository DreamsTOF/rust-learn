// ============================================================
// 练习 003: WebView2 环境检查 (答案)
//
// 目标: 调用后端命令检测当前系统 WebView 状态并展示
// ============================================================
import { invoke } from "@tauri-apps/api/core";

const app = document.getElementById("app")!;

async function main() {
  const result = await invoke<string>("check_webview_status");
  app.innerHTML = `
    <div class="container">
      <h1>🌐 WebView 环境检查</h1>
      <div class="card">
        <p class="result">${result}</p>
      </div>
      <p class="hint">不同平台的 WebView 实现不同，但 Tauri 均已支持</p>
    </div>
  `;
}

main();
