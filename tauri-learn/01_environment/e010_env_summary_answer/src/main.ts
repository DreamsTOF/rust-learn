// ============================================================
// 练习 010: 环境诊断工具 (答案版)
//
// 目标: 调用 get_env_summary 命令，以仪表盘展示诊断信息
// ============================================================
import { invoke } from "@tauri-apps/api/core";

const app = document.getElementById("app")!;

interface EnvSummary {
  rust_version: string;
  node_version: string;
  tauri_cli_version: string;
  platform: string;
  arch: string;
  env_mode: string;
  app_name: string;
  app_version: string;
}

async function main() {
  const summary: EnvSummary = await invoke<EnvSummary>("get_env_summary");
  app.innerHTML = `
    <div class="container">
      <div class="card">
        <h1>📊 环境诊断报告</h1>
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px; text-align: center;">
          <div style="background: #e8f4fd; border-radius: 8px; padding: 1rem;">
            <div style="font-size: 1.5rem; margin-bottom: 0.5rem;">🦀</div>
            <div style="font-weight: 600;">Rust 版本</div>
            <div style="color: #0066cc; word-break: break-word;">${summary.rust_version}</div>
          </div>
          <div style="background: #e8f4fd; border-radius: 8px; padding: 1rem;">
            <div style="font-size: 1.5rem; margin-bottom: 0.5rem;">🟢</div>
            <div style="font-weight: 600;">Node.js 版本</div>
            <div style="color: #0066cc; word-break: break-word;">${summary.node_version}</div>
          </div>
          <div style="background: #e8f4fd; border-radius: 8px; padding: 1rem;">
            <div style="font-size: 1.5rem; margin-bottom: 0.5rem;">📦</div>
            <div style="font-weight: 600;">Tauri CLI 版本</div>
            <div style="color: #0066cc; word-break: break-word;">${summary.tauri_cli_version}</div>
          </div>
          <div style="background: #e8f4fd; border-radius: 8px; padding: 1rem;">
            <div style="font-size: 1.5rem; margin-bottom: 0.5rem;">💻</div>
            <div style="font-weight: 600;">平台</div>
            <div style="color: #0066cc;">${summary.platform}</div>
          </div>
          <div style="background: #e8f4fd; border-radius: 8px; padding: 1rem;">
            <div style="font-size: 1.5rem; margin-bottom: 0.5rem;">🔧</div>
            <div style="font-weight: 600;">架构</div>
            <div style="color: #0066cc;">${summary.arch}</div>
          </div>
          <div style="background: #e8f4fd; border-radius: 8px; padding: 1rem;">
            <div style="font-size: 1.5rem; margin-bottom: 0.5rem;">⚙️</div>
            <div style="font-weight: 600;">环境模式</div>
            <div style="color: #0066cc;">${summary.env_mode}</div>
          </div>
          <div style="background: #e8f4fd; border-radius: 8px; padding: 1rem;">
            <div style="font-size: 1.5rem; margin-bottom: 0.5rem;">📄</div>
            <div style="font-weight: 600;">应用名称</div>
            <div style="color: #0066cc; word-break: break-word;">${summary.app_name}</div>
          </div>
          <div style="background: #e8f4fd; border-radius: 8px; padding: 1rem;">
            <div style="font-size: 1.5rem; margin-bottom: 0.5rem;">🏷️</div>
            <div style="font-weight: 600;">应用版本</div>
            <div style="color: #0066cc;">${summary.app_version}</div>
          </div>
        </div>
        <p class="hint" style="margin-top: 1rem;">综合 env!()、cfg!()、std::env::consts、std::process::Command</p>
      </div>
    </div>
  `;
}

main();
