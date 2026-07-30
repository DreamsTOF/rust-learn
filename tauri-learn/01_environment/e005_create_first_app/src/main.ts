// ============================================================
// 练习 005: 创建第一个 Tauri 应用
//
// 目标: 调用后端命令获取应用元数据并展示
// 难度: ⭐⭐⭐
// ============================================================
import { invoke } from "@tauri-apps/api/core";

const app = document.getElementById("app")!;

// TODO: 定义 AppMetadata 接口（与 Rust 端对应）
// interface AppMetadata {
//   name: string;
//   version: string;
//   tauri_version: string;
//   os: string;
// }

async function main() {
  // TODO: 取消注释下面的代码
  /*
  const metadata = await invoke<AppMetadata>("get_app_metadata");
  app.innerHTML = `
    <div class="container">
      <h1>🚀 应用元数据</h1>
      <div class="card">
        <p><strong>应用名称:</strong> <span class="result">${metadata.name}</span></p>
        <p><strong>版本号:</strong> <span class="result">${metadata.version}</span></p>
        <p><strong>Tauri 版本:</strong> <span class="result">${metadata.tauri_version}</span></p>
        <p><strong>操作系统:</strong> <span class="result">${metadata.os}</span></p>
      </div>
      <p class="hint">这是你的第一个 Tauri 应用，恭喜！🎉</p>
    </div>
  `;
  */
}

main();
