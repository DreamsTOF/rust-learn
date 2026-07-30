// ============================================================
// 练习 008: 构建与产物分析 (练习版)
//
// 目标: 调用 get_build_config 命令，以表格显示构建配置
// ============================================================
import { invoke } from "@tauri-apps/api/core";

const app = document.getElementById("app")!;

// TODO: 定义 BuildConfig 接口（取消注释）
/*
interface BuildConfig {
  profile: string;
  target_os: string;
  target_arch: string;
  rust_version: string;
  cargo_pkg_name: string;
  cargo_pkg_version: string;
}
*/

async function main() {
  // TODO: 取消注释下面的代码以调用后端命令
  /*
  const config: BuildConfig = await invoke<BuildConfig>("get_build_config");
  app.innerHTML = `
    <div class="container">
      <div class="card">
        <h1>🔧 构建配置</h1>
        <table style="width:100%; border-collapse: collapse; text-align: left;">
          <tr><td style="padding: 8px; border-bottom: 1px solid #eee; font-weight: 600;">配置项</td><td style="padding: 8px; border-bottom: 1px solid #eee; font-weight: 600;">值</td></tr>
          <tr><td style="padding: 8px; border-bottom: 1px solid #eee;">构建模式</td><td style="padding: 8px; border-bottom: 1px solid #eee;">${config.profile}</td></tr>
          <tr><td style="padding: 8px; border-bottom: 1px solid #eee;">目标系统</td><td style="padding: 8px; border-bottom: 1px solid #eee;">${config.target_os}</td></tr>
          <tr><td style="padding: 8px; border-bottom: 1px solid #eee;">目标架构</td><td style="padding: 8px; border-bottom: 1px solid #eee;">${config.target_arch}</td></tr>
          <tr><td style="padding: 8px; border-bottom: 1px solid #eee;">Rust 版本</td><td style="padding: 8px; border-bottom: 1px solid #eee;">${config.rust_version}</td></tr>
          <tr><td style="padding: 8px; border-bottom: 1px solid #eee;">包名</td><td style="padding: 8px; border-bottom: 1px solid #eee;">${config.cargo_pkg_name}</td></tr>
          <tr><td style="padding: 8px;">版本</td><td style="padding: 8px;">${config.cargo_pkg_version}</td></tr>
        </table>
        <p class="hint" style="margin-top: 1rem;">使用 cfg!() 宏和 env!() 宏在编译期收集信息</p>
      </div>
    </div>
  `;
  */

  // 临时占位显示
  app.innerHTML = `
    <div class="container">
      <div class="card">
        <h1>🔧 构建配置</h1>
        <p class="hint">请完成 lib.rs 中的 TODO，然后取消注释 main.ts 中的代码</p>
      </div>
    </div>
  `;
}

main();
