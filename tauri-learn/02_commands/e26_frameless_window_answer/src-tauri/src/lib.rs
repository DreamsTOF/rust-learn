// ============================================================
// 练习 E26: 无边框窗口
// 目标: 去掉系统边框，用 data-tauri-drag-region 自定义标题栏
// 知识点: decorations: false / 拖拽区域 / 最小化与关闭按钮
// ============================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}