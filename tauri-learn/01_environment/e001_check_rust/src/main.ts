// ============================================================
// 练习 001: Rust 环境检查
//
// 目标: 调用后端命令检查 Rust 版本并展示
// 难度: ⭐
// ============================================================
import { invoke } from "@tauri-apps/api/core";

const app = document.getElementById("app")!;

async function main() {
  // TODO: 取消注释下面的代码
  /*
  const result = await invoke<string>("check_rust_version");
  app.innerHTML = `
    <div class="container">
      <h1>🔧 Rust 环境检查</h1>
      <div class="card">
        <p class="result">${result}</p>
      </div>
      <p class="hint">如果看到 Rust 版本号，说明环境配置正确</p>
    </div>
  `;
  */
}

main();
