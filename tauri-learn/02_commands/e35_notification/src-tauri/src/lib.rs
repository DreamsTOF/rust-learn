// ============================================================
// 练习 E35: 通知
// 目标: 使用 tauri-plugin-notification 请求权限、发送系统通知并监听点击
// 知识点: 插件注册 / notification:default 权限 / 权限检查与请求 / 通知点击事件
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 注册通知插件（Windows 上通知显示在系统通知中心）
        .plugin(tauri_plugin_notification::init())
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}