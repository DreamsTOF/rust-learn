// ============================================================
// 练习 009: 跨平台开发注意事项 (练习版)
//
// 目标: 调用 get_platform_info 命令，以卡片形式展示平台信息
// ============================================================
import { invoke } from "@tauri-apps/api/core";

const app = document.getElementById("app")!;

// TODO: 定义 PlatformInfo 接口（取消注释）
/*
interface PlatformInfo {
  os: string;
  arch: string;
  family: string;
  is_windows: boolean;
  is_macos: boolean;
  is_linux: boolean;
}
*/

async function main() {
  // TODO: 取消注释下面的代码以调用后端命令
  /*
  const info: PlatformInfo = await invoke<PlatformInfo>("get_platform_info");
  app.innerHTML = `
    <div class="container">
      <div class="card">
        <h1>🌐 平台信息</h1>
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px; text-align: center;">
          <div style="background: #f0f2f5; border-radius: 8px; padding: 1rem;">
            <div style="font-size: 2rem; margin-bottom: 0.5rem;">💻</div>
            <div style="font-weight: 600;">操作系统</div>
            <div style="color: #0066cc;">${info.os}</div>
          </div>
          <div style="background: #f0f2f5; border-radius: 8px; padding: 1rem;">
            <div style="font-size: 2rem; margin-bottom: 0.5rem;">🔧</div>
            <div style="font-weight: 600;">架构</div>
            <div style="color: #0066cc;">${info.arch}</div>
          </div>
          <div style="background: #f0f2f5; border-radius: 8px; padding: 1rem;">
            <div style="font-size: 2rem; margin-bottom: 0.5rem;">🏠</div>
            <div style="font-weight: 600;">家族</div>
            <div style="color: #0066cc;">${info.family}</div>
          </div>
          <div style="background: #f0f2f5; border-radius: 8px; padding: 1rem;">
            <div style="font-size: 2rem; margin-bottom: 0.5rem;">${info.is_windows ? '✅' : '❌'}</div>
            <div style="font-weight: 600;">Windows</div>
            <div style="color: #0066cc;">${info.is_windows ? '是' : '否'}</div>
          </div>
          <div style="background: #f0f2f5; border-radius: 8px; padding: 1rem;">
            <div style="font-size: 2rem; margin-bottom: 0.5rem;">${info.is_macos ? '✅' : '❌'}</div>
            <div style="font-weight: 600;">macOS</div>
            <div style="color: #0066cc;">${info.is_macos ? '是' : '否'}</div>
          </div>
          <div style="background: #f0f2f5; border-radius: 8px; padding: 1rem;">
            <div style="font-size: 2rem; margin-bottom: 0.5rem;">${info.is_linux ? '✅' : '❌'}</div>
            <div style="font-weight: 600;">Linux</div>
            <div style="color: #0066cc;">${info.is_linux ? '是' : '否'}</div>
          </div>
        </div>
        <p class="hint" style="margin-top: 1rem;">使用 cfg!() 宏在编译期检测平台</p>
      </div>
    </div>
  `;
  */

  // 临时占位显示
  app.innerHTML = `
    <div class="container">
      <div class="card">
        <h1>🌐 平台信息</h1>
        <p class="hint">请完成 lib.rs 中的 TODO，然后取消注释 main.ts 中的代码</p>
      </div>
    </div>
  `;
}

main();
