// ============================================================
// 练习 E40: Vite 与 HMR
// 目标: 理解 Tauri 前端开发时 Vite 的集成方式与热更新机制
// 知识点: vite.config.ts / devUrl 与端口一致 / strictPort / TAURI_DEV_HOST / HMR / watch.ignored
// ============================================================

// 本练习无 Rust 命令：纯 vite 配置 + 前端信息页。
// 相关配置见项目根目录 vite.config.ts 与 src-tauri/tauri.conf.json 的 build.devUrl。

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}