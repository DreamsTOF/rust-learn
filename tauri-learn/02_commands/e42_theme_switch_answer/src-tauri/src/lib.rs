// ============================================================
// 练习 E42: 主题切换
// 目标: 用 CSS 变量实现浅色/深色两套主题，支持跟随系统与手动切换
// 知识点: CSS 变量 / prefers-color-scheme / matchMedia / localStorage
// ============================================================

// 本练习无 Rust 命令：纯前端主题切换。
// 主题变量定义见 src/styles.css，切换逻辑见 src/main.ts。

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}