// ============================================================
// 练习 007: 开发服务器与热更新 (答案版)
//
// 目标: 调用 get_env_mode 命令，显示当前环境模式
// ============================================================
import { invoke } from "@tauri-apps/api/core";

const app = document.getElementById("app")!;

async function main() {
  const mode = await invoke<string>("get_env_mode");
  app.innerHTML = `
    <div class="container">
      <div class="card">
        <h1>🖥️ 环境模式</h1>
        <div class="result">${mode}</div>
        <p class="hint">使用条件编译在编译期确定当前模式</p>
      </div>
      <div class="card" style="text-align: left;">
        <p><strong>💡 说明：</strong></p>
        <p>Rust 的 <code>#[cfg(debug_assertions)]</code> 属性在编译期判断当前是否是 debug 构建。</p>
        <p>▸ <code>cargo build</code> / <code>cargo tauri dev</code> → debug</p>
        <p>▸ <code>cargo build --release</code> / <code>cargo tauri build</code> → release</p>
      </div>
    </div>
  `;
}

main();
