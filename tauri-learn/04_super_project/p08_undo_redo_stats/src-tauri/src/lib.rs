// ============================================================
// 超级项目 P08: 撤销/重做与字数统计
// 目标: 由练习 Agent 按规划文档编写
// 状态: 项目骨架（由 scripts/ 初始化脚本生成）
// ============================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}